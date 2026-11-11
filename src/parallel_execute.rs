//! Parallel row pipeline for the `f32` executor (ADR-0022).
//!
//! The calling thread drives the decoder. It copies every needed source row
//! into an owned, budgeted buffer and submits a *horizontal job*; workers
//! apply a deferred source conversion if there is one (the ICC transform,
//! ADR-0023), linearize the row and run the horizontal kernel into a new
//! horizontal row, shared read-only. When all horizontal rows of an output row exist,
//! the driver submits a *vertical job*; a worker runs the vertical kernel and
//! quantizes. The driver reorders finished output rows and hands them to the
//! sink in order. Every job runs exactly the kernels and operation order of
//! the sequential path, so the output is byte-identical.
//!
//! Jobs travel through a bounded queue (backpressure on the decoder); results
//! come back through an unbounded channel, so workers never block on the
//! driver. Errors, including a cancelling sink, drop the job queue, the
//! workers exit, and `std::thread::scope` joins them before returning.

use std::collections::BTreeMap;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use ferrixel_kernels::Kernels;

use crate::decoder::{DecodeReport, DecodeSource, DecodedRow};
use crate::error::Error;
use crate::fast_execute::{
    ConversionScratch, RowPlan, Tables, kernel_error, quantize_row, source_row_to_linear, tables,
};
use crate::memory::{ManagedBuffer, MemoryBudget, try_filled_vec, try_vec};

/// An owned copy of one decoded source row.
enum Samples {
    U8(ManagedBuffer<u8>),
    F32(ManagedBuffer<f32>),
}

type Row = Arc<ManagedBuffer<f32>>;

enum Job {
    Horizontal { rank: usize, samples: Samples },
    Vertical { row: usize, inputs: Vec<(Row, f32)> },
}

enum Done {
    Horizontal {
        rank: usize,
        row: Row,
    },
    Vertical {
        row: usize,
        bytes: ManagedBuffer<u8>,
    },
}

/// Run `plan` on `threads` workers plus the calling thread.
pub(crate) fn run<const C: usize>(
    plan: &RowPlan,
    source: &mut dyn DecodeSource,
    budget: &MemoryBudget,
    kernels: &Kernels,
    threads: usize,
    mut sink: impl FnMut(u32, &[u8]) -> Result<(), Error>,
) -> Result<(DecodeReport, Duration, Duration), Error> {
    let tables = tables();
    let (done_sender, done) = mpsc::channel::<Result<Done, Error>>();
    let (jobs, job_receiver) = mpsc::sync_channel::<Job>(threads.saturating_mul(2).max(2));
    let job_receiver = Mutex::new(job_receiver);
    thread::scope(|scope| {
        for _ in 0..threads {
            let done_sender = done_sender.clone();
            let job_receiver = &job_receiver;
            if crate::thread_spawn::spawn(scope, move || {
                worker::<C>(plan, budget, kernels, tables, job_receiver, &done_sender);
            })
            .is_err()
            {
                // Workers may already be waiting on the queue. Close it
                // before scope joins them, including on a partial startup.
                drop(jobs);
                return Err(Error::ThreadSpawnFailed);
            }
        }
        drop(done_sender);
        // `drive` owns the job sender; when it returns, for any reason, the
        // queue closes and the workers exit before the scope ends.
        drive::<C>(plan, source, budget, jobs, &done, &mut sink)
    })
}

fn worker<const C: usize>(
    plan: &RowPlan,
    budget: &MemoryBudget,
    kernels: &Kernels,
    tables: &Tables,
    jobs: &Mutex<Receiver<Job>>,
    done: &Sender<Result<Done, Error>>,
) {
    let scratch = budget
        .allocate_typed::<f32>(plan.linear_samples)
        .and_then(|linear| {
            Ok((
                linear,
                budget.allocate_typed::<f32>(plan.row_samples)?,
                ConversionScratch::new(plan, budget)?,
            ))
        });
    let (mut linear, mut accumulator, mut conversion) = match scratch {
        Ok(buffers) => buffers,
        Err(error) => {
            let _ = done.send(Err(error));
            // Keep taking jobs until the driver closes the queue, so that a
            // driver that submitted before it saw the error never blocks on
            // a full queue that no worker drains.
            while let Ok(receiver) = jobs.lock() {
                if receiver.recv().is_err() {
                    return;
                }
            }
            return;
        }
    };
    loop {
        let job = match jobs.lock() {
            Ok(receiver) => receiver.recv(),
            Err(_) => return,
        };
        let Ok(job) = job else { return };
        let result = match job {
            Job::Horizontal { rank, samples } => horizontal::<C>(
                plan,
                budget,
                kernels,
                &samples,
                &mut conversion,
                linear.as_mut_slice(),
            )
            .map(|row| Done::Horizontal { rank, row }),
            Job::Vertical { row, inputs } => vertical::<C>(
                plan,
                budget,
                kernels,
                tables,
                &inputs,
                accumulator.as_mut_slice(),
            )
            .map(|bytes| Done::Vertical { row, bytes }),
        };
        if done.send(result).is_err() {
            return;
        }
    }
}

fn horizontal<const C: usize>(
    plan: &RowPlan,
    budget: &MemoryBudget,
    kernels: &Kernels,
    samples: &Samples,
    conversion: &mut ConversionScratch,
    linear: &mut [f32],
) -> Result<Row, Error> {
    let row = match samples {
        Samples::U8(buffer) => DecodedRow::U8(buffer.as_slice()),
        Samples::F32(buffer) => DecodedRow::F32(buffer.as_slice()),
    };
    source_row_to_linear::<C>(plan, row, conversion, linear)?;
    let mut out = budget.allocate_typed::<f32>(plan.row_samples)?;
    kernels
        .horizontal_rgba(linear, &plan.column_taps, out.as_mut_slice())
        .map_err(kernel_error)?;
    Ok(Arc::new(out))
}

fn vertical<const C: usize>(
    plan: &RowPlan,
    budget: &MemoryBudget,
    kernels: &Kernels,
    tables: &Tables,
    inputs: &[(Row, f32)],
    accumulator: &mut [f32],
) -> Result<ManagedBuffer<u8>, Error> {
    accumulator.fill(0.0);
    for (row, weight) in inputs {
        kernels
            .axpy(accumulator, row.as_slice(), *weight)
            .map_err(kernel_error)?;
    }
    let mut bytes = budget.allocate_typed::<u8>(plan.row_samples)?;
    quantize_row::<C>(accumulator, bytes.as_mut_slice(), tables)?;
    Ok(bytes)
}

/// The driver's view of the pipeline.
struct Driver<'p> {
    plan: &'p RowPlan,
    jobs: SyncSender<Job>,
    /// Horizontal rows by rank, `None` until they arrive or after release.
    rows: Vec<Option<Row>>,
    /// Next output row to submit and to emit.
    next_vertical: usize,
    next_emit: usize,
    /// Ranks below this are released.
    released: usize,
    finished: BTreeMap<usize, ManagedBuffer<u8>>,
    /// Jobs submitted and not yet answered.
    outstanding: usize,
}

impl Driver<'_> {
    fn submit(&mut self, job: Job) -> Result<(), Error> {
        self.jobs
            .send(job)
            .map_err(|_| Error::IncompleteDecoderOutput)?;
        self.outstanding += 1;
        Ok(())
    }

    fn absorb(&mut self, result: Result<Done, Error>) -> Result<(), Error> {
        // An error ends the run, and it may not answer a job: a worker that
        // cannot allocate its scratch reports before it takes any job. Only
        // a finished job is counted off.
        let done = result?;
        self.outstanding -= 1;
        match done {
            Done::Horizontal { rank, row } => self.rows[rank] = Some(row),
            Done::Vertical { row, bytes } => {
                self.finished.insert(row, bytes);
            }
        }
        Ok(())
    }

    /// Submit every output row whose inputs exist (in order), release rows
    /// no future output row needs, and emit finished rows in order.
    fn advance(
        &mut self,
        sink: &mut impl FnMut(u32, &[u8]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let plan = self.plan;
        while self.next_vertical < plan.output_rows {
            let (ranks, weights) = plan.vertical_taps(self.next_vertical);
            if ranks.iter().any(|&rank| self.rows[rank].is_none()) {
                break;
            }
            let mut inputs = try_vec(ranks.len())?;
            for (&rank, &weight) in ranks.iter().zip(weights) {
                let row = self.rows[rank]
                    .clone()
                    .ok_or(Error::IncompleteDecoderOutput)?;
                inputs.push((row, weight));
            }
            self.submit(Job::Vertical {
                row: self.next_vertical,
                inputs,
            })?;
            self.next_vertical += 1;
            let keep_from = plan.future_min[self.next_vertical];
            while self.released < plan.needed.len() && plan.needed[self.released] < keep_from {
                self.rows[self.released] = None;
                self.released += 1;
            }
        }
        while let Some(bytes) = self.finished.remove(&self.next_emit) {
            sink(plan.output_y(self.next_emit)?, bytes.as_slice())?;
            self.next_emit += 1;
        }
        Ok(())
    }

    fn drain_ready(
        &mut self,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, &[u8]) -> Result<(), Error>,
    ) -> Result<(), Error> {
        while let Ok(result) = done.try_recv() {
            self.absorb(result)?;
        }
        self.advance(sink)
    }
}

fn drive<const C: usize>(
    plan: &RowPlan,
    source: &mut dyn DecodeSource,
    budget: &MemoryBudget,
    jobs: SyncSender<Job>,
    done: &Receiver<Result<Done, Error>>,
    sink: &mut impl FnMut(u32, &[u8]) -> Result<(), Error>,
) -> Result<(DecodeReport, Duration, Duration), Error> {
    let mut driver = Driver {
        plan,
        jobs,
        rows: try_filled_vec(plan.needed.len(), None)?,
        next_vertical: 0,
        next_emit: 0,
        released: 0,
        finished: BTreeMap::new(),
        outstanding: 0,
    };
    let mut decoded_rows = 0_u32;
    let mut expected_y = plan.source_y;
    let mut transform_time = Duration::ZERO;
    let started = Instant::now();
    let report = source.read_region(plan.source_region, budget, &mut |decoder_y, row| {
        let row_started = Instant::now();
        if decoder_y != expected_y
            || row.len() != plan.source_samples
            || row.sample_type() != plan.sample
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        decoded_rows += 1;
        expected_y = expected_y.checked_add(1).ok_or(Error::GeometryOverflow)?;
        let semantic_y = plan.semantic_y(decoder_y)?;
        if let Ok(rank) = plan.needed.binary_search(&semantic_y) {
            let samples = match row {
                DecodedRow::U8(values) => {
                    let mut copy = budget.allocate_typed::<u8>(values.len())?;
                    copy.as_mut_slice().copy_from_slice(values);
                    Samples::U8(copy)
                }
                DecodedRow::F32(values) => {
                    let mut copy = budget.allocate_typed::<f32>(values.len())?;
                    copy.as_mut_slice().copy_from_slice(values);
                    Samples::F32(copy)
                }
            };
            driver.submit(Job::Horizontal { rank, samples })?;
        }
        driver.drain_ready(done, sink)?;
        transform_time += row_started.elapsed();
        Ok(())
    })?;
    let drain_started = Instant::now();
    while driver.next_emit < plan.output_rows {
        if driver.outstanding == 0 {
            // Nothing in flight and nothing submittable: the decoder did not
            // deliver every row the plan needs.
            return Err(Error::IncompleteDecoderOutput);
        }
        let result = done.recv().map_err(|_| Error::IncompleteDecoderOutput)?;
        driver.absorb(result)?;
        driver.drain_ready(done, sink)?;
    }
    transform_time += drain_started.elapsed();
    let source_time = started.elapsed().saturating_sub(transform_time);
    if decoded_rows != plan.source_height || report.rows != u64::from(decoded_rows) {
        return Err(Error::IncompleteDecoderOutput);
    }
    Ok((report, source_time, transform_time))
}

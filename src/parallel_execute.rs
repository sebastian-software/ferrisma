//! Parallel row pipeline for the `f32` executor (ADR-0022).
//!
//! The calling thread drives the decoder. It copies needed source rows into
//! pooled, budgeted buffers, pairs each with a pre-allocated horizontal output
//! buffer, and submits batched *horizontal strips*; workers apply a deferred
//! source conversion if there is one (the ICC transform, ADR-0023), linearize
//! each row and run the horizontal kernel into the supplied buffer, shared
//! read-only. When a strip of output rows has all of its horizontal inputs,
//! the driver pre-allocates its output buffers and submits a *vertical strip*
//! (prioritized ahead of horizontal strips so horizontal rows release
//! promptly); a worker runs the vertical kernel and quantizes into the
//! supplied output buffers, returning any uniquely-owned horizontal buffers to
//! the driver's pool. The driver reorders finished strips, hands rows to the
//! sink strictly in order, and recycles the output buffers. Every row runs
//! exactly the kernels and operation order of the sequential path, so the
//! output is byte-identical.
//!
//! All per-row budget allocations happen on the driver thread (worker scratch
//! is allocated up front before spawning), so budget pressure can always
//! reclaim idle driver pools, drain completed strips, and flush ready partial
//! vertical strips before reporting `BudgetExceeded`. Jobs travel through a
//! single-mutex bounded deque with backpressure on the driver, and in-flight
//! vertical output rows are strictly capped (`next_vertical - next_emit <=
//! max_vertical_in_flight`, counting each incoming strip before submission) so
//! upscales cannot pile up unbounded `OutputBuffer`s in the completion
//! channel. Errors, including a cancelling sink, close the queue, drop
//! unstarted jobs, wake all workers, and `std::thread::scope` joins them
//! before returning.

use std::collections::{BTreeMap, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use ferrisma_kernels::Kernels;

use crate::decoder::{DecodeReport, DecodeSource, DecodedRow, SampleType};
use crate::error::Error;
use crate::fast_execute::{
    ConversionScratch, RowPlan, Tables, VERTICAL_STACK_TAPS, kernel_error, quantize_into,
    source_row_to_linear, tables,
};
use crate::memory::{ManagedBuffer, MemoryBudget, try_filled_vec, try_vec};
use crate::output_depth::{OutputBuffer, OutputRow};

/// Maximum rows per horizontal strip job (amortizes lock, channel, and reorder
/// overhead across workers while keeping working set L2-friendly and tail
/// latency low on heterogeneous P+E core CPUs).
const MAX_HORIZONTAL_BATCH: usize = 8;

/// Maximum rows per vertical strip job (shares the deduplicated horizontal row
/// window across adjacent output rows and amortizes completion handling).
const MAX_VERTICAL_BATCH: usize = 8;

/// Upper bound on extra horizontal rows retained while waiting to form a full
/// vertical strip (`v_batch` scales down automatically on steep downscales so
/// holding a strip never multiplies the horizontal ring footprint).
const VERTICAL_BATCH_SOURCE_SPAN: usize = 16;

/// An owned copy of one decoded source row, pooled across horizontal jobs.
enum Samples {
    U8(ManagedBuffer<u8>),
    F32(ManagedBuffer<f32>),
}

type Row = Arc<ManagedBuffer<f32>>;

/// Per-worker scratch buffers allocated up front on the calling thread before
/// workers start, so worker hot loops never race the driver on `MemoryBudget`.
struct WorkerScratch {
    linear: ManagedBuffer<f32>,
    accumulator: ManagedBuffer<f32>,
    conversion: ConversionScratch,
}

/// One needed source row submitted in a horizontal strip, carrying a
/// driver-supplied horizontal output buffer (recycled or newly budgeted).
struct HorizontalItem {
    rank: usize,
    samples: Samples,
    out: ManagedBuffer<f32>,
}

/// One needed source row whose horizontal output buffer could not fit while
/// `DecodeSource::read_region` still held its own temporary buffers (such as
/// `OrientedSource`'s stored-order region). Promoted once space opens or
/// immediately after `read_region` returns and drops those buffers.
struct DeferredHorizontal {
    rank: usize,
    samples: Samples,
}

/// One completed horizontal row returned to the driver with its now-free
/// source buffer ready for reuse.
struct HorizontalResult {
    rank: usize,
    row: Row,
    samples: Samples,
}

enum Job {
    HorizontalBatch {
        items: Vec<HorizontalItem>,
    },
    VerticalBatch {
        start_row: usize,
        base_rank: usize,
        rows: Vec<Option<Row>>,
        outputs: Vec<OutputBuffer>,
    },
}

enum Done {
    HorizontalBatch {
        results: Vec<HorizontalResult>,
    },
    VerticalBatch {
        start_row: usize,
        outputs: Vec<OutputBuffer>,
        recycled_horiz: Vec<ManagedBuffer<f32>>,
    },
}

enum TryPush {
    Sent,
    Full(Job),
    Closed,
}

struct JobQueueState {
    buf: VecDeque<Job>,
    waiters: usize,
    closed: bool,
}

/// Single-mutex bounded job deque shared by the driver and all workers.
/// Vertical strips are pushed to the front so completed horizontal rows and
/// output buffers recycle promptly instead of waiting behind queued decode work.
struct JobQueue {
    state: Mutex<JobQueueState>,
    not_empty: Condvar,
    cap: usize,
}

impl JobQueue {
    fn new(cap: usize) -> Self {
        Self {
            state: Mutex::new(JobQueueState {
                buf: VecDeque::with_capacity(cap),
                waiters: 0,
                closed: false,
            }),
            not_empty: Condvar::new(),
            cap,
        }
    }

    fn try_push(&self, job: Job, high_priority: bool) -> TryPush {
        let Ok(mut state) = self.state.lock() else {
            return TryPush::Closed;
        };
        if state.closed {
            return TryPush::Closed;
        }
        if state.buf.len() >= self.cap {
            return TryPush::Full(job);
        }
        if high_priority {
            state.buf.push_front(job);
        } else {
            state.buf.push_back(job);
        }
        let wake = state.waiters > 0;
        drop(state);
        if wake {
            self.not_empty.notify_one();
        }
        TryPush::Sent
    }

    fn pop(&self) -> Option<Job> {
        let Ok(mut state) = self.state.lock() else {
            return None;
        };
        loop {
            if let Some(job) = state.buf.pop_front() {
                return Some(job);
            }
            if state.closed {
                return None;
            }
            state.waiters += 1;
            let Ok(mut next) = self.not_empty.wait(state) else {
                return None;
            };
            next.waiters -= 1;
            state = next;
        }
    }

    fn close(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.closed = true;
            state.buf.clear();
        }
        self.not_empty.notify_all();
    }
}

/// Closes the job queue on every exit path (normal return, error, or unwind)
/// so scoped workers always wake and join promptly.
struct QueueCloser<'q>(&'q JobQueue);

impl Drop for QueueCloser<'_> {
    fn drop(&mut self) {
        self.0.close();
    }
}

enum WorkerState {
    Pending {
        done_sender: Sender<Result<Done, Error>>,
        scratches: Vec<WorkerScratch>,
    },
    Started,
    Failed,
}

/// Run `plan` on `threads` workers plus the calling thread.
///
/// Resize workers are started lazily on the first needed row emitted by
/// `source`, so full-frame multithreaded decoders (such as JPEG XL) finish and
/// join their own decode pool before any resize worker exists. Peak live OS
/// threads therefore stay bounded by `threads` rather than `2 * threads`.
pub(crate) fn run<const C: usize>(
    plan: &RowPlan,
    source: &mut dyn DecodeSource,
    budget: &MemoryBudget,
    kernels: &Kernels,
    threads: usize,
    mut sink: impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
) -> Result<(DecodeReport, Duration, Duration), Error> {
    let tables = tables();
    let mut scratches = try_vec(threads)?;
    for _ in 0..threads {
        scratches.push(WorkerScratch {
            linear: budget.allocate_typed::<f32>(plan.linear_samples)?,
            accumulator: budget.allocate_typed::<f32>(plan.row_samples)?,
            conversion: ConversionScratch::new(plan, budget)?,
        });
    }
    let (done_sender, done) = mpsc::channel::<Result<Done, Error>>();
    let queue = JobQueue::new(threads.saturating_mul(2).max(2));
    thread::scope(|scope| {
        // Close the queue on any exit from this scope (including partial
        // thread startup failure, cancellation, or error) before joining.
        let _closer = QueueCloser(&queue);
        let mut workers = WorkerState::Pending {
            done_sender,
            scratches,
        };
        let mut ensure_workers = || -> Result<(), Error> {
            match std::mem::replace(&mut workers, WorkerState::Started) {
                WorkerState::Started => Ok(()),
                WorkerState::Failed => {
                    workers = WorkerState::Failed;
                    Err(Error::ThreadSpawnFailed)
                }
                WorkerState::Pending {
                    done_sender,
                    scratches,
                } => {
                    for scratch in scratches {
                        let done_sender = done_sender.clone();
                        let queue = &queue;
                        if crate::thread_spawn::spawn(scope, move || {
                            worker::<C>(plan, kernels, tables, queue, &done_sender, scratch);
                        })
                        .is_err()
                        {
                            workers = WorkerState::Failed;
                            return Err(Error::ThreadSpawnFailed);
                        }
                    }
                    Ok(())
                }
            }
        };
        drive::<C>(
            plan,
            source,
            budget,
            &queue,
            &done,
            threads,
            &mut ensure_workers,
            &mut sink,
        )
    })
}

fn worker<const C: usize>(
    plan: &RowPlan,
    kernels: &Kernels,
    tables: &Tables,
    queue: &JobQueue,
    done: &Sender<Result<Done, Error>>,
    mut scratch: WorkerScratch,
) {
    while let Some(job) = queue.pop() {
        let result = match job {
            Job::HorizontalBatch { items } => horizontal_batch::<C>(
                plan,
                kernels,
                items,
                &mut scratch.conversion,
                scratch.linear.as_mut_slice(),
            ),
            Job::VerticalBatch {
                start_row,
                base_rank,
                rows,
                outputs,
            } => vertical_batch::<C>(
                plan,
                kernels,
                tables,
                start_row,
                base_rank,
                rows,
                outputs,
                scratch.accumulator.as_mut_slice(),
            ),
        };
        let is_err = result.is_err();
        if done.send(result).is_err() || is_err {
            return;
        }
    }
}

fn horizontal_batch<const C: usize>(
    plan: &RowPlan,
    kernels: &Kernels,
    items: Vec<HorizontalItem>,
    conversion: &mut ConversionScratch,
    linear: &mut [f32],
) -> Result<Done, Error> {
    let mut results = try_vec(items.len())?;
    for mut item in items {
        let row_view = match &item.samples {
            Samples::U8(buffer) => DecodedRow::U8(buffer.as_slice()),
            Samples::F32(buffer) => DecodedRow::F32(buffer.as_slice()),
        };
        source_row_to_linear::<C>(plan, row_view, conversion, kernels, linear)?;
        kernels
            .horizontal_rgba(linear, &plan.column_taps, item.out.as_mut_slice())
            .map_err(kernel_error)?;
        results.push(HorizontalResult {
            rank: item.rank,
            row: Arc::new(item.out),
            samples: item.samples,
        });
    }
    Ok(Done::HorizontalBatch { results })
}

fn batch_row_slice(rows: &[Option<Row>], base_rank: usize, rank: usize) -> Result<&[f32], Error> {
    rows.get(rank.wrapping_sub(base_rank))
        .and_then(Option::as_ref)
        .map(|row| row.as_slice())
        .ok_or(Error::IncompleteDecoderOutput)
}

fn vertical_rows_chunked(
    kernels: &Kernels,
    rows: &[Option<Row>],
    base_rank: usize,
    ranks: &[usize],
    weights: &[f32],
    accumulator: &mut [f32],
) -> Result<(), Error> {
    let mut stack_rows: [&[f32]; VERTICAL_STACK_TAPS] = [&[]; VERTICAL_STACK_TAPS];
    for (chunk_idx, (rank_chunk, weight_chunk)) in ranks
        .chunks(VERTICAL_STACK_TAPS)
        .zip(weights.chunks(VERTICAL_STACK_TAPS))
        .enumerate()
    {
        for (dst, &rank) in stack_rows.iter_mut().zip(rank_chunk) {
            *dst = batch_row_slice(rows, base_rank, rank)?;
        }
        let slice = &stack_rows[..rank_chunk.len()];
        if chunk_idx == 0 {
            kernels
                .vertical_rgba(slice, weight_chunk, accumulator)
                .map_err(kernel_error)?;
        } else {
            kernels
                .vertical_rgba_add(slice, weight_chunk, accumulator)
                .map_err(kernel_error)?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn vertical_batch<const C: usize>(
    plan: &RowPlan,
    kernels: &Kernels,
    tables: &Tables,
    start_row: usize,
    base_rank: usize,
    rows: Vec<Option<Row>>,
    mut outputs: Vec<OutputBuffer>,
    accumulator: &mut [f32],
) -> Result<Done, Error> {
    for (offset, bytes) in outputs.iter_mut().enumerate() {
        let row_index = start_row + offset;
        let (ranks, weights) = plan.vertical_taps(row_index);
        match ranks {
            [] => kernels
                .vertical_rgba(&[], &[], accumulator)
                .map_err(kernel_error)?,
            &[r0] => kernels
                .vertical_rgba(
                    &[batch_row_slice(&rows, base_rank, r0)?],
                    weights,
                    accumulator,
                )
                .map_err(kernel_error)?,
            &[r0, r1] => kernels
                .vertical_rgba(
                    &[
                        batch_row_slice(&rows, base_rank, r0)?,
                        batch_row_slice(&rows, base_rank, r1)?,
                    ],
                    weights,
                    accumulator,
                )
                .map_err(kernel_error)?,
            &[r0, r1, r2] => kernels
                .vertical_rgba(
                    &[
                        batch_row_slice(&rows, base_rank, r0)?,
                        batch_row_slice(&rows, base_rank, r1)?,
                        batch_row_slice(&rows, base_rank, r2)?,
                    ],
                    weights,
                    accumulator,
                )
                .map_err(kernel_error)?,
            &[r0, r1, r2, r3] => kernels
                .vertical_rgba(
                    &[
                        batch_row_slice(&rows, base_rank, r0)?,
                        batch_row_slice(&rows, base_rank, r1)?,
                        batch_row_slice(&rows, base_rank, r2)?,
                        batch_row_slice(&rows, base_rank, r3)?,
                    ],
                    weights,
                    accumulator,
                )
                .map_err(kernel_error)?,
            _ => vertical_rows_chunked(kernels, &rows, base_rank, ranks, weights, accumulator)?,
        }
        quantize_into::<C>(accumulator, bytes, plan.depth, tables, kernels)?;
    }

    // Reclaim any horizontal buffers whose last `Arc` reference lived in this
    // vertical batch so the driver can reuse them without re-allocating.
    let mut recycled_horiz = Vec::new();
    for slot in rows {
        if let Some(row) = slot
            && let Ok(buf) = Arc::try_unwrap(row)
        {
            recycled_horiz.push(buf);
        }
    }
    Ok(Done::VerticalBatch {
        start_row,
        outputs,
        recycled_horiz,
    })
}

/// Outcome of `Driver::try_alloc_with_reclaim`, separating the target buffer's
/// own `BudgetExceeded` from any error (including a sink's `BudgetExceeded`)
/// returned during reclamation.
enum AllocOutcome<T> {
    Allocated(T),
    BudgetExceeded { requested: usize, available: usize },
}

/// The driver's view of the pipeline, including zero-lock local buffer pools.
struct Driver<'p, 'q> {
    plan: &'p RowPlan,
    queue: &'q JobQueue,
    /// Horizontal rows by rank, `None` until they arrive or after release.
    rows: Vec<Option<Row>>,
    /// Next output row to submit and to emit.
    next_vertical: usize,
    next_emit: usize,
    /// Ranks below this are released.
    released: usize,
    /// Smallest rank `>= released` whose horizontal row is not yet present in
    /// `self.rows`; every rank in `released..completed_prefix` is ready.
    completed_prefix: usize,
    /// Completed vertical strips keyed by their `start_row`.
    finished: BTreeMap<usize, Vec<OutputBuffer>>,
    /// Jobs submitted and not yet answered.
    outstanding: usize,
    /// Horizontal strip jobs currently submitted and not yet answered.
    in_flight_h: usize,
    /// Horizontal rows currently inside submitted-but-unfinished horizontal strips.
    in_flight_h_rows: usize,
    /// Completed horizontal rows currently retained in `self.rows`.
    retained_h_rows: usize,
    /// Upper bound on `in_flight_h` so the driver does not sprint far ahead of
    /// workers and inflate peak working memory.
    max_h_in_flight: usize,
    /// Hard upper bound on `retained_h_rows + in_flight_h_rows + pending_h +
    /// deferred_h` past the next vertical row's required ranks, preventing
    /// unbounded horizontal row retention if an early strip stalls on one worker.
    max_h_rows: usize,
    /// Horizontal rows accumulating into the next `Job::HorizontalBatch`.
    pending_h: Vec<HorizontalItem>,
    /// Source rows waiting for horizontal output buffer space (allocated once
    /// earlier work or `DecodeSource::read_region` frees memory).
    deferred_h: VecDeque<DeferredHorizontal>,
    h_batch: usize,
    v_batch: usize,
    /// Hard upper bound on `next_vertical - next_emit` (issue #210).
    max_vertical_in_flight: usize,
    /// Byte size of one horizontal `f32` row (used for `BudgetExceeded` reports).
    horiz_row_bytes: usize,
    /// Dedicated single `OutputBuffer` reserved lazily on the first needed
    /// row callback (and restored whenever no output buffer is live, e.g.
    /// right after `read_region` drops decoder buffers before draining
    /// `deferred_h`) so horizontal pre-allocation cannot starve vertical
    /// emission under tight budgets. Never cleared by horizontal pool
    /// reclamation or by lookahead horizontal rows, except as a last resort
    /// for a row required by `next_vertical` when no work is in flight.
    reserved_output: Option<OutputBuffer>,
    /// Driver-local pools of already-budgeted buffers for steady-state reuse.
    source_pool: Vec<Samples>,
    horiz_pool: Vec<ManagedBuffer<f32>>,
    output_pool: Vec<OutputBuffer>,
    source_pool_cap: usize,
    horiz_pool_cap: usize,
    output_pool_cap: usize,
}

impl<'p, 'q> Driver<'p, 'q> {
    fn new(plan: &'p RowPlan, queue: &'q JobQueue, threads: usize) -> Result<Self, Error> {
        let h_batch =
            (plan.needed.len() / threads.saturating_mul(8).max(1)).clamp(1, MAX_HORIZONTAL_BATCH);
        let rows_per_output = plan.needed.len().div_ceil(plan.output_rows.max(1)).max(1);
        let v_batch_by_ratio =
            (VERTICAL_BATCH_SOURCE_SPAN / rows_per_output).clamp(1, MAX_VERTICAL_BATCH);
        let v_batch =
            (plan.output_rows / threads.saturating_mul(8).max(1)).clamp(1, v_batch_by_ratio);
        let max_h_in_flight = threads.saturating_add(2);
        let max_h_rows = plan
            .capacity
            .saturating_add(max_h_in_flight.saturating_mul(h_batch).saturating_mul(2));
        let max_vertical_in_flight = threads.saturating_add(2).saturating_mul(v_batch);
        let source_pool_cap = max_h_in_flight.saturating_mul(h_batch);
        let horiz_pool_cap = plan
            .capacity
            .saturating_add(max_h_in_flight.saturating_mul(h_batch))
            .max(h_batch.saturating_mul(2));
        let output_pool_cap = max_vertical_in_flight;
        let horiz_row_bytes = plan.row_samples.saturating_mul(std::mem::size_of::<f32>());

        Ok(Self {
            plan,
            queue,
            rows: try_filled_vec(plan.needed.len(), None)?,
            next_vertical: 0,
            next_emit: 0,
            released: 0,
            completed_prefix: 0,
            finished: BTreeMap::new(),
            outstanding: 0,
            in_flight_h: 0,
            in_flight_h_rows: 0,
            retained_h_rows: 0,
            max_h_in_flight,
            max_h_rows,
            pending_h: try_vec(h_batch)?,
            deferred_h: VecDeque::new(),
            h_batch,
            v_batch,
            max_vertical_in_flight,
            horiz_row_bytes,
            reserved_output: None,
            source_pool: Vec::new(),
            horiz_pool: Vec::new(),
            output_pool: Vec::new(),
            source_pool_cap,
            horiz_pool_cap,
            output_pool_cap,
        })
    }

    fn total_h_rows(&self) -> usize {
        self.retained_h_rows
            .saturating_add(self.in_flight_h_rows)
            .saturating_add(self.pending_h.len())
            .saturating_add(self.deferred_h.len())
    }

    /// O(1) check whether `rank` lies strictly past every vertical tap required
    /// by `self.next_vertical` (using `RowPlan::ready`, which stores the
    /// running maximum tap source `y` per output row).
    fn is_past_next_vertical(&self, rank: usize) -> bool {
        self.plan
            .ready
            .get(self.next_vertical)
            .is_some_and(|&ready_y| self.plan.needed[rank] > ready_y)
    }

    fn is_vertical_row_ready(&self, out_row: usize) -> bool {
        let (ranks, _) = self.plan.vertical_taps(out_row);
        if ranks.is_empty() {
            return true;
        }
        if self.completed_prefix == self.rows.len()
            || self.plan.needed[self.completed_prefix] > self.plan.ready[out_row]
        {
            return true;
        }
        if ranks.last().is_some_and(|&rank| self.rows[rank].is_none()) {
            return false;
        }
        let start = ranks.partition_point(|&rank| rank < self.completed_prefix);
        ranks[start..].iter().all(|&rank| self.rows[rank].is_some())
    }

    fn recycle_source(&mut self, samples: Samples) {
        if self.source_pool.len() < self.source_pool_cap {
            self.source_pool.push(samples);
        }
    }

    fn recycle_horiz(&mut self, buf: ManagedBuffer<f32>) {
        if self.horiz_pool.len() < self.horiz_pool_cap {
            self.horiz_pool.push(buf);
        }
    }

    fn recycle_output(&mut self, buf: OutputBuffer) {
        if self.reserved_output.is_none() && self.next_vertical < self.plan.output_rows {
            self.reserved_output = Some(buf);
        } else if self.output_pool.len() < self.output_pool_cap {
            self.output_pool.push(buf);
        }
    }

    fn clear_idle_pools(&mut self) -> bool {
        let had = !self.source_pool.is_empty()
            || !self.horiz_pool.is_empty()
            || !self.output_pool.is_empty();
        self.source_pool.clear();
        self.horiz_pool.clear();
        self.output_pool.clear();
        had
    }

    /// Allocate a horizontal-side buffer (`Samples` or horizontal `f32` row),
    /// reclaiming idle pools, draining completions, flushing ready partial
    /// vertical strips, and waiting on in-flight work before returning
    /// `AllocOutcome::BudgetExceeded`. Any error from workers, `advance`, or
    /// `sink` (including a sink's `BudgetExceeded`) propagates immediately as
    /// `Err(...)`.
    fn try_alloc_with_reclaim<T>(
        &mut self,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
        release_reserved_output: bool,
        mut try_pop_or_alloc: impl FnMut(&mut Self, &MemoryBudget) -> Result<T, Error>,
    ) -> Result<AllocOutcome<T>, Error> {
        loop {
            match try_pop_or_alloc(self, budget) {
                Ok(value) => return Ok(AllocOutcome::Allocated(value)),
                Err(Error::BudgetExceeded {
                    requested,
                    available,
                }) => {
                    if self.clear_idle_pools() {
                        continue;
                    }
                    let mut made_progress = false;
                    while let Ok(result) = done.try_recv() {
                        self.absorb(result)?;
                        made_progress = true;
                    }
                    let emitted_before = self.next_emit;
                    self.emit_finished(sink)?;
                    if self.next_emit > emitted_before {
                        made_progress = true;
                    }
                    if self.clear_idle_pools() {
                        made_progress = true;
                    }
                    let submitted_v_before = self.next_vertical;
                    self.advance(budget, done, sink, true)?;
                    if self.next_vertical > submitted_v_before || self.clear_idle_pools() {
                        made_progress = true;
                    }
                    if made_progress {
                        continue;
                    }
                    if !self.pending_h.is_empty() {
                        self.flush_horizontal(done, sink)?;
                        continue;
                    }
                    if self.outstanding > 0 {
                        let result = done.recv().map_err(|_| Error::IncompleteDecoderOutput)?;
                        self.absorb(result)?;
                        self.emit_finished(sink)?;
                        self.clear_idle_pools();
                        continue;
                    }
                    // Last resort (only when allowed — never for deferrable
                    // horizontal rows during `read_region`, where releasing
                    // `reserved_output` could let a horizontal row fit while
                    // starving vertical emission): no work is in flight, no
                    // idle pools remain, and no vertical row is ready yet.
                    if release_reserved_output && self.reserved_output.take().is_some() {
                        continue;
                    }
                    return Ok(AllocOutcome::BudgetExceeded {
                        requested,
                        available,
                    });
                }
                Err(err) => return Err(err),
            }
        }
    }

    fn copy_source_row(
        &mut self,
        row: DecodedRow<'_>,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<Samples, Error> {
        match self.try_alloc_with_reclaim(budget, done, sink, true, |driver, budget| {
            if let Some(mut pooled) = driver.source_pool.pop() {
                match (&mut pooled, row) {
                    (Samples::U8(copy), DecodedRow::U8(values)) => {
                        copy.as_mut_slice().copy_from_slice(values);
                        return Ok(pooled);
                    }
                    (Samples::F32(copy), DecodedRow::F32(values)) => {
                        copy.as_mut_slice().copy_from_slice(values);
                        return Ok(pooled);
                    }
                    _ => {}
                }
            }
            match row {
                DecodedRow::U8(values) => Ok(Samples::U8(budget.allocate_copied(values)?)),
                DecodedRow::F32(values) => Ok(Samples::F32(budget.allocate_copied(values)?)),
            }
        })? {
            AllocOutcome::Allocated(samples) => Ok(samples),
            AllocOutcome::BudgetExceeded {
                requested,
                available,
            } => Err(Error::BudgetExceeded {
                requested,
                available,
            }),
        }
    }

    /// Allocate a horizontal `f32` row during `read_region`, keeping
    /// `reserved_output` intact so vertical emission cannot be starved while
    /// decoder-held buffers are still live. Returns `Ok(None)` ONLY when the
    /// horizontal row allocation itself cannot fit after full reclamation; any
    /// sink or worker error (including a sink's `BudgetExceeded`) propagates
    /// immediately as `Err(...)`.
    fn try_alloc_horiz_row(
        &mut self,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<Option<ManagedBuffer<f32>>, Error> {
        let row_samples = self.plan.row_samples;
        match self.try_alloc_with_reclaim(budget, done, sink, false, |driver, budget| {
            if let Some(buf) = driver.horiz_pool.pop() {
                return Ok(buf);
            }
            budget.allocate_typed::<f32>(row_samples)
        })? {
            AllocOutcome::Allocated(buf) => Ok(Some(buf)),
            AllocOutcome::BudgetExceeded { .. } => Ok(None),
        }
    }

    fn ensure_reserved_output(&mut self, budget: &MemoryBudget) {
        if self.reserved_output.is_none()
            && self.output_pool.is_empty()
            && self.next_vertical == self.next_emit
            && self.next_vertical < self.plan.output_rows
            && let Ok(buf) = OutputBuffer::allocate(self.plan.depth, self.plan.row_samples, budget)
        {
            self.reserved_output = Some(buf);
        }
    }

    fn alloc_horiz_row(
        &mut self,
        release_reserved_output: bool,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<ManagedBuffer<f32>, Error> {
        let row_samples = self.plan.row_samples;
        match self.try_alloc_with_reclaim(
            budget,
            done,
            sink,
            release_reserved_output,
            |driver, budget| {
                if let Some(buf) = driver.horiz_pool.pop() {
                    return Ok(buf);
                }
                budget.allocate_typed::<f32>(row_samples)
            },
        )? {
            AllocOutcome::Allocated(buf) => Ok(buf),
            AllocOutcome::BudgetExceeded {
                requested,
                available,
            } => Err(Error::BudgetExceeded {
                requested,
                available,
            }),
        }
    }

    /// Collect between `1` and `target` output buffers for a vertical strip.
    /// Reclaims idle non-output pools and drains in-flight completions before
    /// ever failing with `BudgetExceeded`; under tight budgets, returns a
    /// smaller non-empty strip (`1..target`) so vertical work can proceed and
    /// release horizontal rows promptly.
    fn alloc_vertical_outputs(
        &mut self,
        target: usize,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<Vec<OutputBuffer>, Error> {
        let mut outputs = try_vec(target)?;
        while outputs.len() < target {
            if let Some(buf) = self.output_pool.pop() {
                outputs.push(buf);
                continue;
            }
            if outputs.is_empty()
                && let Some(buf) = self.reserved_output.take()
            {
                outputs.push(buf);
                continue;
            }
            match OutputBuffer::allocate(self.plan.depth, self.plan.row_samples, budget) {
                Ok(buf) => outputs.push(buf),
                Err(Error::BudgetExceeded {
                    requested,
                    available,
                }) => {
                    // Drop idle source/horizontal buffers first so output
                    // allocation never fails while another pool holds memory.
                    if !self.source_pool.is_empty() || !self.horiz_pool.is_empty() {
                        self.source_pool.clear();
                        self.horiz_pool.clear();
                        continue;
                    }
                    // If we already have at least one output buffer, submit
                    // this smaller vertical strip immediately so it can
                    // release horizontal rows.
                    if !outputs.is_empty() {
                        break;
                    }
                    // Absorb any ready completions and emit finished rows to
                    // recycle OutputBuffers before waiting.
                    let mut made_progress = false;
                    while let Ok(result) = done.try_recv() {
                        self.absorb(result)?;
                        made_progress = true;
                    }
                    let emitted_before = self.next_emit;
                    self.emit_finished(sink)?;
                    if self.next_emit > emitted_before {
                        made_progress = true;
                    }
                    if !self.source_pool.is_empty() || !self.horiz_pool.is_empty() {
                        self.source_pool.clear();
                        self.horiz_pool.clear();
                        made_progress = true;
                    }
                    if made_progress {
                        continue;
                    }
                    if self.outstanding > 0 {
                        let result = done.recv().map_err(|_| Error::IncompleteDecoderOutput)?;
                        self.absorb(result)?;
                        self.emit_finished(sink)?;
                        self.source_pool.clear();
                        self.horiz_pool.clear();
                        continue;
                    }
                    return Err(Error::BudgetExceeded {
                        requested,
                        available,
                    });
                }
                Err(err) => return Err(err),
            }
        }
        Ok(outputs)
    }

    fn absorb(&mut self, result: Result<Done, Error>) -> Result<(), Error> {
        let done = result?;
        self.outstanding -= 1;
        match done {
            Done::HorizontalBatch { results } => {
                self.in_flight_h -= 1;
                self.in_flight_h_rows -= results.len();
                for item in results {
                    if self.rows[item.rank].replace(item.row).is_none() {
                        self.retained_h_rows += 1;
                    }
                    self.recycle_source(item.samples);
                }
                while self.completed_prefix < self.rows.len()
                    && self.rows[self.completed_prefix].is_some()
                {
                    self.completed_prefix += 1;
                }
            }
            Done::VerticalBatch {
                start_row,
                outputs,
                recycled_horiz,
            } => {
                for buf in recycled_horiz {
                    self.recycle_horiz(buf);
                }
                self.finished.insert(start_row, outputs);
            }
        }
        Ok(())
    }

    fn emit_finished(
        &mut self,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        while let Some(outputs) = self.finished.remove(&self.next_emit) {
            for bytes in outputs {
                sink(self.plan.output_y(self.next_emit)?, bytes.row())?;
                self.next_emit += 1;
                self.recycle_output(bytes);
            }
        }
        Ok(())
    }

    /// Push `job` to the bounded queue. When full, block on `done` to absorb
    /// completions, recycle buffers, and emit finished rows rather than
    /// letting `done` accumulate unbounded results.
    fn submit_with_drain(
        &mut self,
        mut job: Job,
        high_priority: bool,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        loop {
            match self.queue.try_push(job, high_priority) {
                TryPush::Sent => {
                    self.outstanding += 1;
                    return Ok(());
                }
                TryPush::Full(returned) => {
                    job = returned;
                    let result = done.recv().map_err(|_| Error::IncompleteDecoderOutput)?;
                    self.absorb(result)?;
                    while let Ok(more) = done.try_recv() {
                        self.absorb(more)?;
                    }
                    self.emit_finished(sink)?;
                }
                TryPush::Closed => return Err(Error::IncompleteDecoderOutput),
            }
        }
    }

    fn flush_horizontal(
        &mut self,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if self.pending_h.is_empty() {
            return Ok(());
        }
        let batch_rows = self.pending_h.len();
        let items = std::mem::replace(&mut self.pending_h, try_vec(self.h_batch)?);
        self.submit_with_drain(Job::HorizontalBatch { items }, false, done, sink)?;
        self.in_flight_h += 1;
        self.in_flight_h_rows += batch_rows;
        Ok(())
    }

    /// Wait until both in-flight horizontal strips (`in_flight_h < max_h_in_flight`
    /// when starting a batch) and total horizontal rows including pending,
    /// deferred, and the incoming row (`total_h_rows + 1 <= max_h_rows` past
    /// `next_vertical`'s required ranks) stay within their caps, absorbing
    /// completions, submitting prioritized vertical strips, and recycling
    /// buffers so a stalled early strip cannot let the decoder advance
    /// unboundedly ahead of vertical work.
    fn pace_horizontal(
        &mut self,
        incoming_rank: usize,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        loop {
            let too_many_batches =
                self.pending_h.is_empty() && self.in_flight_h >= self.max_h_in_flight;
            let too_many_rows = self.total_h_rows().saturating_add(1) > self.max_h_rows
                && self.is_past_next_vertical(incoming_rank);
            if !too_many_batches && !too_many_rows {
                return Ok(());
            }
            if too_many_rows {
                if !self.pending_h.is_empty() {
                    self.flush_horizontal(done, sink)?;
                }
                let prev_released = self.released;
                self.advance(budget, done, sink, true)?;
                if self.released > prev_released {
                    continue;
                }
            }
            if self.outstanding == 0 {
                if too_many_rows {
                    // Nothing is in flight and pending_h is empty, so deferred
                    // horizontal rows cannot drain before `read_region` ends;
                    // refuse rather than silently queuing past `max_h_rows`.
                    return Err(Error::BudgetExceeded {
                        requested: self.horiz_row_bytes,
                        available: budget.available(),
                    });
                }
                return Ok(());
            }
            let result = done.recv().map_err(|_| Error::IncompleteDecoderOutput)?;
            self.absorb(result)?;
            self.advance(budget, done, sink, too_many_rows)?;
        }
    }

    /// Opportunistically allocate horizontal rows for previously deferred
    /// source rows during `read_region` once budget space opens up.
    fn try_promote_deferred(
        &mut self,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        while let Some(item) = self.deferred_h.pop_front() {
            self.pace_horizontal(item.rank, budget, done, sink)?;
            if let Some(out) = self.try_alloc_horiz_row(budget, done, sink)? {
                self.pending_h.push(HorizontalItem {
                    rank: item.rank,
                    samples: item.samples,
                    out,
                });
                if self.pending_h.len() >= self.h_batch {
                    self.flush_horizontal(done, sink)?;
                    self.drain_ready(budget, done, sink, false)?;
                }
            } else {
                self.deferred_h.push_front(item);
                break;
            }
        }
        Ok(())
    }

    /// Drain all deferred horizontal rows after `DecodeSource::read_region`
    /// returns and drops decoder-held buffers; `BudgetExceeded` is fatal here.
    fn drain_deferred_horizontal(
        &mut self,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.ensure_reserved_output(budget);
        while let Some(item) = self.deferred_h.pop_front() {
            self.pace_horizontal(item.rank, budget, done, sink)?;
            self.ensure_reserved_output(budget);
            // Protect `reserved_output` from lookahead horizontal rows past
            // `next_vertical`; only a row required to complete `next_vertical`
            // may release it as a last resort.
            let allow_release = !self.is_past_next_vertical(item.rank);
            let out = self.alloc_horiz_row(allow_release, budget, done, sink)?;
            self.pending_h.push(HorizontalItem {
                rank: item.rank,
                samples: item.samples,
                out,
            });
            if self.pending_h.len() >= self.h_batch {
                self.flush_horizontal(done, sink)?;
                self.drain_ready(budget, done, sink, false)?;
            }
        }
        Ok(())
    }

    fn push_needed_row(
        &mut self,
        rank: usize,
        row: DecodedRow<'_>,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.ensure_reserved_output(budget);
        if !self.deferred_h.is_empty() {
            self.try_promote_deferred(budget, done, sink)?;
        }
        self.pace_horizontal(rank, budget, done, sink)?;
        let samples = self.copy_source_row(row, budget, done, sink)?;
        if self.deferred_h.is_empty() {
            if let Some(out) = self.try_alloc_horiz_row(budget, done, sink)? {
                self.pending_h.push(HorizontalItem { rank, samples, out });
            } else {
                self.deferred_h
                    .push_back(DeferredHorizontal { rank, samples });
            }
        } else {
            self.deferred_h
                .push_back(DeferredHorizontal { rank, samples });
        }
        debug_assert!(!self.is_past_next_vertical(rank) || self.total_h_rows() <= self.max_h_rows);
        if self.pending_h.len() >= self.h_batch {
            self.flush_horizontal(done, sink)?;
            self.drain_ready(budget, done, sink, false)?;
        }
        Ok(())
    }

    /// Submit every ready strip of output rows (in order), release horizontal
    /// rows no future output row needs, and emit finished rows in order while
    /// strictly enforcing `next_vertical - next_emit <= max_vertical_in_flight`
    /// (issue #210).
    fn advance(
        &mut self,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
        flush_partial: bool,
    ) -> Result<(), Error> {
        let plan = self.plan;
        loop {
            while let Ok(result) = done.try_recv() {
                self.absorb(result)?;
            }
            self.emit_finished(sink)?;

            if self.next_vertical >= plan.output_rows {
                break;
            }

            let mut ready_count = 0_usize;
            let max_count = self.v_batch.min(plan.output_rows - self.next_vertical);
            while ready_count < max_count
                && self.is_vertical_row_ready(self.next_vertical + ready_count)
            {
                ready_count += 1;
            }
            if ready_count == 0 {
                break;
            }
            let reaches_end = self.next_vertical + ready_count == plan.output_rows;
            if !flush_partial && !reaches_end && ready_count < self.v_batch {
                break;
            }

            // Issue #210: count the incoming strip (`ready_count`) against
            // `max_vertical_in_flight` so outstanding vertical rows never
            // exceed the cap by even one row (during decode or after EOF).
            while (self.next_vertical - self.next_emit).saturating_add(ready_count)
                > self.max_vertical_in_flight
            {
                let result = done.recv().map_err(|_| Error::IncompleteDecoderOutput)?;
                self.absorb(result)?;
                while let Ok(more) = done.try_recv() {
                    self.absorb(more)?;
                }
                self.emit_finished(sink)?;
            }

            let outputs = self.alloc_vertical_outputs(ready_count, budget, done, sink)?;
            let batch_count = outputs.len();
            let start_row = self.next_vertical;
            let mut min_rank = usize::MAX;
            let mut max_rank = 0_usize;
            for offset in 0..batch_count {
                let (ranks, _) = plan.vertical_taps(start_row + offset);
                for &rank in ranks {
                    min_rank = min_rank.min(rank);
                    max_rank = max_rank.max(rank);
                }
            }
            let span = if min_rank <= max_rank {
                max_rank - min_rank + 1
            } else {
                0
            };
            let mut batch_rows: Vec<Option<Row>> = try_filled_vec(span, None)?;
            for offset in 0..batch_count {
                let (ranks, _) = plan.vertical_taps(start_row + offset);
                for &rank in ranks {
                    let slot = &mut batch_rows[rank - min_rank];
                    if slot.is_none() {
                        let row = self.rows[rank]
                            .clone()
                            .ok_or(Error::IncompleteDecoderOutput)?;
                        *slot = Some(row);
                    }
                }
            }

            self.next_vertical += batch_count;
            debug_assert!(self.next_vertical - self.next_emit <= self.max_vertical_in_flight);
            let keep_from = plan.future_min[self.next_vertical];
            while self.released < plan.needed.len() && plan.needed[self.released] < keep_from {
                if let Some(row) = self.rows[self.released].take() {
                    self.retained_h_rows -= 1;
                    if let Ok(buf) = Arc::try_unwrap(row) {
                        self.recycle_horiz(buf);
                    }
                }
                self.released += 1;
            }
            debug_assert!(self.completed_prefix >= self.released);

            self.submit_with_drain(
                Job::VerticalBatch {
                    start_row,
                    base_rank: if span == 0 { 0 } else { min_rank },
                    rows: batch_rows,
                    outputs,
                },
                true,
                done,
                sink,
            )?;
        }
        self.emit_finished(sink)
    }

    fn drain_ready(
        &mut self,
        budget: &MemoryBudget,
        done: &Receiver<Result<Done, Error>>,
        sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
        flush_partial: bool,
    ) -> Result<(), Error> {
        self.advance(budget, done, sink, flush_partial)
    }
}

/// Exact minimum managed-memory working set `Driver` needs after
/// `DecodeSource::read_region` begins delivering callbacks to guarantee forward
/// progress to completion (1 source row copy + `plan.capacity` horizontal `f32`
/// rows + 1 output row; worker scratch and plan reservations are already live).
fn min_downstream_working_bytes(plan: &RowPlan) -> usize {
    let sample_bytes = match plan.sample {
        SampleType::U8 => 1,
        SampleType::F32 => std::mem::size_of::<f32>(),
    };
    let source_row_bytes = plan.source_samples.saturating_mul(sample_bytes);
    let horiz_row_bytes = plan.row_samples.saturating_mul(std::mem::size_of::<f32>());
    let output_sample_bytes = if plan.depth > crate::output_depth::EIGHT {
        std::mem::size_of::<u16>()
    } else {
        1
    };
    let output_row_bytes = if plan.output_rows > 0 {
        plan.row_samples.saturating_mul(output_sample_bytes)
    } else {
        0
    };
    source_row_bytes
        .saturating_add(plan.capacity.max(1).saturating_mul(horiz_row_bytes))
        .saturating_add(output_row_bytes)
}

#[allow(clippy::too_many_arguments)] // Plan, source, budget, queue, completion channel, thread count, lazy spawner and sink.
fn drive<const C: usize>(
    plan: &RowPlan,
    source: &mut dyn DecodeSource,
    budget: &MemoryBudget,
    queue: &JobQueue,
    done: &Receiver<Result<Done, Error>>,
    threads: usize,
    ensure_workers: &mut impl FnMut() -> Result<(), Error>,
    sink: &mut impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
) -> Result<(DecodeReport, Duration, Duration), Error> {
    let mut driver = Driver::new(plan, queue, threads)?;
    let mut decoded_rows = 0_u32;
    let mut expected_y = plan.source_y;
    let mut needed_cursor = 0_usize;
    let mut transform_time = Duration::ZERO;
    let started = Instant::now();
    source.set_downstream_headroom(min_downstream_working_bytes(plan));
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
        while needed_cursor < plan.needed.len() && plan.needed[needed_cursor] < semantic_y {
            needed_cursor += 1;
        }
        if needed_cursor < plan.needed.len() && plan.needed[needed_cursor] == semantic_y {
            ensure_workers()?;
            let rank = needed_cursor;
            needed_cursor += 1;
            driver.push_needed_row(rank, row, budget, done, sink)?;
        }
        transform_time += row_started.elapsed();
        Ok(())
    });
    source.set_downstream_headroom(0);
    let report = report?;
    let drain_started = Instant::now();
    driver.drain_deferred_horizontal(budget, done, sink)?;
    driver.flush_horizontal(done, sink)?;
    driver.drain_ready(budget, done, sink, true)?;
    while driver.next_emit < plan.output_rows {
        if driver.outstanding == 0 {
            // Nothing in flight and nothing submittable: the decoder did not
            // deliver every row the plan needs.
            return Err(Error::IncompleteDecoderOutput);
        }
        let result = done.recv().map_err(|_| Error::IncompleteDecoderOutput)?;
        driver.absorb(result)?;
        driver.drain_ready(budget, done, sink, true)?;
    }
    debug_assert!(driver.deferred_h.is_empty());
    debug_assert_eq!(driver.in_flight_h_rows, 0);
    debug_assert_eq!(driver.retained_h_rows, 0);
    transform_time += drain_started.elapsed();
    let source_time = started.elapsed().saturating_sub(transform_time);
    if decoded_rows != plan.source_height || report.rows != u64::from(decoded_rows) {
        return Err(Error::IncompleteDecoderOutput);
    }
    Ok((report, source_time, transform_time))
}

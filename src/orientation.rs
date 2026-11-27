//! Display orientation of decoded samples, applied as an explicit,
//! engine-budgeted source-side materialization (ADR-0019).
//!
//! Decoders emit samples in *stored* order. EXIF orientation and AVIF
//! `irot`/`imir` describe how to display them. `OrientedSource` presents an
//! upright image to the graph: it maps each requested display rectangle back
//! to the stored rectangle, reads that region once into a managed buffer, and
//! emits upright rows. Crop coordinates are therefore display coordinates.

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::thread;

use crate::decoder::{DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::{ManagedBuffer, MemoryBudget};

/// One of the eight EXIF orientations. Display pixel `(x, y)` reads the
/// stored pixel obtained by optionally swapping the axes (`transpose`) and
/// then mirroring the stored x and/or y coordinate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Orientation {
    transpose: bool,
    flip_x: bool,
    flip_y: bool,
}

impl Orientation {
    pub(crate) const IDENTITY: Self = Self {
        transpose: false,
        flip_x: false,
        flip_y: false,
    };

    /// EXIF/TIFF `Orientation` values 1–8; anything else is not an orientation.
    pub(crate) fn from_exif(value: u16) -> Option<Self> {
        let (transpose, flip_x, flip_y) = match value {
            1 => (false, false, false),
            2 => (false, true, false),
            3 => (false, true, true),
            4 => (false, false, true),
            5 => (true, false, false),
            6 => (true, false, true),
            7 => (true, true, true),
            8 => (true, true, false),
            _ => return None,
        };
        Some(Self {
            transpose,
            flip_x,
            flip_y,
        })
    }

    /// HEIF `irot` (anticlockwise quarter turns) followed by `imir`, whose
    /// axis 0 exchanges top and bottom and axis 1 exchanges left and right
    /// (ISO/IEC 23008-12:2022 6.5.12, as documented by libavif).
    #[cfg_attr(
        not(feature = "avif-read"),
        allow(dead_code, reason = "constructed only by the AVIF adapter")
    )]
    pub(crate) fn from_avif(rotation: u8, mirror: Option<u8>) -> Option<Self> {
        let exif = match (rotation, mirror) {
            (0, None) => 1,
            (1, None) => 8,
            (2, None) => 3,
            (3, None) => 6,
            (0, Some(0)) | (2, Some(1)) => 4,
            (0, Some(1)) | (2, Some(0)) => 2,
            (1, Some(0)) | (3, Some(1)) => 5,
            (1, Some(1)) | (3, Some(0)) => 7,
            _ => return None,
        };
        Self::from_exif(exif)
    }

    #[cfg_attr(
        not(feature = "avif-write"),
        allow(dead_code, reason = "used by the facade")
    )]
    pub(crate) fn is_identity(self) -> bool {
        self == Self::IDENTITY
    }

    /// True for EXIF orientation 2 (`flip_x` only), which can stream row by
    /// row in O(1 row) memory without buffering the stored region.
    fn is_horizontal_mirror(self) -> bool {
        !self.transpose && self.flip_x && !self.flip_y
    }

    /// Upright dimensions of a stored image.
    pub(crate) fn display_size(self, stored: Size) -> Result<Size, Error> {
        if self.transpose {
            Size::new(stored.height, stored.width)
        } else {
            Ok(stored)
        }
    }

    /// The stored rectangle that contains exactly the samples of `display`.
    pub(crate) fn stored_rect(self, display: Rect, stored: Size) -> Result<Rect, Error> {
        let (x, y, width, height) = if self.transpose {
            (display.y, display.x, display.height, display.width)
        } else {
            (display.x, display.y, display.width, display.height)
        };
        let mirror = |offset: u32, extent: u32, total: u32| {
            total
                .checked_sub(offset)
                .and_then(|rest| rest.checked_sub(extent))
                .ok_or(Error::OutOfBounds)
        };
        let x = if self.flip_x {
            mirror(x, width, stored.width)?
        } else {
            x
        };
        let y = if self.flip_y {
            mirror(y, height, stored.height)?
        } else {
            y
        };
        Rect::new(x, y, width, height)
    }
}

/// Presents an upright view of a source whose samples need reorientation.
pub(crate) struct OrientedSource<'a> {
    source: Box<dyn DecodeSource + 'a>,
    orientation: Orientation,
    info: DecoderInfo,
    /// Threads that gather upright rows from the stored-order buffer.
    threads: usize,
    /// Downstream working-set headroom (bytes) to preserve when deciding how
    /// many optional gather blocks beyond block 0 to pre-allocate.
    downstream_headroom: usize,
}

impl<'a> OrientedSource<'a> {
    pub(crate) fn new(source: Box<dyn DecodeSource + 'a>) -> Result<Self, Error> {
        let orientation = source.orientation();
        let mut info = source.info().clone();
        info.size = orientation.display_size(info.size)?;
        Ok(Self {
            source,
            orientation,
            info,
            threads: 1,
            downstream_headroom: 0,
        })
    }

    /// Gather upright rows on `threads` threads (ADR-0023). The rows and
    /// their order are the same for every thread count.
    #[cfg_attr(
        not(feature = "avif-write"),
        allow(dead_code, reason = "used by the facade")
    )]
    pub(crate) fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads.max(1);
        self
    }
}

/// Interleaved sample types that decoders emit.
trait RowSample: Copy + Default + Send + Sync {
    fn wrap(row: &[Self]) -> DecodedRow<'_>;
    fn unwrap(row: DecodedRow<'_>) -> Option<&[Self]>;
}

impl RowSample for u8 {
    fn wrap(row: &[Self]) -> DecodedRow<'_> {
        DecodedRow::U8(row)
    }
    fn unwrap(row: DecodedRow<'_>) -> Option<&[Self]> {
        match row {
            DecodedRow::U8(samples) => Some(samples),
            DecodedRow::F32(_) => None,
        }
    }
}

impl RowSample for f32 {
    fn wrap(row: &[Self]) -> DecodedRow<'_> {
        DecodedRow::F32(row)
    }
    fn unwrap(row: DecodedRow<'_>) -> Option<&[Self]> {
        match row {
            DecodedRow::F32(samples) => Some(samples),
            DecodedRow::U8(_) => None,
        }
    }
}

/// Geometry shared by every row of one reorientation.
struct Layout {
    orientation: Orientation,
    display: Rect,
    stored: Rect,
    stored_size: Size,
    stride: usize,
    channels: usize,
    row_len: usize,
}

/// Reverse interleaved pixels of `N` samples each from `src` into `out`.
#[inline]
fn reverse_pixels<T: Copy, const N: usize>(src: &[T], out: &mut [T]) {
    for (dst, &pixel) in out
        .as_chunks_mut::<N>()
        .0
        .iter_mut()
        .zip(src.as_chunks::<N>().0.iter().rev())
    {
        *dst = pixel;
    }
}

/// Reverse one row of interleaved pixels (`1..=4` channels) from `src` into `out`.
fn reverse_row<T: Copy>(src: &[T], out: &mut [T], channels: usize) -> Result<(), Error> {
    if src.len() != out.len() || !src.len().is_multiple_of(channels) {
        return Err(Error::OutOfBounds);
    }
    match channels {
        1 => reverse_pixels::<T, 1>(src, out),
        2 => reverse_pixels::<T, 2>(src, out),
        3 => reverse_pixels::<T, 3>(src, out),
        4 => reverse_pixels::<T, 4>(src, out),
        _ => return Err(Error::UnsupportedCodecLayout),
    }
    Ok(())
}

/// Fill display row `y`. Non-transposing orientations copy or reverse a whole
/// stored row directly; transposing orientations gather fixed-size pixels from
/// one stored column with row-level bounds checks hoisted outside the loop.
fn gather_row<T: Copy>(pixels: &[T], out: &mut [T], layout: &Layout, y: u32) -> Result<(), Error> {
    if !layout.orientation.transpose {
        let sy = if layout.orientation.flip_y {
            layout
                .stored_size
                .height
                .checked_sub(1)
                .and_then(|h| h.checked_sub(y))
                .ok_or(Error::OutOfBounds)?
        } else {
            y
        };
        let local_y = usize::try_from(sy.checked_sub(layout.stored.y).ok_or(Error::OutOfBounds)?)
            .map_err(|_| Error::GeometryOverflow)?;
        let start = local_y
            .checked_mul(layout.stride)
            .ok_or(Error::GeometryOverflow)?;
        let end = start
            .checked_add(layout.row_len)
            .ok_or(Error::GeometryOverflow)?;
        let src_row = pixels.get(start..end).ok_or(Error::OutOfBounds)?;
        if out.len() != src_row.len() {
            return Err(Error::OutOfBounds);
        }
        if layout.orientation.flip_x {
            return reverse_row(src_row, out, layout.channels);
        }
        out.copy_from_slice(src_row);
        return Ok(());
    }
    match layout.channels {
        1 => gather_transpose::<T, 1>(pixels, out, layout, y),
        2 => gather_transpose::<T, 2>(pixels, out, layout, y),
        3 => gather_transpose::<T, 3>(pixels, out, layout, y),
        4 => gather_transpose::<T, 4>(pixels, out, layout, y),
        _ => Err(Error::UnsupportedCodecLayout),
    }
}

/// Below this many display rows, gather threads cost more than they save.
pub(crate) const PARALLEL_MIN_ROWS: u32 = 64;

/// Whether [`OrientedSource`] will keep parallel gather workers alive while
/// delivering rows for a `display_height`-row upright region.
///
/// Only transposing orientations (EXIF 5–8) parallelize the gather (issue #68).
pub(crate) fn will_emit_parallel(
    has_transposing_orientation: bool,
    display_height: u32,
    threads: usize,
) -> bool {
    has_transposing_orientation && threads > 1 && display_height >= PARALLEL_MIN_ROWS
}
/// Display rows per parallel gather job.
const BLOCK_ROWS: u32 = 16;

type Block<T> = Result<(usize, ManagedBuffer<T>), Error>;

struct GatherJob<T> {
    block: usize,
    buffer: ManagedBuffer<T>,
}

/// Gather blocks of display rows on `threads` workers and hand the rows to
/// `sink` in order on the calling thread. Block 0 is pre-allocated mandatorily
/// and optional additional blocks up to `min(threads + 1, blocks)` are
/// pre-allocated only while `budget.available() >= block_bytes +
/// downstream_headroom`, so at most `threads + 1` blocks are ever live at once,
/// gather workers perform zero `MemoryBudget` allocations once callbacks begin,
/// tight budgets gracefully shrink to `1..=max_window` reusable blocks, and
/// budget admission stays strictly monotonic. Errors, including a cancelling
/// sink, close the job queue; the workers exit and the scope joins them before
/// this returns (as in ADR-0022's pipeline).
fn emit_parallel<T: RowSample>(
    pixels: &[T],
    layout: &Layout,
    threads: usize,
    budget: &MemoryBudget,
    downstream_headroom: usize,
    sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
) -> Result<(), Error> {
    let blocks = usize::try_from(layout.display.height.div_ceil(BLOCK_ROWS))
        .map_err(|_| Error::GeometryOverflow)?;
    let max_window = threads.saturating_add(1).min(blocks);
    let mut preallocated = Vec::with_capacity(max_window);
    for block in 0..max_window {
        let (_, count) = block_rows(layout, block)?;
        let len = usize::try_from(count)
            .ok()
            .and_then(|rows| rows.checked_mul(layout.row_len))
            .ok_or(Error::GeometryOverflow)?;
        if block > 0 {
            let block_bytes = len
                .checked_mul(std::mem::size_of::<T>())
                .ok_or(Error::GeometryOverflow)?;
            if budget.available() < block_bytes.saturating_add(downstream_headroom) {
                break;
            }
        }
        preallocated.push(budget.allocate_typed::<T>(len)?);
    }
    let window = preallocated.len();
    let (jobs, job_receiver) = mpsc::sync_channel::<GatherJob<T>>(window);
    let job_receiver = Mutex::new(job_receiver);
    let (done_sender, done) = mpsc::channel::<Block<T>>();
    thread::scope(|scope| {
        for _ in 0..threads.min(window) {
            let done_sender = done_sender.clone();
            let job_receiver = &job_receiver;
            if crate::thread_spawn::spawn(scope, move || {
                gather_worker(pixels, layout, job_receiver, &done_sender);
            })
            .is_err()
            {
                // Close the queue before scope joins any started workers.
                drop(jobs);
                return Err(Error::ThreadSpawnFailed);
            }
        }
        drop(done_sender);
        emit_blocks(layout, blocks, preallocated, jobs, &done, sink)
    })
}

fn gather_worker<T: RowSample>(
    pixels: &[T],
    layout: &Layout,
    jobs: &Mutex<Receiver<GatherJob<T>>>,
    done: &Sender<Block<T>>,
) {
    loop {
        let job = match jobs.lock() {
            Ok(receiver) => receiver.recv(),
            Err(_) => return,
        };
        let Ok(job) = job else {
            return;
        };
        let block = job.block;
        let result = gather_block(pixels, layout, job).map(|rows| (block, rows));
        if done.send(result).is_err() {
            return;
        }
    }
}

fn block_rows(layout: &Layout, block: usize) -> Result<(u32, u32), Error> {
    let first = u32::try_from(block)
        .ok()
        .and_then(|block| block.checked_mul(BLOCK_ROWS))
        .and_then(|offset| layout.display.y.checked_add(offset))
        .ok_or(Error::GeometryOverflow)?;
    Ok((first, (layout.display.bottom() - first).min(BLOCK_ROWS)))
}

fn gather_block<T: RowSample>(
    pixels: &[T],
    layout: &Layout,
    mut job: GatherJob<T>,
) -> Result<ManagedBuffer<T>, Error> {
    let (first, count) = block_rows(layout, job.block)?;
    let rows = usize::try_from(count).map_err(|_| Error::GeometryOverflow)?;
    let len = rows
        .checked_mul(layout.row_len)
        .ok_or(Error::GeometryOverflow)?;
    let slice = job
        .buffer
        .as_mut_slice()
        .get_mut(..len)
        .ok_or(Error::OutOfBounds)?;
    gather_rows(pixels, slice, layout, first)?;
    Ok(job.buffer)
}

/// Display columns per tile of a transposing gather.
const TILE_COLUMNS: u32 = 64;

/// Fill the consecutive display rows from `first` that `out` holds.
///
/// Without a transpose each display row reads one stored row, so rows are
/// copied or reversed one by one. With a transpose a display row reads a
/// stored column; the rows are then gathered in tiles of `TILE_COLUMNS`
/// columns, so that the stored pixels of neighboring display rows, which are
/// adjacent in one stored row, are read while their cache lines are loaded.
fn gather_rows<T: Copy>(
    pixels: &[T],
    out: &mut [T],
    layout: &Layout,
    first: u32,
) -> Result<(), Error> {
    if !layout.orientation.transpose || layout.row_len == 0 {
        for (y, row) in (first..).zip(out.chunks_exact_mut(layout.row_len)) {
            gather_row(pixels, row, layout, y)?;
        }
        return Ok(());
    }
    let mut x = layout.display.x;
    while x < layout.display.right() {
        let width = (layout.display.right() - x).min(TILE_COLUMNS);
        let tile = Layout {
            display: Rect {
                x,
                y: layout.display.y,
                width,
                height: layout.display.height,
            },
            ..*layout
        };
        let start = usize::try_from(x - layout.display.x)
            .ok()
            .and_then(|offset| offset.checked_mul(layout.channels))
            .ok_or(Error::GeometryOverflow)?;
        let end = usize::try_from(width)
            .ok()
            .and_then(|width| width.checked_mul(layout.channels))
            .and_then(|len| len.checked_add(start))
            .ok_or(Error::GeometryOverflow)?;
        for (y, row) in (first..).zip(out.chunks_exact_mut(layout.row_len)) {
            let part = row.get_mut(start..end).ok_or(Error::OutOfBounds)?;
            gather_row(pixels, part, &tile, y)?;
        }
        x += width;
    }
    Ok(())
}

/// Fill one transposing display row (EXIF orientations 5–8) from one column of
/// the stored-order buffer, `N` samples per pixel. Bounds are checked once for
/// the span so the inner loop is a straight strided pixel copy.
fn gather_transpose<T: Copy, const N: usize>(
    pixels: &[T],
    out: &mut [T],
    layout: &Layout,
    y: u32,
) -> Result<(), Error> {
    let out = out.as_chunks_mut::<N>().0;
    let count = out.len();
    if count == 0 {
        return Ok(());
    }
    let sx = if layout.orientation.flip_x {
        layout
            .stored_size
            .width
            .checked_sub(1)
            .and_then(|w| w.checked_sub(y))
            .ok_or(Error::OutOfBounds)?
    } else {
        y
    };
    let local_x = usize::try_from(sx.checked_sub(layout.stored.x).ok_or(Error::OutOfBounds)?)
        .map_err(|_| Error::GeometryOverflow)?;
    let stride_px = layout.stride / N;
    if local_x >= stride_px {
        return Err(Error::OutOfBounds);
    }
    let src_px = pixels.as_chunks::<N>().0;
    let span = u32::try_from(count - 1).map_err(|_| Error::GeometryOverflow)?;
    if layout.orientation.flip_y {
        let sy_first = layout
            .stored_size
            .height
            .checked_sub(1)
            .and_then(|h| h.checked_sub(layout.display.x))
            .ok_or(Error::OutOfBounds)?;
        let sy_last = sy_first.checked_sub(span).ok_or(Error::OutOfBounds)?;
        let first_y = usize::try_from(
            sy_first
                .checked_sub(layout.stored.y)
                .ok_or(Error::OutOfBounds)?,
        )
        .map_err(|_| Error::GeometryOverflow)?;
        let last_y = usize::try_from(
            sy_last
                .checked_sub(layout.stored.y)
                .ok_or(Error::OutOfBounds)?,
        )
        .map_err(|_| Error::GeometryOverflow)?;
        let mut idx = first_y
            .checked_mul(stride_px)
            .and_then(|o| o.checked_add(local_x))
            .ok_or(Error::GeometryOverflow)?;
        let last_idx = last_y
            .checked_mul(stride_px)
            .and_then(|o| o.checked_add(local_x))
            .ok_or(Error::GeometryOverflow)?;
        if idx >= src_px.len() || last_idx >= src_px.len() {
            return Err(Error::OutOfBounds);
        }
        for pixel in out {
            *pixel = src_px[idx];
            idx = idx.wrapping_sub(stride_px);
        }
    } else {
        let first_y = usize::try_from(
            layout
                .display
                .x
                .checked_sub(layout.stored.y)
                .ok_or(Error::OutOfBounds)?,
        )
        .map_err(|_| Error::GeometryOverflow)?;
        let last_y = first_y
            .checked_add(count - 1)
            .ok_or(Error::GeometryOverflow)?;
        let mut idx = first_y
            .checked_mul(stride_px)
            .and_then(|o| o.checked_add(local_x))
            .ok_or(Error::GeometryOverflow)?;
        let last_idx = last_y
            .checked_mul(stride_px)
            .and_then(|o| o.checked_add(local_x))
            .ok_or(Error::GeometryOverflow)?;
        if last_idx >= src_px.len() {
            return Err(Error::OutOfBounds);
        }
        for pixel in out {
            *pixel = src_px[idx];
            idx += stride_px;
        }
    }
    Ok(())
}

/// The calling thread's side of `emit_parallel`. It owns the job sender, so
/// that returning for any reason closes the queue and the workers exit.
#[allow(
    clippy::needless_pass_by_value,
    reason = "dropping the sender on return is what stops the workers"
)]
fn emit_blocks<T: RowSample>(
    layout: &Layout,
    blocks: usize,
    preallocated: Vec<ManagedBuffer<T>>,
    jobs: SyncSender<GatherJob<T>>,
    done: &Receiver<Block<T>>,
    sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
) -> Result<(), Error> {
    let mut submitted = 0;
    for buffer in preallocated {
        jobs.send(GatherJob {
            block: submitted,
            buffer,
        })
        .map_err(|_| Error::IncompleteDecoderOutput)?;
        submitted += 1;
    }
    let mut finished = BTreeMap::new();
    for next in 0..blocks {
        let rows = loop {
            if let Some(rows) = finished.remove(&next) {
                break rows;
            }
            let (block, rows) = done.recv().map_err(|_| Error::IncompleteDecoderOutput)??;
            finished.insert(block, rows);
        };
        let (first, count) = block_rows(layout, next)?;
        let len = usize::try_from(count)
            .ok()
            .and_then(|rows| rows.checked_mul(layout.row_len))
            .ok_or(Error::GeometryOverflow)?;
        let slice = rows.as_slice().get(..len).ok_or(Error::OutOfBounds)?;
        for (y, row) in (first..).zip(slice.chunks_exact(layout.row_len)) {
            sink(y, T::wrap(row))?;
        }
        // Reuse the emitting block for the next job so at most `window`
        // (`<= threads + 1`) blocks are ever live simultaneously with zero
        // allocations during callbacks.
        if submitted < blocks {
            jobs.send(GatherJob {
                block: submitted,
                buffer: rows,
            })
            .map_err(|_| Error::IncompleteDecoderOutput)?;
            submitted += 1;
        }
    }
    Ok(())
}

/// Stream EXIF orientation 2 (`flip_x` only) row by row in O(1 row) memory
/// without materializing the stored region (issue #68, ADR-0019).
fn reorient_stream_flip_x<T: RowSample>(
    source: &mut dyn DecodeSource,
    stored: Rect,
    channels: usize,
    budget: &MemoryBudget,
    sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
) -> Result<(DecodeReport, usize, u64), Error> {
    if !matches!(channels, 1..=4) {
        return Err(Error::UnsupportedCodecLayout);
    }
    let stride = usize::try_from(stored.width)
        .ok()
        .and_then(|width| width.checked_mul(channels))
        .ok_or(Error::GeometryOverflow)?;
    let mut row_buf = budget.allocate_typed::<T>(stride)?;
    let mut expected_y = stored.y;
    let report = source.read_region(stored, budget, &mut |y, row| {
        let samples = T::unwrap(row).ok_or(Error::IncompleteDecoderOutput)?;
        if y != expected_y || samples.len() != stride {
            return Err(Error::IncompleteDecoderOutput);
        }
        reverse_row(samples, row_buf.as_mut_slice(), channels)?;
        sink(y, T::wrap(row_buf.as_slice()))?;
        expected_y += 1;
        Ok(())
    })?;
    if expected_y != stored.bottom() {
        return Err(Error::IncompleteDecoderOutput);
    }
    let copied_bytes = u64::from(stored.height)
        .checked_mul(u64::try_from(stride).map_err(|_| Error::GeometryOverflow)?)
        .and_then(|n| n.checked_mul(u64::try_from(std::mem::size_of::<T>()).ok()?))
        .ok_or(Error::GeometryOverflow)?;
    Ok((report, 0, copied_bytes))
}

/// Reorient decoded stored rows and emit upright rows. Orientation 2 streams
/// row-by-row with a single row buffer when `source.streams_rows()` is true
/// (e.g. scanline JPEG) AND the requested stored region exceeds one full-width
/// source scanline (so tiny crops like `1×1` stay buffered and release the
/// decoder's scanline before downstream resize buffers allocate); frame-backed
/// sources (WebP/AVIF/JXL) and orientations 3–8 materialize the stored region
/// once so the decoder can release its frame before downstream resize buffers
/// are allocated (parallel gather only for transposing orientations 5–8 when
/// `threads > 1`).
#[allow(clippy::too_many_arguments)] // Geometry, threads, budget, headroom and sink.
fn reorient<T: RowSample>(
    source: &mut dyn DecodeSource,
    orientation: Orientation,
    display: Rect,
    stored: Rect,
    channels: usize,
    threads: usize,
    budget: &MemoryBudget,
    downstream_headroom: usize,
    sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
) -> Result<(DecodeReport, usize, u64), Error> {
    let region_pixels = u64::from(stored.width).saturating_mul(u64::from(stored.height));
    if orientation.is_horizontal_mirror()
        && source.streams_rows()
        && region_pixels > u64::from(source.info().size.width)
    {
        return reorient_stream_flip_x::<T>(source, stored, channels, budget, sink);
    }
    let stored_size = source.info().size;
    let stride = usize::try_from(stored.width)
        .ok()
        .and_then(|width| width.checked_mul(channels))
        .ok_or(Error::GeometryOverflow)?;
    let length = usize::try_from(stored.height)
        .ok()
        .and_then(|height| height.checked_mul(stride))
        .ok_or(Error::GeometryOverflow)?;
    let mut region = budget.allocate_typed::<T>(length)?;
    let mut expected_y = stored.y;
    let report = source.read_region(stored, budget, &mut |y, row| {
        let samples = T::unwrap(row).ok_or(Error::IncompleteDecoderOutput)?;
        if y != expected_y || samples.len() != stride {
            return Err(Error::IncompleteDecoderOutput);
        }
        let start = usize::try_from(y - stored.y).map_err(|_| Error::GeometryOverflow)? * stride;
        region.as_mut_slice()[start..start + stride].copy_from_slice(samples);
        expected_y += 1;
        Ok(())
    })?;
    if expected_y != stored.bottom() {
        return Err(Error::IncompleteDecoderOutput);
    }

    let row_len = usize::try_from(display.width)
        .ok()
        .and_then(|width| width.checked_mul(channels))
        .ok_or(Error::GeometryOverflow)?;
    let layout = Layout {
        orientation,
        display,
        stored,
        stored_size,
        stride,
        channels,
        row_len,
    };
    let pixels = region.as_slice();
    // Only transposing orientations (5–8) parallelize the gather; orientations
    // 3 and 4 are single-row copy/reverse passes where a thread pool and
    // parallel block queue add memory and latency overhead (issue #68).
    if will_emit_parallel(orientation.transpose, display.height, threads) {
        emit_parallel(pixels, &layout, threads, budget, downstream_headroom, sink)?;
    } else {
        // A transposing gather fills blocks of display rows so that it can
        // tile (`gather_rows`); this holds BLOCK_ROWS rows instead of one.
        // Other orientations keep a single row.
        let step = if layout.orientation.transpose {
            display.height.min(BLOCK_ROWS)
        } else {
            display.height.min(1)
        };
        let block_len = usize::try_from(step)
            .ok()
            .and_then(|rows| rows.checked_mul(row_len))
            .ok_or(Error::GeometryOverflow)?;
        let mut block = budget.allocate_typed::<T>(block_len)?;
        let mut first = display.y;
        while first < display.bottom() {
            let rows = (display.bottom() - first).min(step);
            let len = usize::try_from(rows)
                .ok()
                .and_then(|rows| rows.checked_mul(row_len))
                .ok_or(Error::GeometryOverflow)?;
            let part = block
                .as_mut_slice()
                .get_mut(..len)
                .ok_or(Error::OutOfBounds)?;
            gather_rows(pixels, part, &layout, first)?;
            for (y, row) in (first..).zip(part.chunks_exact(row_len.max(1))) {
                sink(y, T::wrap(row))?;
            }
            first += rows;
        }
    }
    let bytes = length
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(Error::GeometryOverflow)?;
    // One copy fills the stored-order region, one gathers it upright.
    let copied_bytes = u64::try_from(bytes)
        .ok()
        .and_then(|bytes| bytes.checked_mul(2))
        .ok_or(Error::GeometryOverflow)?;
    Ok((report, bytes, copied_bytes))
}

impl DecodeSource for OrientedSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }

    fn set_orientation_threads(&mut self, threads: usize) {
        self.threads = threads.max(1);
    }

    fn has_transposing_orientation(&self) -> bool {
        self.orientation.transpose
    }

    fn streams_rows(&self) -> bool {
        self.orientation.is_horizontal_mirror() && self.source.streams_rows()
    }

    fn prepare_conversion(
        &mut self,
        expected_pixels: u64,
        budget: &MemoryBudget,
    ) -> Result<(), Error> {
        self.source.prepare_conversion(expected_pixels, budget)
    }

    fn set_downstream_headroom(&mut self, bytes: usize) {
        self.downstream_headroom = bytes;
    }

    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        if !region.fits(self.info.size) {
            return Err(Error::OutOfBounds);
        }
        let stored = self
            .orientation
            .stored_rect(region, self.source.info().size)?;
        let channels = usize::from(self.info.layout.channels());
        let source = &mut *self.source;
        let downstream_headroom = self.downstream_headroom;
        let (inner, bytes, copied_bytes) = match self.info.sample {
            SampleType::U8 => reorient::<u8>(
                source,
                self.orientation,
                region,
                stored,
                channels,
                self.threads,
                budget,
                downstream_headroom,
                sink,
            )?,
            SampleType::F32 => reorient::<f32>(
                source,
                self.orientation,
                region,
                stored,
                channels,
                self.threads,
                budget,
                downstream_headroom,
                sink,
            )?,
        };
        Ok(DecodeReport {
            requested: region,
            rows: u64::from(region.height),
            engine_buffer_bytes: inner
                .engine_buffer_bytes
                .checked_add(bytes)
                .ok_or(Error::GeometryOverflow)?,
            engine_copied_bytes: copied_bytes
                .checked_add(inner.engine_copied_bytes)
                .ok_or(Error::GeometryOverflow)?,
            peak_engine_managed_bytes: budget.snapshot().peak,
            ..inner
        })
    }
}

#[cfg(test)]
#[path = "orientation_tests.rs"]
mod tests;

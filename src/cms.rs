//! Optional, explicit embedded-ICC -> linear-light sRGB pixel transform.
//!
//! No ICC profile is reinterpreted as sRGB. The caller must explicitly opt
//! into using the decoder's **rendered output ICC** signal. The transform
//! operates on borrowed input rows and engine-budgeted f32 scratch buffers.
//! The codec's own image allocations remain independently opaque.
//!
//! Opaque RGB and RGBA with **straight** alpha are accepted (issue #222).
//! Only the three color channels go through the profile transform; the
//! alpha sample passes through unchanged (8-bit alpha normalized to `[0, 1]`
//! like the color channels, float alpha copied as is), so the output stays
//! linear-light sRGB with straight alpha (ADR-0014). Premultiplied and
//! unspecified alpha are refused rather than guessed (ADR-0015).
//!
//! Gray and gray with straight alpha are accepted with a gray (`GRAY`)
//! profile (owner decision 2026-11-15, ADR-0043): moxcms converts the gray
//! channel through the profile's tone curve into linear-light sRGB RGB
//! (R = G = B), and alpha bypasses the transform as for RGBA. A gray
//! profile beside RGB samples, or an RGB profile beside gray samples, is
//! refused (`IccColorSpaceMismatch`); neither is reinterpreted.

use std::fmt;
use std::sync::{Arc, Mutex};

use moxcms::{
    ColorProfile, DataColorSpace, Layout, RenderingIntent, ToneReprCurve, TransformF32Executor,
    TransformOptions,
};

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, RowConversion, SampleType,
};
use crate::error::Error;
use crate::geometry::Rect;
use crate::memory::{ExternalReservation, MemoryBudget};

/// Maximum number of validated ICC profiles retained in one batch-scoped
/// [`IccTableCache`].
const MAX_CACHE_ENTRIES: usize = 32;

/// Default maximum retained bytes allowed in one batch-scoped
/// [`IccTableCache`] (512 KiB).
pub(crate) const MAX_CACHE_BYTES: usize = 512 * 1024;

/// Maximum single ICC profile size eligible for caching (256 KiB). Standard
/// RGB and gray profiles are well under this limit; capping prevents oversized
/// profiles from dominating the batch cache budget.
const MAX_CACHED_PROFILE_BYTES: usize = 256 * 1024;

/// Control-block header bytes (strong + weak counts) for one `Arc` allocation.
const ARC_HEADER_BYTES: usize = 2 * std::mem::size_of::<usize>();

/// Minimum full-image pixel count before an 8-bit RGB matrix-shaper image
/// pays the 7,473-pixel `ChannelTable` build and probe cost (issue #219).
/// Keyed on full pre-crop source dimensions (never crop size or cache state)
/// so overlapping crops stay byte-identical (ADR-0020).
pub(crate) const RGB_TABLE_MIN_PIXELS: u64 = 10_000;

/// Minimum resolved source-crop pixel count before an 8-bit gray image uses
/// the exact 256-entry `GrayTable` (issue #219). Applied identically on cache
/// hits and misses so managed-memory budget admission never depends on cache
/// warmth or batch worker scheduling.
pub(crate) const GRAY_TABLE_MIN_PIXELS: u64 = 512;

/// Deterministic 64-bit FNV-1a digest used to fast-reject non-matching ICC
/// cache entries before comparing the raw profile bytes for exact equality.
fn hash_profile_bytes(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Payload bytes of one 768-entry 8-bit RGB [`ChannelTable`] reserved against
/// the image's [`MemoryBudget`] independently of batch cache warmth (issue #219).
pub(crate) const RGB_TABLE_BYTES: usize = std::mem::size_of::<[[f32; 4]; 768]>();

/// Payload bytes of one 256-entry 8-bit [`GrayTable`] reserved against the
/// image's [`MemoryBudget`] independently of batch cache warmth (issue #219).
pub(crate) const GRAY_TABLE_BYTES: usize = std::mem::size_of::<[[f32; 4]; 256]>();

/// Exact heap byte size of `Arc<T>`'s inner allocation (`ArcInner<T>` = strong
/// and weak reference counts plus `T`), rounded up to the allocation's
/// alignment (`align_of::<usize>().max(align_of::<T>())`) so alignment padding
/// is never undercounted for any `T`.
const fn padded_arc_bytes<T>() -> usize {
    let usize_align = std::mem::align_of::<usize>();
    let t_align = std::mem::align_of::<T>();
    let max_align = if t_align > usize_align {
        t_align
    } else {
        usize_align
    };
    let header = ARC_HEADER_BYTES.next_multiple_of(t_align);
    (header + std::mem::size_of::<T>()).next_multiple_of(max_align)
}

const fn rgb_table_retained_bytes() -> usize {
    padded_arc_bytes::<ChannelTable>() + RGB_TABLE_BYTES
}

const fn gray_table_retained_bytes() -> usize {
    padded_arc_bytes::<GrayTable>() + GRAY_TABLE_BYTES
}

/// Fallibly copy raw ICC profile bytes into an exact-capacity `Vec<u8>` key
/// so OOM degrades cleanly by skipping caching instead of aborting.
fn try_alloc_key(bytes: &[u8]) -> Option<Vec<u8>> {
    let mut key = Vec::new();
    key.try_reserve_exact(bytes.len()).ok()?;
    key.extend_from_slice(bytes);
    Some(key)
}

#[derive(Clone)]
enum CachedTable {
    /// `Some(table)` for verified 8-bit RGB matrix-shaper profiles; `None`
    /// only when an extended matrix-shaper profile failed the `ChannelTable`
    /// probe (ineligible LUT profiles are not cached).
    Rgb(Option<Arc<ChannelTable>>),
    /// `Some(table)` for verified 8-bit gray profiles; `None` if `moxcms`
    /// produced non-finite output during `GrayTable::build`.
    Gray(Option<Arc<GrayTable>>),
}

impl CachedTable {
    const fn retained_bytes(&self) -> usize {
        match self {
            Self::Rgb(Some(_)) => rgb_table_retained_bytes(),
            Self::Gray(Some(_)) => gray_table_retained_bytes(),
            Self::Rgb(None) | Self::Gray(None) => 0,
        }
    }

    fn is_in_use(&self) -> bool {
        match self {
            Self::Rgb(Some(table)) => Arc::strong_count(table) > 1,
            Self::Gray(Some(table)) => Arc::strong_count(table) > 1,
            Self::Rgb(None) | Self::Gray(None) => false,
        }
    }
}

struct CachedProfile {
    hash: u64,
    bytes: Vec<u8>,
    gray: bool,
    table: CachedTable,
    // Covers `Vec<u8>` key capacity; dropped after `bytes`. Table storage is
    // reserved inside the shared `Arc<ChannelTable>` / `Arc<GrayTable>` itself
    // so bytes stay charged until the last concurrent reader drops its handle.
    _key_reservation: ExternalReservation,
}

impl CachedProfile {
    fn retained_bytes(&self) -> usize {
        self.bytes
            .capacity()
            .saturating_add(self.table.retained_bytes())
    }

    /// Bytes immediately freed in the cache budget if this entry is evicted.
    /// Entries whose table `Arc` is currently held by an active batch worker
    /// (`strong_count > 1`) stay cached and report `0` reclaimable bytes.
    fn reclaimable_bytes(&self) -> usize {
        if self.table.is_in_use() {
            0
        } else {
            self.retained_bytes()
        }
    }
}

/// Zero-copy hit returned from [`IccTableCache::get`]: clones only reference-
/// counted table handles (`Arc`) while holding the mutex, never profile bytes
/// or `moxcms` executors.
struct CacheHit {
    gray: bool,
    table: CachedTable,
}

#[derive(Default)]
struct CacheState {
    // Dropped before `vec_reservation` so the backing storage is freed before
    // releasing its budget reservation.
    entries: Vec<CachedProfile>,
    vec_reservation: Option<ExternalReservation>,
}

/// Batch-scoped, `MemoryBudget`-backed, thread-safe cache of verified
/// [`ChannelTable`]/[`GrayTable`] lookup tables, keyed by exact raw ICC
/// profile bytes (issue #219, ADR-0004, ADR-0016, ADR-0022, ADR-0023).
///
/// Created for a single multi-item `Engine::transcode_batch` call and dropped
/// when that call returns so `Engine` retains zero bytes between calls. Every
/// retained heap allocation — `Vec<CachedProfile>` capacity, fallibly
/// allocated `Vec<u8>` profile keys, and alignment-padded `Arc<ChannelTable>`
/// / `Arc<GrayTable>` allocations and payloads (never
/// `moxcms::TransformF32Executor`) — is reserved against the cache's own
/// `MemoryBudget`, keeping live and peak bytes strictly `<= max_bytes` and
/// observable via [`Self::peak_bytes`].
/// Only profiles that passed header validation, `moxcms` parsing, transform
/// creation, and profile-tag CICP checks are stored. Ineligible LUT RGB
/// profiles are never cached so they cannot waste budget or evict reusable
/// tables. Cache hits require byte-for-byte identical profile bytes and return
/// only `Arc` handles without copying profile bytes; decoder-supplied CICP
/// beside the profile is still verified on every hit. The mutex is never held
/// while calling `moxcms`, and a poisoned mutex degrades cleanly to a cache
/// miss.
pub(crate) struct IccTableCache {
    max_bytes: usize,
    budget: MemoryBudget,
    state: Mutex<CacheState>,
}

impl fmt::Debug for IccTableCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.state.lock().map_or(0, |s| s.entries.len());
        let snap = self.budget.snapshot();
        f.debug_struct("IccTableCache")
            .field("entries", &len)
            .field("live_bytes", &snap.live)
            .field("peak_bytes", &snap.peak)
            .field("max_bytes", &self.max_bytes)
            .finish()
    }
}

enum PrepareInsert<'a> {
    /// Another concurrent worker already cached the exact profile bytes while
    /// we were building; reuse its shared handle without evicting.
    Existing(CachedTable),
    /// Ready to insert a new entry with its fallibly allocated key and
    /// cache-budget reservations.
    Ready(
        std::sync::MutexGuard<'a, CacheState>,
        Vec<u8>,
        ExternalReservation,
        Option<ExternalReservation>,
    ),
    /// Cannot cache (disabled, oversized, OOM, or in-use tables occupy budget).
    Skip,
}

impl IccTableCache {
    pub(crate) fn with_max_bytes(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            budget: MemoryBudget::new(max_bytes),
            state: Mutex::new(CacheState::default()),
        }
    }

    #[cfg(test)]
    pub(crate) fn new() -> Self {
        Self::with_max_bytes(MAX_CACHE_BYTES)
    }

    pub(crate) fn peak_bytes(&self) -> usize {
        self.budget.snapshot().peak
    }

    #[cfg(test)]
    pub(crate) fn live_bytes(&self) -> usize {
        self.budget.snapshot().live
    }

    fn get(&self, hash: u64, bytes: &[u8]) -> Option<CacheHit> {
        if self.max_bytes == 0 {
            return None;
        }
        let mut state = self.state.lock().ok()?;
        let index = state
            .entries
            .iter()
            .position(|entry| entry.hash == hash && &*entry.bytes == bytes)?;
        if index != 0 {
            let entry = state.entries.remove(index);
            state.entries.insert(0, entry);
        }
        let entry = &state.entries[0];
        Some(CacheHit {
            gray: entry.gray,
            table: entry.table.clone(),
        })
    }

    /// Ensure `state.entries` has room for at least one more slot while keeping
    /// its backing capacity reserved under `self.budget`.
    fn reserve_vec_slot(&self, state: &mut CacheState) -> bool {
        if state.entries.len() < state.entries.capacity() {
            return true;
        }
        let slot_bytes = std::mem::size_of::<CachedProfile>();
        let old_cap = state.entries.capacity();
        let target_cap = old_cap + 1;
        state.vec_reservation = None;
        let restore_old = |state: &mut CacheState| {
            if old_cap > 0 {
                state.vec_reservation = self
                    .budget
                    .reserve_external(old_cap.saturating_mul(slot_bytes))
                    .ok();
            }
        };
        let Ok(vec_res) = self
            .budget
            .reserve_external(target_cap.saturating_mul(slot_bytes))
        else {
            restore_old(state);
            return false;
        };
        if state.entries.try_reserve_exact(1).is_err() {
            drop(vec_res);
            restore_old(state);
            return false;
        }
        let actual_cap = state.entries.capacity();
        if actual_cap == target_cap {
            state.vec_reservation = Some(vec_res);
            return true;
        }
        drop(vec_res);
        if let Ok(actual_res) = self
            .budget
            .reserve_external(actual_cap.saturating_mul(slot_bytes))
        {
            state.vec_reservation = Some(actual_res);
            true
        } else {
            state.entries.shrink_to(old_cap);
            let shrunk_cap = state.entries.capacity();
            if shrunk_cap > 0 {
                state.vec_reservation = self
                    .budget
                    .reserve_external(shrunk_cap.saturating_mul(slot_bytes))
                    .ok();
            }
            false
        }
    }

    /// Evict idle (`strong_count == 1` or negative) entries in LRU order if
    /// needed to make room for `entry_bytes`. Returns `false` without evicting
    /// anything if all entries are in use or idle entries cannot free enough
    /// space.
    fn evict_for_insert(
        &self,
        state: &mut CacheState,
        entry_bytes: usize,
        slot_bytes: usize,
    ) -> bool {
        let live = self.budget.snapshot().live;
        let initial_grow = if state.entries.len() == state.entries.capacity() {
            slot_bytes
        } else {
            0
        };
        let needs_eviction = state.entries.len() >= MAX_CACHE_ENTRIES
            || live
                .saturating_add(entry_bytes)
                .saturating_add(initial_grow)
                > self.max_bytes;
        if !needs_eviction {
            return true;
        }
        let evictable_count = state
            .entries
            .iter()
            .filter(|entry| !entry.table.is_in_use())
            .count();
        if evictable_count == 0 {
            return false;
        }
        let reclaimable: usize = state
            .entries
            .iter()
            .map(CachedProfile::reclaimable_bytes)
            .sum();
        let slack_reclaim = if evictable_count == state.entries.len() {
            state
                .entries
                .capacity()
                .saturating_sub(1)
                .saturating_mul(slot_bytes)
        } else {
            0
        };
        let available = self.max_bytes.saturating_sub(live);
        if available
            .saturating_add(reclaimable)
            .saturating_add(slack_reclaim)
            < entry_bytes
        {
            return false;
        }
        loop {
            let grow_bytes = if state.entries.len() == state.entries.capacity() {
                slot_bytes
            } else {
                0
            };
            if state.entries.len() < MAX_CACHE_ENTRIES
                && self
                    .budget
                    .snapshot()
                    .live
                    .saturating_add(entry_bytes)
                    .saturating_add(grow_bytes)
                    <= self.max_bytes
            {
                break;
            }
            let Some(evict_idx) = state
                .entries
                .iter()
                .rposition(|entry| !entry.table.is_in_use())
            else {
                break;
            };
            state.entries.remove(evict_idx);
        }
        true
    }

    fn prepare_insert(&self, hash: u64, bytes: &[u8], table_bytes: usize) -> PrepareInsert<'_> {
        let key_bytes = bytes.len();
        let entry_bytes = key_bytes.saturating_add(table_bytes);
        let slot_bytes = std::mem::size_of::<CachedProfile>();
        if self.max_bytes == 0
            || bytes.len() > MAX_CACHED_PROFILE_BYTES
            || entry_bytes.saturating_add(slot_bytes) > self.max_bytes
        {
            return PrepareInsert::Skip;
        }
        let Ok(mut state) = self.state.lock() else {
            return PrepareInsert::Skip;
        };
        // Concurrent double-miss on the same profile: reuse the already-cached
        // table and refresh its LRU recency rather than removing/evicting it.
        if let Some(index) = state
            .entries
            .iter()
            .position(|entry| entry.hash == hash && entry.bytes.as_slice() == bytes)
        {
            if index != 0 {
                let entry = state.entries.remove(index);
                state.entries.insert(0, entry);
            }
            return PrepareInsert::Existing(state.entries[0].table.clone());
        }
        if !self.evict_for_insert(&mut state, entry_bytes, slot_bytes) {
            return PrepareInsert::Skip;
        }
        if state.entries.len() >= MAX_CACHE_ENTRIES {
            return PrepareInsert::Skip;
        }
        // Release slack `Vec` capacity if evictions emptied the vector and the
        // old capacity would block a new entry that fits in `max_bytes`.
        if state.entries.is_empty()
            && state.entries.capacity() > 0
            && self.budget.snapshot().live.saturating_add(entry_bytes) > self.max_bytes
        {
            state.entries = Vec::new();
            state.vec_reservation = None;
        }
        let grow_bytes = if state.entries.len() == state.entries.capacity() {
            slot_bytes
        } else {
            0
        };
        if self
            .budget
            .snapshot()
            .live
            .saturating_add(entry_bytes)
            .saturating_add(grow_bytes)
            > self.max_bytes
        {
            return PrepareInsert::Skip;
        }
        if !self.reserve_vec_slot(&mut state) {
            return PrepareInsert::Skip;
        }
        let Ok(mut key_res) = self.budget.reserve_external(key_bytes) else {
            return PrepareInsert::Skip;
        };
        let Some(key) = try_alloc_key(bytes) else {
            return PrepareInsert::Skip;
        };
        if key.capacity() > key_bytes {
            drop(key_res);
            let Ok(exact_res) = self.budget.reserve_external(key.capacity()) else {
                return PrepareInsert::Skip;
            };
            key_res = exact_res;
        }
        let table_res = if table_bytes > 0 {
            let Ok(res) = self.budget.reserve_external(table_bytes) else {
                return PrepareInsert::Skip;
            };
            Some(res)
        } else {
            None
        };
        PrepareInsert::Ready(state, key, key_res, table_res)
    }

    fn insert_rgb(&self, hash: u64, bytes: &[u8], mut table: ChannelTable) -> Arc<ChannelTable> {
        match self.prepare_insert(hash, bytes, rgb_table_retained_bytes()) {
            PrepareInsert::Existing(CachedTable::Rgb(Some(existing))) => existing,
            PrepareInsert::Existing(_) | PrepareInsert::Skip => Arc::new(table),
            PrepareInsert::Ready(mut state, key, key_res, table_res) => {
                table.reservation = table_res;
                let arc = Arc::new(table);
                state.entries.insert(
                    0,
                    CachedProfile {
                        hash,
                        bytes: key,
                        gray: false,
                        table: CachedTable::Rgb(Some(arc.clone())),
                        _key_reservation: key_res,
                    },
                );
                arc
            }
        }
    }

    fn insert_negative_rgb(&self, hash: u64, bytes: &[u8]) {
        if let PrepareInsert::Ready(mut state, key, key_res, _) =
            self.prepare_insert(hash, bytes, 0)
        {
            state.entries.insert(
                0,
                CachedProfile {
                    hash,
                    bytes: key,
                    gray: false,
                    table: CachedTable::Rgb(None),
                    _key_reservation: key_res,
                },
            );
        }
    }

    fn insert_gray(&self, hash: u64, bytes: &[u8], mut table: GrayTable) -> Arc<GrayTable> {
        match self.prepare_insert(hash, bytes, gray_table_retained_bytes()) {
            PrepareInsert::Existing(CachedTable::Gray(Some(existing))) => existing,
            PrepareInsert::Existing(_) | PrepareInsert::Skip => Arc::new(table),
            PrepareInsert::Ready(mut state, key, key_res, table_res) => {
                table.reservation = table_res;
                let arc = Arc::new(table);
                state.entries.insert(
                    0,
                    CachedProfile {
                        hash,
                        bytes: key,
                        gray: true,
                        table: CachedTable::Gray(Some(arc.clone())),
                        _key_reservation: key_res,
                    },
                );
                arc
            }
        }
    }

    fn insert_negative_gray(&self, hash: u64, bytes: &[u8]) {
        if let PrepareInsert::Ready(mut state, key, key_res, _) =
            self.prepare_insert(hash, bytes, 0)
        {
            state.entries.insert(
                0,
                CachedProfile {
                    hash,
                    bytes: key,
                    gray: true,
                    table: CachedTable::Gray(None),
                    _key_reservation: key_res,
                },
            );
        }
    }
}

pub(crate) struct IccSrgbSource<'a> {
    source: Box<dyn DecodeSource + 'a>,
    output: DecoderInfo,
    state: IccState<'a>,
    /// The executor took the conversion; rows pass through unconverted.
    deferred: bool,
}

enum IccState<'a> {
    /// Profile and any CICP signal have been validated, while lookup-table
    /// probing and per-image `MemoryBudget` reservation wait for
    /// `DecodeSource::prepare_conversion` after all option/encoder checks pass.
    Pending(Box<PendingIcc<'a>>),
    /// Row conversion is finalized and ready for `defer_conversion` or
    /// `read_region`.
    Ready(Arc<IccConversion>),
}

struct PendingIcc<'a> {
    cache: Option<&'a IccTableCache>,
    hash: u64,
    cacheable: bool,
    gray: bool,
    channels: usize,
    output_channels: usize,
    input: SampleType,
    source_pixels: u64,
    outcome: ValidatedProfile,
}

enum ValidatedProfile {
    CacheHit {
        table: CachedTable,
        /// When a cache hit also carried decoder CICP, keep the already-built
        /// `moxcms` transform so a fallback non-table path does not rebuild it.
        cicp_transformer: Option<Arc<TransformF32Executor>>,
    },
    Miss {
        transformer: Arc<TransformF32Executor>,
        extended_matrix_shaper: bool,
    },
}

/// The moxcms transform of one profile into linear-light sRGB, per row.
pub(crate) struct IccConversion {
    /// `None` when `rgb_table` or `gray_table` handles every 8-bit row
    /// directly so no `moxcms` executor is retained in memory.
    transformer: Option<Arc<TransformF32Executor>>,
    input: SampleType,
    /// Interleaved input samples per pixel: 3 (opaque RGB) or 4 (RGB
    /// followed by straight alpha, which bypasses the transform); 1 or 2 for
    /// gray and gray with straight alpha.
    channels: usize,
    /// Interleaved output samples per pixel: 3 or 4.
    output_channels: usize,
    /// Per-channel contributions for 8-bit RGB/RGBA matrix-shaper input;
    /// `None` keeps rows on the moxcms transform.
    rgb_table: Option<Arc<ChannelTable>>,
    /// Exact 256-entry lookup table for 8-bit gray/gray+alpha input; `None`
    /// keeps rows on the moxcms transform.
    gray_table: Option<Arc<GrayTable>>,
    /// Per-image `MemoryBudget` reservation covering `RGB_TABLE_BYTES` or
    /// `GRAY_TABLE_BYTES` whenever `rgb_table` or `gray_table` is active
    /// (whether private or cache-shared) so admission and `peak_managed_bytes`
    /// never depend on cache warmth. Dropped after `rgb_table` and
    /// `gray_table` (field drop order).
    #[allow(dead_code, reason = "held for RAII drop")]
    table_reservation: Option<ExternalReservation>,
}

/// Largest absolute difference, in linear light, that the per-channel table
/// may have from moxcms on any probed 8-bit triplet. Measured worst case over
/// all 2^24 triplets: 1.2e-7 (ADR-0016 and ADR-0023 amendments, 2026-11-12;
/// confirmed by the owner on 2026-11-15, ADR-0016).
const CHANNEL_TABLE_BOUND: f32 = 2.5e-7;

/// 8-bit matrix-shaper conversion as a sum of per-channel contributions
/// (issue #219, ADR-0016 and ADR-0023; confirmed by the owner on 2026-11-15,
/// ADR-0016).
///
/// A matrix-shaper profile into linear sRGB computes `M · [trc_r(r),
/// trc_g(g), trc_b(b)]`: each output is a weighted sum of three independent
/// per-channel curves. With 8-bit input each channel has 256 values, so the
/// transform is the sum of three entries of a 768-entry table. Each entry is
/// padded to `[f32; 4]` with lane 3 = `0.0` so fused RGBA linear writes
/// auto-vectorize cleanly. Every entry is checked finite and bounded at build
/// time so the three-term sum can never overflow or produce NaN.
struct ChannelTable {
    entries: Box<[[f32; 4]; 768]>,
    // Dropped after `entries`; keeps cached table bytes reserved in the batch
    // `IccTableCache` budget until the last `Arc<ChannelTable>` reader drops.
    #[allow(dead_code, reason = "held for RAII drop")]
    reservation: Option<ExternalReservation>,
}

/// Stack chunk size for zero-heap ICC ramp extraction and probe verification.
const TABLE_BUILD_CHUNK: usize = 64;

/// 17-point probe grid per channel (every 16th 8-bit code plus 255).
const CHANNEL_PROBE_GRID: [u8; 17] = [
    0, 16, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240, 255,
];

impl ChannelTable {
    fn verify_grid_chunk(
        transformer: &TransformF32Executor,
        grid_ramps: &[[[f32; 3]; CHANNEL_PROBE_GRID.len()]; 3],
        indices: &[[usize; 3]],
        input: &mut [f32; TABLE_BUILD_CHUNK * 3],
        exact: &mut [f32; TABLE_BUILD_CHUNK * 3],
    ) -> bool {
        let count = indices.len();
        for (i, &[ri, gi, bi]) in indices.iter().enumerate() {
            input[i * 3] = f32::from(CHANNEL_PROBE_GRID[ri]) / 255.0;
            input[i * 3 + 1] = f32::from(CHANNEL_PROBE_GRID[gi]) / 255.0;
            input[i * 3 + 2] = f32::from(CHANNEL_PROBE_GRID[bi]) / 255.0;
        }
        if transformer
            .transform(&input[..count * 3], &mut exact[..count * 3])
            .is_err()
        {
            return false;
        }
        for (&[ri, gi, bi], want) in indices.iter().zip(exact[..count * 3].as_chunks::<3>().0) {
            let r = grid_ramps[0][ri];
            let g = grid_ramps[1][gi];
            let b = grid_ramps[2][bi];
            let got = [r[0] + g[0] + b[0], r[1] + g[1] + b[1], r[2] + g[2] + b[2]];
            if got.iter().zip(want).any(|(got, want)| {
                let d = (got - want).abs();
                d.is_nan() || d > CHANNEL_TABLE_BOUND
            }) {
                return false;
            }
        }
        true
    }

    /// Stack-only additivity and finiteness verification (zero heap
    /// allocations). Runs strictly before reserving [`RGB_TABLE_BYTES`] from
    /// the image's `MemoryBudget`, so non-additive profiles stay uncharged and
    /// behave identically on cold misses and warm negative cache hits.
    fn probe_additive(transformer: &TransformF32Executor) -> bool {
        // Black must contribute nothing, or the 3-term sum would count it thrice.
        let mut black = [0.0_f32; 3];
        if transformer.transform(&[0.0_f32; 3], &mut black).is_err()
            || black.iter().any(|&v| !v.is_finite() || v.abs() > 0.0)
        {
            return false;
        }
        let mut input = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        let mut out_r = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        let mut out_g = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        let mut out_b = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        let mut out_gray = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        let mut grid_ramps = [[[0.0_f32; 3]; CHANNEL_PROBE_GRID.len()]; 3];
        let mut max_abs = [[0.0_f64; 3]; 3];

        // Verify every single-channel code (0..=255), collect overflow bounds
        // and the 17-point basis ramps, and check every gray code [v, v, v].
        for start in (0..256).step_by(TABLE_BUILD_CHUNK) {
            for (channel, out) in [(0_usize, &mut out_r), (1, &mut out_g), (2, &mut out_b)] {
                input.fill(0.0);
                for offset in 0..TABLE_BUILD_CHUNK {
                    let value = u8::try_from(start + offset).unwrap_or(u8::MAX);
                    input[offset * 3 + channel] = f32::from(value) / 255.0;
                }
                if transformer.transform(&input, out).is_err() {
                    return false;
                }
                for (offset, rgb) in out.as_chunks::<3>().0.iter().enumerate() {
                    if rgb.iter().any(|v| !v.is_finite()) {
                        return false;
                    }
                    for (slot, &sample) in max_abs[channel].iter_mut().zip(rgb) {
                        *slot = slot.max(f64::from(sample).abs());
                    }
                    let code = start + offset;
                    if code.is_multiple_of(16) {
                        grid_ramps[channel][code / 16] = *rgb;
                    } else if code == 255 {
                        grid_ramps[channel][CHANNEL_PROBE_GRID.len() - 1] = *rgb;
                    }
                }
            }
            for offset in 0..TABLE_BUILD_CHUNK {
                let v = f32::from(u8::try_from(start + offset).unwrap_or(u8::MAX)) / 255.0;
                input[offset * 3] = v;
                input[offset * 3 + 1] = v;
                input[offset * 3 + 2] = v;
            }
            if transformer.transform(&input, &mut out_gray).is_err() {
                return false;
            }
            for (((r, g), b), want) in out_r
                .as_chunks::<3>()
                .0
                .iter()
                .zip(out_g.as_chunks::<3>().0)
                .zip(out_b.as_chunks::<3>().0)
                .zip(out_gray.as_chunks::<3>().0)
            {
                let got = [r[0] + g[0] + b[0], r[1] + g[1] + b[1], r[2] + g[2] + b[2]];
                if got.iter().zip(want).any(|(got, want)| {
                    let d = (got - want).abs();
                    d.is_nan() || d > CHANNEL_TABLE_BOUND
                }) {
                    return false;
                }
            }
        }
        // Verify that summing any three channel entries can never overflow
        // `f32`, so fused linear writes need no per-pixel finiteness checks.
        for component in 0..3 {
            let max_sum = max_abs[0][component] + max_abs[1][component] + max_abs[2][component];
            if !max_sum.is_finite() || max_sum > f64::from(f32::MAX) * 0.5 {
                return false;
            }
        }
        // Verify the full 17³ grid (4,913 triplets) in 64-pixel stack chunks.
        let mut batch = [[0_usize; 3]; TABLE_BUILD_CHUNK];
        let mut batch_len = 0_usize;
        for ri in 0..CHANNEL_PROBE_GRID.len() {
            for gi in 0..CHANNEL_PROBE_GRID.len() {
                for bi in 0..CHANNEL_PROBE_GRID.len() {
                    batch[batch_len] = [ri, gi, bi];
                    batch_len += 1;
                    if batch_len == TABLE_BUILD_CHUNK {
                        if !Self::verify_grid_chunk(
                            transformer,
                            &grid_ramps,
                            &batch,
                            &mut input,
                            &mut out_gray,
                        ) {
                            return false;
                        }
                        batch_len = 0;
                    }
                }
            }
        }
        batch_len == 0
            || Self::verify_grid_chunk(
                transformer,
                &grid_ramps,
                &batch[..batch_len],
                &mut input,
                &mut out_gray,
            )
    }

    /// Allocate and fill the 768-entry per-channel table for a transform that
    /// already passed [`Self::probe_additive`]. Callers must reserve
    /// [`RGB_TABLE_BYTES`] from the image's `MemoryBudget` before calling.
    fn fill_verified(transformer: &TransformF32Executor) -> Result<Self, Error> {
        let mut raw = crate::memory::try_filled_vec(768, [0.0_f32; 4])?;
        let mut input = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        let mut output = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        for channel in 0..3 {
            for start in (0..256).step_by(TABLE_BUILD_CHUNK) {
                input.fill(0.0);
                for offset in 0..TABLE_BUILD_CHUNK {
                    let value = u8::try_from(start + offset).unwrap_or(u8::MAX);
                    input[offset * 3 + channel] = f32::from(value) / 255.0;
                }
                transformer
                    .transform(&input, &mut output)
                    .map_err(|_| Error::UnsupportedColorEncoding)?;
                for (offset, rgb) in output.as_chunks::<3>().0.iter().enumerate() {
                    if rgb.iter().any(|v| !v.is_finite()) {
                        return Err(Error::UnsupportedColorEncoding);
                    }
                    raw[channel * 256 + start + offset] = [rgb[0], rgb[1], rgb[2], 0.0];
                }
            }
        }
        Ok(Self {
            entries: raw
                .into_boxed_slice()
                .try_into()
                .map_err(|_| Error::AllocationFailed)?,
            reservation: None,
        })
    }

    /// Probe additivity on the stack and, if verified, build the 768-entry
    /// per-channel table.
    #[cfg(test)]
    fn build(transformer: &TransformF32Executor) -> Option<Self> {
        if !Self::probe_additive(transformer) {
            return None;
        }
        Self::fill_verified(transformer).ok()
    }

    #[inline]
    fn apply(&self, [r, g, b]: [u8; 3]) -> [f32; 3] {
        let r = self.entries[usize::from(r)];
        let g = self.entries[256 + usize::from(g)];
        let b = self.entries[512 + usize::from(b)];
        [r[0] + g[0] + b[0], r[1] + g[1] + b[1], r[2] + g[2] + b[2]]
    }

    #[inline]
    fn apply_rgba(&self, [r, g, b]: [u8; 3]) -> [f32; 4] {
        let r = self.entries[usize::from(r)];
        let g = self.entries[256 + usize::from(g)];
        let b = self.entries[512 + usize::from(b)];
        [
            r[0] + g[0] + b[0],
            r[1] + g[1] + b[1],
            r[2] + g[2] + b[2],
            0.0,
        ]
    }
}

/// Exact 256-entry lookup table for 8-bit gray ICC profiles (issue #219).
///
/// Every possible 8-bit gray value (`0..=255`) is evaluated once through
/// `moxcms`, so the table is bit-identical to `moxcms` on every input code
/// with zero approximation error. Entries are padded to `[f32; 4]` with lane
/// 3 = `0.0` for fused linear RGBA writes, and every entry is verified finite
/// at build time.
struct GrayTable {
    entries: Box<[[f32; 4]; 256]>,
    // Dropped after `entries`; keeps cached table bytes reserved in the batch
    // `IccTableCache` budget until the last `Arc<GrayTable>` reader drops.
    #[allow(dead_code, reason = "held for RAII drop")]
    reservation: Option<ExternalReservation>,
}

impl GrayTable {
    /// Stack-only finiteness probe across all 256 8-bit gray codes (zero heap
    /// allocations), so failed-probe profiles stay uncharged and never touch
    /// the image's `MemoryBudget`.
    fn probe_finite(transformer: &TransformF32Executor) -> bool {
        let mut input = [0.0_f32; TABLE_BUILD_CHUNK];
        let mut output = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        for start in (0..256).step_by(TABLE_BUILD_CHUNK) {
            for (offset, dst) in input.iter_mut().enumerate() {
                let value = u8::try_from(start + offset).unwrap_or(u8::MAX);
                *dst = f32::from(value) / 255.0;
            }
            if transformer.transform(&input, &mut output).is_err() {
                return false;
            }
            if output.iter().any(|v| !v.is_finite()) {
                return false;
            }
        }
        true
    }

    /// Build the 256-entry gray table in 64-entry stack chunks (zero temporary
    /// heap probe buffers). Callers must reserve [`GRAY_TABLE_BYTES`] from the
    /// image's `MemoryBudget` before calling.
    fn build(transformer: &TransformF32Executor) -> Result<Self, Error> {
        let mut raw = crate::memory::try_filled_vec(256, [0.0_f32; 4])?;
        let mut input = [0.0_f32; TABLE_BUILD_CHUNK];
        let mut output = [0.0_f32; TABLE_BUILD_CHUNK * 3];
        for start in (0..256).step_by(TABLE_BUILD_CHUNK) {
            for (offset, dst) in input.iter_mut().enumerate() {
                let value = u8::try_from(start + offset).unwrap_or(u8::MAX);
                *dst = f32::from(value) / 255.0;
            }
            transformer
                .transform(&input, &mut output)
                .map_err(|_| Error::UnsupportedColorEncoding)?;
            for (offset, rgb) in output.as_chunks::<3>().0.iter().enumerate() {
                if rgb.iter().any(|v| !v.is_finite()) {
                    return Err(Error::UnsupportedColorEncoding);
                }
                raw[start + offset] = [rgb[0], rgb[1], rgb[2], 0.0];
            }
        }
        Ok(Self {
            entries: raw
                .into_boxed_slice()
                .try_into()
                .map_err(|_| Error::AllocationFailed)?,
            reservation: None,
        })
    }

    #[inline]
    fn apply(&self, value: u8) -> [f32; 3] {
        let c = self.entries[usize::from(value)];
        [c[0], c[1], c[2]]
    }

    #[inline]
    fn apply_rgba(&self, value: u8) -> [f32; 4] {
        self.entries[usize::from(value)]
    }
}

impl RowConversion for IccConversion {
    fn input_sample(&self) -> SampleType {
        self.input
    }

    fn input_channels(&self) -> usize {
        self.channels
    }

    fn scratch_channels(&self) -> (usize, usize) {
        if self.has_fused_table() {
            return (0, 0);
        }
        let scratch = match (self.channels, self.input) {
            (1 | 3, SampleType::F32) => 0,
            (1 | 2, _) => 1,
            (3 | 4, _) => 3,
            _ => self.channels,
        };
        (scratch, self.output_channels)
    }

    fn convert_to_linear_rgba(
        &self,
        row: DecodedRow<'_>,
        linear: &mut [f32],
    ) -> Result<bool, Error> {
        if !self.has_fused_table() {
            return Ok(false);
        }
        let DecodedRow::U8(values) = row else {
            return Err(Error::IncompleteDecoderOutput);
        };
        if !values.len().is_multiple_of(self.channels) {
            return Err(Error::IncompleteDecoderOutput);
        }
        let pixels = values.len() / self.channels;
        if linear.len() != pixels * 4 {
            return Err(Error::IncompleteDecoderOutput);
        }
        match (self.channels, &self.rgb_table, &self.gray_table) {
            (3, Some(table), _) => {
                for (dst, src) in linear
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<3>().0)
                {
                    *dst = table.apply_rgba(*src);
                }
                Ok(true)
            }
            (4, Some(table), _) => {
                for (dst, src) in linear
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<4>().0)
                {
                    let c = table.apply_rgba([src[0], src[1], src[2]]);
                    let alpha = f32::from(src[3]) / 255.0;
                    *dst = [c[0] * alpha, c[1] * alpha, c[2] * alpha, alpha];
                }
                Ok(true)
            }
            (1, _, Some(table)) => {
                for (dst, &value) in linear.as_chunks_mut::<4>().0.iter_mut().zip(values) {
                    *dst = table.apply_rgba(value);
                }
                Ok(true)
            }
            (2, _, Some(table)) => {
                for (dst, src) in linear
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<2>().0)
                {
                    let c = table.apply_rgba(src[0]);
                    let alpha = f32::from(src[1]) / 255.0;
                    *dst = [c[0] * alpha, c[1] * alpha, c[2] * alpha, alpha];
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn convert(
        &self,
        row: DecodedRow<'_>,
        scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        if self.channels < 3 {
            return self.convert_gray(row, scratch, out);
        }
        if row.len() != out.len()
            || row.sample_type() != self.input
            || !row.len().is_multiple_of(self.channels)
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        if self.channels == 4 {
            return self.convert_straight_rgba(row, scratch, out);
        }
        if let (Some(table), DecodedRow::U8(values)) = (&self.rgb_table, row) {
            for (dst, src) in out
                .as_chunks_mut::<3>()
                .0
                .iter_mut()
                .zip(values.as_chunks::<3>().0)
            {
                *dst = table.apply(*src);
            }
            return Ok(());
        }
        let input: &[f32] = match row {
            DecodedRow::U8(values) => {
                let dst = scratch
                    .get_mut(..values.len())
                    .ok_or(Error::IncompleteDecoderOutput)?;
                for (dst, src) in dst.iter_mut().zip(values) {
                    *dst = f32::from(*src) / 255.0;
                }
                dst
            }
            DecodedRow::F32(values) => values,
        };
        self.transformer
            .as_ref()
            .ok_or(Error::UnsupportedColorEncoding)?
            .transform(input, out)
            .map_err(|_| Error::UnsupportedColorEncoding)
    }
}

impl IccConversion {
    fn has_fused_table(&self) -> bool {
        self.input == SampleType::U8
            && match self.channels {
                3 | 4 => self.rgb_table.is_some(),
                1 | 2 => self.gray_table.is_some(),
                _ => false,
            }
    }

    /// Gray (`channels == 1`) or gray plus straight alpha (2) through a gray
    /// profile: 8-bit inputs with a [`GrayTable`] convert directly via the
    /// 256-entry table; float and uncached small inputs pack the gray samples
    /// into `scratch` (or read `F32` gray directly), run `moxcms`, and widen
    /// to RGBA in place when alpha is present.
    fn convert_gray(
        &self,
        row: DecodedRow<'_>,
        scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        let pixels = row.len() / self.channels;
        if !row.len().is_multiple_of(self.channels)
            || row.sample_type() != self.input
            || out.len() != pixels * self.output_channels
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        if let (Some(table), DecodedRow::U8(values)) = (&self.gray_table, row) {
            if self.channels == 1 {
                for (dst, &value) in out.as_chunks_mut::<3>().0.iter_mut().zip(values) {
                    *dst = table.apply(value);
                }
            } else {
                for (dst, src) in out
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<2>().0)
                {
                    let [r, g, b] = table.apply(src[0]);
                    *dst = [r, g, b, f32::from(src[1]) / 255.0];
                }
            }
            return Ok(());
        }
        let gray: &[f32] = match (self.channels, row) {
            (1, DecodedRow::F32(values)) => values,
            (_, DecodedRow::U8(values)) => {
                let dst = scratch
                    .get_mut(..pixels)
                    .ok_or(Error::IncompleteDecoderOutput)?;
                for (dst, src) in dst.iter_mut().zip(values.chunks_exact(self.channels)) {
                    *dst = f32::from(src[0]) / 255.0;
                }
                dst
            }
            (_, DecodedRow::F32(values)) => {
                let dst = scratch
                    .get_mut(..pixels)
                    .ok_or(Error::IncompleteDecoderOutput)?;
                for (dst, src) in dst.iter_mut().zip(values.chunks_exact(self.channels)) {
                    *dst = src[0];
                }
                dst
            }
        };
        self.transformer
            .as_ref()
            .ok_or(Error::UnsupportedColorEncoding)?
            .transform(gray, &mut out[..pixels * 3])
            .map_err(|_| Error::UnsupportedColorEncoding)?;
        if self.channels == 2 {
            for pixel in (0..pixels).rev() {
                let alpha = match row {
                    DecodedRow::U8(values) => f32::from(values[pixel * 2 + 1]) / 255.0,
                    DecodedRow::F32(values) => values[pixel * 2 + 1],
                };
                let rgb = [out[pixel * 3], out[pixel * 3 + 1], out[pixel * 3 + 2]];
                out[pixel * 4..pixel * 4 + 4].copy_from_slice(&[rgb[0], rgb[1], rgb[2], alpha]);
            }
        }
        Ok(())
    }

    /// RGB plus straight alpha: the color channels are converted exactly as
    /// opaque RGB is (the channel table for 8-bit matrix-shaper input,
    /// moxcms otherwise), and the alpha sample is carried over unchanged.
    /// The caller has checked the row length and sample type.
    fn convert_straight_rgba(
        &self,
        row: DecodedRow<'_>,
        scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error> {
        let pixels = row.len() / 4;
        if let (Some(table), DecodedRow::U8(values)) = (&self.rgb_table, row) {
            for (dst, src) in out
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(values.as_chunks::<4>().0)
            {
                let [r, g, b] = table.apply([src[0], src[1], src[2]]);
                *dst = [r, g, b, f32::from(src[3]) / 255.0];
            }
            return Ok(());
        }
        // Pack the color channels into the first three quarters of
        // `scratch`, transform them into the first three quarters of `out`,
        // then widen `out` to RGBA in place from the last pixel backwards:
        // pixel `i` is written at `4i..4i + 4` after it was read at
        // `3i..3i + 3`, and every pixel still to be read lies below `3i`.
        let color = scratch
            .get_mut(..pixels * 3)
            .ok_or(Error::IncompleteDecoderOutput)?;
        match row {
            DecodedRow::U8(values) => {
                for (dst, src) in color
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<4>().0)
                {
                    *dst = [src[0], src[1], src[2]].map(|v| f32::from(v) / 255.0);
                }
            }
            DecodedRow::F32(values) => {
                for (dst, src) in color
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(values.as_chunks::<4>().0)
                {
                    *dst = [src[0], src[1], src[2]];
                }
            }
        }
        self.transformer
            .as_ref()
            .ok_or(Error::UnsupportedColorEncoding)?
            .transform(color, &mut out[..pixels * 3])
            .map_err(|_| Error::UnsupportedColorEncoding)?;
        for pixel in (0..pixels).rev() {
            let alpha = match row {
                DecodedRow::U8(values) => f32::from(values[pixel * 4 + 3]) / 255.0,
                DecodedRow::F32(values) => values[pixel * 4 + 3],
            };
            let rgb = [out[pixel * 3], out[pixel * 3 + 1], out[pixel * 3 + 2]];
            out[pixel * 4..pixel * 4 + 4].copy_from_slice(&[rgb[0], rgb[1], rgb[2], alpha]);
        }
        Ok(())
    }
}

impl<'a> IccSrgbSource<'a> {
    #[cfg(test)]
    pub(crate) fn new(source: Box<dyn DecodeSource + 'a>) -> Result<Self, Error> {
        let mut this = Self::new_deferred(source, None)?;
        this.prepare_conversion_inner(None, None)?;
        Ok(this)
    }

    #[cfg(test)]
    pub(crate) fn new_with_cache(
        source: Box<dyn DecodeSource + 'a>,
        cache: Option<&'a IccTableCache>,
        expected_pixels: u64,
        budget: Option<&MemoryBudget>,
    ) -> Result<Self, Error> {
        let mut this = Self::new_deferred(source, cache)?;
        this.prepare_conversion_inner(Some(expected_pixels), budget)?;
        Ok(this)
    }

    #[cfg(test)]
    fn conversion(&self) -> &Arc<IccConversion> {
        match &self.state {
            IccState::Ready(conversion) => conversion,
            IccState::Pending(_) => {
                panic!("IccSrgbSource::conversion called before prepare_conversion")
            }
        }
    }

    /// Validate the rendered ICC profile, color space, and any CICP signal
    /// without reserving or building lookup tables yet. Lookup-table probing
    /// and per-image `MemoryBudget` admission run later in
    /// [`DecodeSource::prepare_conversion`] after all request, crop, depth,
    /// and encoder validations have passed.
    pub(crate) fn new_deferred(
        source: Box<dyn DecodeSource + 'a>,
        cache: Option<&'a IccTableCache>,
    ) -> Result<Self, Error> {
        let info = source.info();
        // Opaque RGB, or RGB with straight alpha whose sample bypasses the
        // transform (issue #222). Premultiplied color would have to be
        // divided by alpha before a nonlinear profile curve, and unspecified
        // alpha has no defined meaning; neither is guessed (ADR-0015).
        let (channels, output_channels) = match (info.layout, info.alpha) {
            (ChannelLayout::Rgb, AlphaMode::Opaque) => (3, 3),
            (ChannelLayout::Rgba, AlphaMode::Straight) => (4, 4),
            (ChannelLayout::Gray, AlphaMode::Opaque) => (1, 3),
            (ChannelLayout::GrayAlpha, AlphaMode::Straight) => (2, 4),
            (ChannelLayout::Rgba | ChannelLayout::GrayAlpha, _) => {
                return Err(Error::UnsupportedAlphaMode);
            }
            _ => return Err(Error::UnsupportedLayout),
        };
        let gray = channels < 3;
        // Without a profile this policy has nothing to convert. A PQ or HLG
        // CICP is named as such: it is refused under every policy.
        let bytes = info.rendered_icc.as_deref().ok_or(
            if info
                .rendered_cicp
                .is_some_and(|cicp| matches!(cicp[1], 16 | 18))
            {
                Error::HdrTransfer
            } else {
                Error::RenderedIccMissing
            },
        )?;
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(Error::UnsupportedColorEncoding);
        }
        let source_pixels = u64::from(info.size.width).saturating_mul(u64::from(info.size.height));
        let input = info.sample;
        let cacheable = cache.is_some_and(|c| c.max_bytes > 0 && bytes.len() <= c.max_bytes)
            && bytes.len() <= MAX_CACHED_PROFILE_BYTES;
        let hash = if cacheable {
            hash_profile_bytes(bytes)
        } else {
            0
        };

        let outcome = if let (Some(cache), true) = (cache, cacheable)
            && let Some(cached) = cache.get(hash, bytes)
        {
            if cached.gray != gray {
                return Err(Error::IccColorSpaceMismatch);
            }
            let cicp_transformer = if let Some(signal) = info.rendered_cicp {
                if gray {
                    return Err(Error::CicpBesideIcc(
                        "cannot be checked against a gray profile",
                    ));
                }
                let (transformer, _) = build_moxcms_transform(bytes, false, Some(signal))?;
                Some(transformer)
            } else {
                None
            };
            ValidatedProfile::CacheHit {
                table: cached.table,
                cicp_transformer,
            }
        } else {
            let (transformer, extended_matrix_shaper) =
                build_moxcms_transform(bytes, gray, info.rendered_cicp)?;
            ValidatedProfile::Miss {
                transformer,
                extended_matrix_shaper,
            }
        };

        let mut output = info.clone();
        output.layout = if output_channels == 4 {
            ChannelLayout::Rgba
        } else {
            ChannelLayout::Rgb
        };
        output.sample = SampleType::F32;
        output.rendered_icc = None;
        output.rendered_cicp = Some([1, 8, 0, 1]);
        Ok(Self {
            source,
            output,
            state: IccState::Pending(Box::new(PendingIcc {
                cache,
                hash,
                cacheable,
                gray,
                channels,
                output_channels,
                input,
                source_pixels,
                outcome,
            })),
            deferred: false,
        })
    }

    #[allow(clippy::too_many_lines)] // Cache-hit and stack-probe-before-budget-reserve miss paths stay side-by-side.
    fn prepare_conversion_inner(
        &mut self,
        expected_pixels: Option<u64>,
        budget: Option<&MemoryBudget>,
    ) -> Result<(), Error> {
        let IccState::Pending(pending) = &self.state else {
            return Ok(());
        };
        let info = self.source.info();
        let bytes = info
            .rendered_icc
            .as_deref()
            .ok_or(Error::RenderedIccMissing)?;
        // RGB `ChannelTable` selection depends ONLY on the full pre-crop
        // source dimensions (never crop size or cache warmth) so overlapping
        // crops stay byte-identical (ADR-0020). Gray `GrayTable` is
        // bit-identical to `moxcms` on all 256 codes, so its cutoff uses the
        // validated resolved crop pixel count identically on hits and misses.
        let allow_rgb_table = pending.input == SampleType::U8
            && !pending.gray
            && expected_pixels
                .is_none_or(|px| px > 0 && pending.source_pixels >= RGB_TABLE_MIN_PIXELS);
        let allow_gray_table = pending.input == SampleType::U8
            && pending.gray
            && expected_pixels.is_none_or(|px| px >= GRAY_TABLE_MIN_PIXELS);

        let (transformer, rgb_table, gray_table, table_reservation) = match &pending.outcome {
            ValidatedProfile::CacheHit {
                table: CachedTable::Rgb(Some(table)),
                ..
            } if allow_rgb_table => {
                let rgb_res = budget
                    .map(|b| b.reserve_external(RGB_TABLE_BYTES))
                    .transpose()?;
                (None, Some(Arc::clone(table)), None, rgb_res)
            }
            ValidatedProfile::CacheHit {
                table: CachedTable::Gray(Some(table)),
                ..
            } if allow_gray_table => {
                let gray_res = budget
                    .map(|b| b.reserve_external(GRAY_TABLE_BYTES))
                    .transpose()?;
                (None, None, Some(Arc::clone(table)), gray_res)
            }
            ValidatedProfile::CacheHit {
                cicp_transformer, ..
            } => {
                let transformer = if let Some(transformer) = cicp_transformer {
                    Arc::clone(transformer)
                } else {
                    build_moxcms_transform(bytes, pending.gray, info.rendered_cicp)?.0
                };
                (Some(transformer), None, None, None)
            }
            ValidatedProfile::Miss {
                transformer,
                extended_matrix_shaper,
            } => {
                if pending.gray {
                    if allow_gray_table {
                        if GrayTable::probe_finite(transformer.as_ref()) {
                            let gray_res = budget
                                .map(|b| b.reserve_external(GRAY_TABLE_BYTES))
                                .transpose()?;
                            let built = match GrayTable::build(transformer.as_ref()) {
                                Ok(table) => table,
                                Err(err) => {
                                    drop(gray_res);
                                    return Err(err);
                                }
                            };
                            let table =
                                if let (Some(cache), true) = (pending.cache, pending.cacheable) {
                                    cache.insert_gray(pending.hash, bytes, built)
                                } else {
                                    Arc::new(built)
                                };
                            (None, None, Some(table), gray_res)
                        } else {
                            if let (Some(cache), true) = (pending.cache, pending.cacheable) {
                                cache.insert_negative_gray(pending.hash, bytes);
                            }
                            (Some(Arc::clone(transformer)), None, None, None)
                        }
                    } else {
                        (Some(Arc::clone(transformer)), None, None, None)
                    }
                } else if !*extended_matrix_shaper {
                    // Ineligible profiles (e.g. LUT-based) never probe
                    // `ChannelTable` on a miss, so caching a negative entry
                    // saves no work and would waste budget or evict tables.
                    (Some(Arc::clone(transformer)), None, None, None)
                } else if allow_rgb_table {
                    // Verify additivity strictly on the stack (zero heap)
                    // before reserving `RGB_TABLE_BYTES`, so non-additive
                    // matrix-shaper profiles stay uncharged and match warm
                    // negative cache hits identically.
                    if ChannelTable::probe_additive(transformer.as_ref()) {
                        let rgb_res = budget
                            .map(|b| b.reserve_external(RGB_TABLE_BYTES))
                            .transpose()?;
                        let built = match ChannelTable::fill_verified(transformer.as_ref()) {
                            Ok(table) => table,
                            Err(err) => {
                                drop(rgb_res);
                                return Err(err);
                            }
                        };
                        let table = if let (Some(cache), true) = (pending.cache, pending.cacheable)
                        {
                            cache.insert_rgb(pending.hash, bytes, built)
                        } else {
                            Arc::new(built)
                        };
                        (None, Some(table), None, rgb_res)
                    } else {
                        if let (Some(cache), true) = (pending.cache, pending.cacheable) {
                            cache.insert_negative_rgb(pending.hash, bytes);
                        }
                        (Some(Arc::clone(transformer)), None, None, None)
                    }
                } else {
                    (Some(Arc::clone(transformer)), None, None, None)
                }
            }
        };
        let conversion = Arc::new(IccConversion {
            transformer,
            input: pending.input,
            channels: pending.channels,
            output_channels: pending.output_channels,
            rgb_table,
            gray_table,
            table_reservation,
        });
        self.state = IccState::Ready(conversion);
        Ok(())
    }
}

/// Validate an ICC profile header, parse it with `moxcms`, construct the
/// relative-colorimetric transform into linear-light sRGB, and verify any
/// decoder-supplied or profile-tag CICP signal (ADR-0015, ADR-0016).
fn build_moxcms_transform(
    bytes: &[u8],
    gray: bool,
    decoder_cicp: Option<[u8; 4]>,
) -> Result<(Arc<TransformF32Executor>, bool), Error> {
    // The same bounded header check as the color contract (ADR-0015):
    // moxcms 0.9.1 accepts a header whose declared size exceeds the data.
    crate::color_contract::IccHeader::inspect(bytes)
        .map_err(|_| Error::UnsupportedColorEncoding)?;
    let src = ColorProfile::new_from_slice(bytes).map_err(|_| Error::UnsupportedColorEncoding)?;
    // The profile describes the decoded channels or it is not used: a
    // gray profile is not an RGB profile, and the reverse.
    match (gray, src.color_space) {
        (true, DataColorSpace::Gray) | (false, DataColorSpace::Rgb) => {}
        _ => return Err(Error::IccColorSpaceMismatch),
    }
    // Linear-light sRGB: the working space of the resample chain, so
    // the executor neither re-encodes nor re-decodes the transfer curve
    // (an empty LUT is the identity curve, as in moxcms' linear ACEScg).
    let mut dst = ColorProfile::new_srgb();
    let linear = ToneReprCurve::Lut(Vec::new());
    dst.red_trc = Some(linear.clone());
    dst.green_trc = Some(linear.clone());
    dst.blue_trc = Some(linear);
    dst.cicp = None;
    // moxcms' extended floating-point path avoids the destination-gamut
    // clamp for matrix-shaper profiles; LUT-based profiles keep moxcms'
    // LUT path.
    let no_luts = src.lut_a_to_b_perceptual.is_none()
        && src.lut_a_to_b_colorimetric.is_none()
        && src.lut_a_to_b_saturation.is_none();
    let extended_matrix_shaper =
        src.pcs == DataColorSpace::Xyz && src.is_matrix_shaper() && no_luts;
    // A gray tone-curve profile takes moxcms' extended gray path (no
    // clamp), as a matrix-shaper RGB profile takes its extended path.
    let extended_gray = gray && src.pcs == DataColorSpace::Xyz && src.gray_trc.is_some() && no_luts;
    let options = TransformOptions {
        // Relative colorimetric, set here and never taken from the
        // library default (perceptual) or the profile header (owner
        // decision 2026-11-15, ADR-0016 amendment of that date). It
        // selects a LUT profile's colorimetric table (A2B1) and, in
        // moxcms 0.9.1, no black-point scaling; matrix/TRC profiles do
        // not read the intent, so their output is unchanged.
        rendering_intent: RenderingIntent::RelativeColorimetric,
        allow_extended_range_rgb_xyz: extended_matrix_shaper || extended_gray,
        ..TransformOptions::default()
    };
    let layout = if gray { Layout::Gray } else { Layout::Rgb };
    let transformer = src
        .create_transform_f32(layout, &dst, Layout::Rgb, options)
        .map_err(|_| Error::UnsupportedColorEncoding)?;
    check_cicp_beside(decoder_cicp, &src, transformer.as_ref(), gray)?;
    Ok((transformer, extended_matrix_shaper))
}

/// Decoder-reported CICP evidence beside the rendered profile.
fn check_decoder_cicp(
    signal: [u8; 4],
    transformer: &TransformF32Executor,
    gray: bool,
) -> Result<(), Error> {
    // The probes are RGB colors; a gray profile cannot be checked against an
    // RGB code point (no built-in reader reports one).
    if gray {
        return Err(Error::CicpBesideIcc(
            "cannot be checked against a gray profile",
        ));
    }
    check_cicp_agrees(signal, transformer)
}

/// CICP evidence beside the rendered profile: the decoder's own signal and
/// the profile's `cicp` tag (issues #17 and #41; see `check_cicp_agrees`).
fn check_cicp_beside(
    decoder_cicp: Option<[u8; 4]>,
    src: &ColorProfile,
    transformer: &TransformF32Executor,
    gray: bool,
) -> Result<(), Error> {
    // CICP evidence beside the profile must agree with what the profile
    // does (issues #17 and #41). The decoder's own CICP is independent
    // evidence and is always checked. The profile's `cicp` tag is checked
    // in full only when it is a well-formed RGB tag (matrix 0, full
    // range); profiles in the wild carry miscoded tags (moxcms 0.9.1's
    // own Display P3 profile writes `[11, 13, 1, 0]`), so a malformed tag
    // is not trusted as a description, but its PQ/HLG transfer is still
    // honored as a reason to reject.
    if let Some(signal) = decoder_cicp {
        check_decoder_cicp(signal, transformer, gray)?;
    }
    if let Some(cicp) = src.cicp.as_ref() {
        let tag = [
            cicp.color_primaries as u8,
            cicp.transfer_characteristics as u8,
            cicp.matrix_coefficients as u8,
            u8::from(cicp.full_range),
        ];
        if tag[2] == 0 && tag[3] == 1 && !gray {
            check_cicp_agrees(tag, transformer)?;
        } else if matches!(tag[1], 16 | 18) {
            return Err(Error::CicpBesideIcc(HDR));
        }
    }
    Ok(())
}

/// Fail closed unless a CICP signal next to the rendered ICC is an SDR
/// family the engine can evaluate itself and the profile transform agrees
/// with it (issues #17 and #41).
///
/// The rendered-ICC policy makes the profile authoritative for the
/// conversion, but it must not silently override contradicting evidence:
/// - PQ and HLG need reference-white and tone-mapping decisions (ADR-0016
///   rule 7). An ICC approximation of those curves, such as the profiles
///   JPEG XL decoders synthesize for PQ/HLG, would be converted as if it were
///   SDR, so they are rejected.
/// - Code points outside sRGB, linear sRGB and Display P3 cannot be checked
///   here and are rejected rather than ignored.
/// - For the supported families, probe colors through the profile must match
///   the engine's own CICP reference within `2e-3` linear (the moxcms versus
///   reference P3 matrix bound of ADR-0023); otherwise the two signals
///   contradict each other.
///
/// MIAF gives an AVIF ICC `colr` precedence over `nclx` for color
/// interpretation; the engine does not apply that precedence silently. Each
/// refusal is a `CicpBesideIcc` that names the reason (review of 2026-11-12;
/// the outcome for SDR transfers outside these families, such as BT.709
/// transfer 1 beside an sRGB profile, awaits owner confirmation in ADR-0015).
fn check_cicp_agrees(raw: [u8; 4], transformer: &TransformF32Executor) -> Result<(), Error> {
    use crate::color::ColorEncoding;
    use crate::color_contract::{Cicp, CicpFamily};
    use crate::working_color::decode_to_linear_srgb;
    const PROBES: [[f32; 3]; 7] = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.5, 0.5, 0.5],
        [0.04, 0.04, 0.04],
        [0.2, 0.6, 0.9],
    ];
    if matches!(raw[1], 16 | 18) {
        return Err(Error::CicpBesideIcc(HDR));
    }
    let cicp = Cicp::try_from(raw)
        .map_err(|_| Error::CicpBesideIcc("is not a valid full-range RGB code point"))?;
    if !matches!(
        cicp.family(),
        CicpFamily::Srgb | CicpFamily::LinearSrgb | CicpFamily::DisplayP3
    ) {
        return Err(Error::CicpBesideIcc(
            "is outside sRGB, linear sRGB and Display P3, so it cannot be checked against the profile",
        ));
    }
    let input: Vec<f32> = PROBES.iter().flatten().copied().collect();
    let mut output = vec![0.0_f32; input.len()];
    transformer
        .transform(&input, &mut output)
        .map_err(|_| Error::UnsupportedColorEncoding)?;
    for (probe, actual) in PROBES.iter().zip(output.as_chunks::<3>().0) {
        let expected = decode_to_linear_srgb(&ColorEncoding::Cicp(raw), probe.map(f64::from))?;
        for (want, got) in expected.rgb.iter().zip(actual) {
            if (want - f64::from(*got)).abs() > 2e-3 {
                return Err(Error::CicpBesideIcc("disagrees with the profile transform"));
            }
        }
    }
    Ok(())
}

/// The refusal reason for a PQ or HLG transfer beside the profile.
const HDR: &str = "signals a PQ or HLG transfer, which needs tone mapping (ADR-0016 rule 7)";

impl DecodeSource for IccSrgbSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.output
    }
    fn set_orientation_threads(&mut self, threads: usize) {
        self.source.set_orientation_threads(threads);
    }
    fn has_transposing_orientation(&self) -> bool {
        self.source.has_transposing_orientation()
    }
    fn streams_rows(&self) -> bool {
        self.source.streams_rows()
    }
    fn prepare_conversion(
        &mut self,
        expected_pixels: u64,
        budget: &MemoryBudget,
    ) -> Result<(), Error> {
        self.prepare_conversion_inner(Some(expected_pixels), Some(budget))
    }
    fn defer_conversion(&mut self) -> Option<Arc<dyn RowConversion>> {
        let IccState::Ready(conversion) = &self.state else {
            return None;
        };
        self.deferred = true;
        Some(conversion.clone())
    }
    fn set_downstream_headroom(&mut self, bytes: usize) {
        self.source.set_downstream_headroom(bytes);
    }
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        if matches!(self.state, IccState::Pending(_)) {
            let pixels = u64::from(region.width).saturating_mul(u64::from(region.height));
            self.prepare_conversion(pixels, budget)?;
        }
        let IccState::Ready(conversion) = &self.state else {
            return Err(Error::UnsupportedColorEncoding);
        };
        let channels = conversion.output_channels;
        let width = usize::try_from(region.width).map_err(|_| Error::GeometryOverflow)?;
        let input_len = width
            .checked_mul(conversion.channels)
            .ok_or(Error::GeometryOverflow)?;
        let len = width.checked_mul(channels).ok_or(Error::GeometryOverflow)?;
        let expected_sample = self.source.info().sample;
        let mut report = if self.deferred {
            self.source.read_region(region, budget, &mut |y, row| {
                if row.len() != input_len || row.sample_type() != expected_sample {
                    return Err(Error::IncompleteDecoderOutput);
                }
                sink(y, row)
            })?
        } else {
            let (scratch_ch, _) = conversion.scratch_channels();
            let scratch_len = width
                .checked_mul(scratch_ch)
                .ok_or(Error::GeometryOverflow)?;
            let mut scratch = budget.allocate_typed::<f32>(scratch_len)?;
            let mut output = budget.allocate_typed::<f32>(len)?;
            let conversion = Arc::clone(conversion);
            self.source.read_region(region, budget, &mut |y, row| {
                conversion.convert(row, scratch.as_mut_slice(), output.as_mut_slice())?;
                sink(y, DecodedRow::F32(output.as_slice()))
            })?
        };
        let pixel_bytes = u64::try_from(4 * channels).map_err(|_| Error::GeometryOverflow)?;
        report.output_sample_bytes = report
            .rows
            .checked_mul(u64::from(region.width))
            .and_then(|n| n.checked_mul(pixel_bytes))
            .ok_or(Error::GeometryOverflow)?;
        report.peak_engine_managed_bytes = budget.snapshot().peak;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::IccSrgbSource;
    use crate::color::{AlphaMode, ColorEncoding};
    use crate::decoder::RowConversion;
    use crate::decoder::{
        ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo,
        SampleType,
    };
    use crate::error::Error;
    use crate::geometry::{Rect, Size};
    use crate::memory::MemoryBudget;
    use crate::working_color::decode_to_linear_srgb;
    use moxcms::{ColorProfile, Layout, RenderingIntent, ToneReprCurve, TransformOptions};

    /// One row holding every value of a coarse 8-bit RGB grid.
    #[derive(Clone)]
    struct Grid {
        info: DecoderInfo,
        row: Vec<u8>,
    }

    impl DecodeSource for Grid {
        fn info(&self) -> &DecoderInfo {
            &self.info
        }
        fn read_region(
            &mut self,
            region: Rect,
            _budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            sink(0, DecodedRow::U8(&self.row))?;
            Ok(DecodeReport {
                requested: region,
                access: DecodeAccess::CodecRegionRequest,
                rows: 1,
                output_sample_bytes: 0,
                known_full_frame_buffers: 0,
                peak_engine_managed_bytes: 0,
                opaque_codec_memory_unbounded: false,
                engine_buffer_bytes: 0,
                engine_copied_bytes: 0,
                work: crate::decoder::CodecWork::region_at_least(region),
            })
        }
    }

    struct FloatRow {
        info: DecoderInfo,
        row: Vec<f32>,
    }

    impl DecodeSource for FloatRow {
        fn info(&self) -> &DecoderInfo {
            &self.info
        }
        fn read_region(
            &mut self,
            region: Rect,
            _budget: &MemoryBudget,
            sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            sink(0, DecodedRow::F32(&self.row))?;
            Ok(DecodeReport {
                requested: region,
                access: DecodeAccess::CodecRegionRequest,
                rows: 1,
                output_sample_bytes: 0,
                known_full_frame_buffers: 0,
                peak_engine_managed_bytes: 0,
                opaque_codec_memory_unbounded: false,
                engine_buffer_bytes: 0,
                engine_copied_bytes: 0,
                work: crate::decoder::CodecWork::region_at_least(region),
            })
        }
    }

    fn signalled(icc: Vec<u8>, cicp: Option<[u8; 4]>) -> Result<(), Error> {
        let mut grid = p3_grid(vec![10, 20, 30]);
        grid.info.rendered_icc = Some(icc);
        grid.info.rendered_cicp = cicp;
        IccSrgbSource::new(Box::new(grid)).map(|_| ())
    }

    /// Issues #17 and #41: CICP beside the rendered ICC is never silently
    /// overridden. Agreeing SDR evidence passes; contradicting, unknown and
    /// HDR evidence is a controlled `CicpBesideIcc` naming the reason.
    #[test]
    fn cicp_beside_the_rendered_icc_must_agree() {
        let srgb = moxcms::ColorProfile::new_srgb().encode().unwrap();
        let p3 = moxcms::ColorProfile::new_display_p3().encode().unwrap();
        assert_eq!(signalled(srgb.clone(), Some([1, 13, 0, 1])), Ok(()));
        assert_eq!(signalled(p3.clone(), Some([12, 13, 0, 1])), Ok(()));
        // moxcms's own P3 profile carries a miscoded `[11, 13, 1, 0]` tag; a
        // malformed RGB tag is not trusted as a description.
        assert_eq!(signalled(p3.clone(), None), Ok(()));
        for (icc, cicp) in [
            (p3.clone(), [1, 13, 0, 1]),
            (srgb.clone(), [12, 13, 0, 1]),
            (srgb.clone(), [1, 8, 0, 1]),
            (srgb.clone(), [9, 16, 0, 1]),
            (srgb.clone(), [9, 18, 0, 1]),
            (srgb.clone(), [11, 13, 0, 1]),
            (srgb.clone(), [1, 13, 0, 0]),
            (srgb.clone(), [1, 13, 0, 7]),
        ] {
            assert!(
                matches!(signalled(icc, Some(cicp)), Err(Error::CicpBesideIcc(_))),
                "{cicp:?}"
            );
        }
        // A PQ profile is rejected by its own tag, even without decoder CICP.
        let pq = moxcms::ColorProfile::new_display_p3_pq().encode().unwrap();
        assert_eq!(signalled(pq, None), Err(Error::CicpBesideIcc(super::HDR)));
    }

    /// Hostile profiles fail with a controlled error, never a panic.
    #[test]
    fn malformed_and_hostile_profiles_are_controlled_errors() {
        let srgb = moxcms::ColorProfile::new_srgb().encode().unwrap();
        let mut cases: Vec<Vec<u8>> = vec![
            Vec::new(),
            vec![0; 131],
            srgb[..200].to_vec(),
            vec![0xff; 4096],
        ];
        // Declared size far beyond the data.
        let mut oversized = srgb.clone();
        oversized[0..4].copy_from_slice(&u32::MAX.to_be_bytes());
        cases.push(oversized);
        // A tag count that overflows the table.
        let mut tags = srgb.clone();
        tags[128..132].copy_from_slice(&u32::MAX.to_be_bytes());
        cases.push(tags);
        // Every tag offset pointing past the end.
        let mut offsets = srgb.clone();
        let count = u32::from_be_bytes(offsets[128..132].try_into().unwrap()) as usize;
        for i in 0..count {
            let at = 132 + 12 * i + 4;
            offsets[at..at + 4].copy_from_slice(&(u32::MAX - 8).to_be_bytes());
        }
        cases.push(offsets);
        // Larger than the 4 MiB cap.
        cases.push(vec![0; 4 * 1024 * 1024 + 1]);
        for (i, icc) in cases.into_iter().enumerate() {
            assert_eq!(
                signalled(icc, None),
                Err(Error::UnsupportedColorEncoding),
                "case {i}"
            );
        }
    }

    fn p3_grid(row: Vec<u8>) -> Grid {
        Grid {
            info: DecoderInfo {
                depth: 8,
                size: Size::new(u32::try_from(row.len() / 3).unwrap(), 1).unwrap(),
                layout: ChannelLayout::Rgb,
                sample: SampleType::U8,
                access: DecodeAccess::CodecRegionRequest,
                alpha: AlphaMode::Opaque,
                rendered_icc: Some(moxcms::ColorProfile::new_display_p3().encode().unwrap()),
                rendered_cicp: None,
            },
            row,
        }
    }

    /// Deterministic hostile-profile corpus derived from a real v4 profile:
    /// truncations, every header and tag-table word forced to 0 and
    /// `u32::MAX`, wrong data/PCS color spaces, and seeded byte flips. Each
    /// profile must either build a transform that converts a row without
    /// panicking or be rejected as unsupported color.
    #[test]
    fn hostile_icc_profiles_are_rejected_or_converted_without_panicking() {
        use std::panic::{AssertUnwindSafe, catch_unwind};

        let valid = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let tag_table_end = 132 + 12 * usize::from(valid[131]);
        let mut corpus: Vec<(String, Vec<u8>)> = Vec::new();
        for length in [0, 1, 4, 127, 128, 131, 132, 143, 144, tag_table_end, 400] {
            corpus.push((format!("truncated-{length}"), valid[..length].to_vec()));
        }
        for offset in (0..tag_table_end).step_by(4) {
            for word in [[0; 4], [0xff; 4]] {
                let mut profile = valid.clone();
                profile[offset..offset + 4].copy_from_slice(&word);
                corpus.push((format!("word-{offset}-{:02x}", word[0]), profile));
            }
        }
        for (offset, value) in [
            (16, *b"GRAY"),
            (16, *b"CMYK"),
            (16, *b"Lab "),
            (20, *b"Lab "),
            (20, *b"RGB "),
        ] {
            let mut profile = valid.clone();
            profile[offset..offset + 4].copy_from_slice(&value);
            corpus.push((format!("space-{offset}-{value:?}"), profile));
        }
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        for case in 0..256 {
            let mut profile = valid.clone();
            for _ in 0..4 {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let index = usize::try_from(state >> 33).unwrap() % profile.len();
                profile[index] ^= u8::try_from(state >> 56).unwrap() | 1;
            }
            corpus.push((format!("flip-{case}"), profile));
        }

        let mut accepted = 0;
        for (name, profile) in corpus {
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                let mut grid = p3_grid(vec![0, 64, 128, 255, 255, 255, 12, 200, 7]);
                grid.info.rendered_icc = Some(profile);
                let mut source = IccSrgbSource::new(Box::new(grid))?;
                source.read_region(
                    Rect::new(0, 0, 3, 1)?,
                    &MemoryBudget::new(1 << 20),
                    &mut |_, _| Ok(()),
                )?;
                Ok::<_, Error>(())
            }))
            .unwrap_or_else(|_| panic!("ICC profile {name} panicked"));
            match outcome {
                Ok(()) => accepted += 1,
                // A header that names another data color space (GRAY, CMYK,
                // Lab) is the named mismatch since 2026-11-15; every other
                // rejection stays unsupported color.
                Err(Error::IccColorSpaceMismatch) => {
                    assert!(name.starts_with("space-16-"), "{name}");
                }
                Err(error) => assert_eq!(error, Error::UnsupportedColorEncoding, "{name}"),
            }
        }
        // The unchanged tag-table words and some flips keep a usable profile.
        assert!(accepted > 0);
    }

    fn read_rows(source: &mut IccSrgbSource<'_>, width: u32) -> Vec<Vec<f32>> {
        let mut rows = Vec::new();
        source
            .read_region(
                Rect::new(0, 0, width, 1).unwrap(),
                &MemoryBudget::new(1 << 20),
                &mut |_, row| {
                    rows.push(match row {
                        DecodedRow::F32(values) => values.to_vec(),
                        DecodedRow::U8(values) => values.iter().map(|&v| f32::from(v)).collect(),
                    });
                    Ok(())
                },
            )
            .unwrap();
        rows
    }

    #[test]
    fn deferred_conversion_reproduces_the_adapter_output_exactly() {
        let row: Vec<u8> = (0..=255_u8).flat_map(|v| [v, 255 - v, v / 2]).collect();
        let width = u32::try_from(row.len() / 3).unwrap();
        let converted = read_rows(
            &mut IccSrgbSource::new(Box::new(p3_grid(row.clone()))).unwrap(),
            width,
        );

        let mut source = IccSrgbSource::new(Box::new(p3_grid(row.clone()))).unwrap();
        let conversion = source.defer_conversion().expect("the ICC adapter defers");
        assert_eq!(conversion.input_sample(), SampleType::U8);
        // The info still describes converted rows; the rows pass through.
        assert_eq!(source.info().sample, SampleType::F32);
        let raw = read_rows(&mut source, width);
        let expected_raw: Vec<f32> = row.iter().map(|&v| f32::from(v)).collect();
        assert_eq!(raw, vec![expected_raw]);

        let mut scratch = vec![0.0; row.len()];
        let mut out = vec![0.0; row.len()];
        conversion
            .convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(
            out.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            converted[0].iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        // A row of the wrong type or length is rejected.
        assert_eq!(
            conversion.convert(DecodedRow::F32(&out.clone()), &mut scratch, &mut out),
            Err(Error::IncompleteDecoderOutput)
        );
        assert_eq!(
            conversion.convert(DecodedRow::U8(&row[3..]), &mut scratch, &mut out),
            Err(Error::IncompleteDecoderOutput)
        );
    }

    #[test]
    fn display_p3_icc_becomes_linear_srgb_matching_the_reference_matrix() {
        let icc = moxcms::ColorProfile::new_display_p3().encode().unwrap();
        let steps: Vec<u8> = (0..=255).step_by(17).collect();
        let mut row = Vec::new();
        for &r in &steps {
            for &g in &steps {
                for &b in &steps {
                    row.extend([r, g, b]);
                }
            }
        }
        let width = u32::try_from(row.len() / 3).unwrap();
        let grid = Grid {
            info: DecoderInfo {
                depth: 8,
                size: Size::new(width, 1).unwrap(),
                layout: ChannelLayout::Rgb,
                sample: SampleType::U8,
                access: DecodeAccess::CodecRegionRequest,
                alpha: AlphaMode::Opaque,
                rendered_icc: Some(icc),
                rendered_cicp: None,
            },
            row: row.clone(),
        };
        let mut source = IccSrgbSource::new(Box::new(grid)).unwrap();
        // The adapter reports what it emits: linear-light sRGB.
        assert_eq!(source.info().rendered_cicp, Some([1, 8, 0, 1]));
        assert_eq!(source.info().rendered_icc, None);
        let mut worst = 0.0_f64;
        let mut worst_extended = 0.0_f64;
        let mut extended = 0_usize;
        source
            .read_region(
                Rect::new(0, 0, width, 1).unwrap(),
                &MemoryBudget::new(1 << 20),
                &mut |_, out| {
                    let DecodedRow::F32(out) = out else {
                        panic!("the adapter emits f32");
                    };
                    for (pixel, linear) in row.as_chunks::<3>().0.iter().zip(out.as_chunks::<3>().0)
                    {
                        let reference = decode_to_linear_srgb(
                            &ColorEncoding::Cicp([12, 13, 0, 1]),
                            pixel.map(|v| f64::from(v) / 255.0),
                        )
                        .unwrap();
                        let in_gamut = reference.rgb.iter().all(|v| (0.0..=1.0).contains(v));
                        for (expected, &actual) in reference.rgb.iter().zip(linear) {
                            let difference = (expected - f64::from(actual)).abs();
                            if in_gamut {
                                worst = worst.max(difference);
                            } else {
                                worst_extended = worst_extended.max(difference);
                                assert!(difference < 2.0e-3, "{expected} vs {actual}");
                            }
                        }
                        extended += usize::from(!in_gamut);
                    }
                    Ok(())
                },
            )
            .unwrap();
        // moxcms (Q1.30 matrix shaper) against the CSS Color 4 matrices:
        // a few 1e-4 in linear light is below one 8-bit sRGB code at the
        // bright end and far below it in the dark range, where the linear
        // segment dominates.
        assert!(worst < 2.0e-3, "worst in-gamut linear difference {worst}");
        assert!(extended > 0, "grid must include out-of-sRGB-gamut pixels");
        eprintln!(
            "display P3 ICC via moxcms vs reference: in gamut worst {worst:e}; {extended} out-of-gamut pixels retained, worst {worst_extended:e}"
        );
    }

    #[test]
    #[allow(clippy::unreadable_literal, clippy::excessive_precision)] // Exact oracle output.
    fn independent_littlecms_profiles_preserve_extended_linear_samples() {
        // Profiles and expected values are generated by LittleCMS 2.19, not
        // moxcms. See tests/fixtures/icc/README.md for the exact recipe.
        const INPUT: [u8; 24] = [
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 51, 102, 153,
            255, 255, 255,
        ];
        const P3: [[f32; 3]; 8] = [
            [1.22493947, -0.0420472585, -0.0196422264],
            [-0.224948391, 1.04206085, -0.078627646],
            [-0.00000937440291, 0.000013065016, 1.09827411],
            [-0.224957764, 1.04207385, 1.01964653],
            [1.22493005, -0.0420341939, 1.078632],
            [0.999991059, 1.00001359, -0.0982698724],
            [0.00554445293, 0.137596995, 0.345936954],
            [0.999981701, 1.00002658, 1.00000429],
        ];
        const ADOBE: [[f32; 3]; 8] = [
            [1.39835644, -0.00000232951379, 0.00000708362313],
            [-0.398333818, 0.999988735, -0.0429379642],
            [-0.00000875071964, 0.0000104610817, 1.04291725],
            [-0.39834258, 0.999999166, 0.999979258],
            [1.39834774, 0.00000813156839, 1.04292428],
            [1.00002265, 0.99998641, -0.0429308824],
            [-0.0125112822, 0.133305743, 0.333398402],
            [1.00001383, 0.999996841, 0.999986351],
        ];
        for (name, icc, expected) in [
            (
                "Display P3 gamma 2.2",
                &include_bytes!("../tests/fixtures/icc/display-p3-gamma22.icc")[..],
                P3,
            ),
            (
                "Adobe RGB 1998",
                &include_bytes!("../tests/fixtures/icc/adobe-rgb-1998.icc")[..],
                ADOBE,
            ),
        ] {
            let grid = Grid {
                info: DecoderInfo {
                    depth: 8,
                    size: Size::new(8, 1).unwrap(),
                    layout: ChannelLayout::Rgb,
                    sample: SampleType::U8,
                    access: DecodeAccess::CodecRegionRequest,
                    alpha: AlphaMode::Opaque,
                    rendered_icc: Some(icc.to_vec()),
                    rendered_cicp: None,
                },
                row: INPUT.to_vec(),
            };
            let mut source = IccSrgbSource::new(Box::new(grid)).unwrap();
            let rows = read_rows(&mut source, 8);
            let actual = &rows[0];
            let mut worst_in_gamut = 0.0_f32;
            let mut worst_out_of_gamut = 0.0_f32;
            for (index, (pixel, oracle)) in
                actual.as_chunks::<3>().0.iter().zip(expected).enumerate()
            {
                let out_of_gamut = oracle.iter().any(|&v| !(0.0..=1.0).contains(&v));
                for (channel, (&actual, expected)) in pixel.iter().zip(oracle).enumerate() {
                    if out_of_gamut {
                        worst_out_of_gamut = worst_out_of_gamut.max((actual - expected).abs());
                    } else {
                        worst_in_gamut = worst_in_gamut.max((actual - expected).abs());
                    }
                    assert!(
                        (actual - expected).abs() < 8.0e-4,
                        "{name} pixel {index} channel {channel}: {actual} vs {expected}"
                    );
                }
            }
            eprintln!("{name}: in gamut {worst_in_gamut:e}, out of gamut {worst_out_of_gamut:e}");
            assert!(actual.iter().any(|&v| v < -0.01));
            assert!(actual.iter().any(|&v| v > 1.01));
        }
    }

    #[test]
    #[allow(clippy::unreadable_literal, clippy::excessive_precision)] // Exact oracle output.
    fn float_input_and_deferred_conversion_keep_extended_values() {
        let icc = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let info = DecoderInfo {
            depth: 8,
            size: Size::new(2, 1).unwrap(),
            layout: ChannelLayout::Rgb,
            sample: SampleType::F32,
            access: DecodeAccess::CodecRegionRequest,
            alpha: AlphaMode::Opaque,
            rendered_icc: Some(icc),
            rendered_cicp: None,
        };
        let row = vec![1.0, 0.0, 0.0, 0.0, 1.0, 1.0];
        let direct = read_rows(
            &mut IccSrgbSource::new(Box::new(FloatRow {
                info: info.clone(),
                row: row.clone(),
            }))
            .unwrap(),
            2,
        );
        let mut deferred = IccSrgbSource::new(Box::new(FloatRow {
            info,
            row: row.clone(),
        }))
        .unwrap();
        let conversion = deferred.defer_conversion().unwrap();
        let mut scratch = vec![0.0; row.len()];
        let mut out = vec![0.0; row.len()];
        conversion
            .convert(DecodedRow::F32(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(out, direct[0]);
        assert!((out[0] - 1.22494018).abs() < 2.0e-3);
        assert!((out[3] + 0.224940181).abs() < 2.0e-3);
    }

    #[test]
    #[allow(clippy::unreadable_literal, clippy::excessive_precision)] // Exact oracle output.
    fn external_profiles_match_littlecms_including_srgb_knee() {
        // These are the exact CC0 profile bytes reopened by the LittleCMS
        // oracle, including the sRGB parametric TRC of DisplayP3-v4.
        const INPUT: [u8; 30] = [
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 51, 102, 153,
            255, 255, 255, 10, 11, 12, 11, 10, 9,
        ];
        const P3: [[f32; 3]; 10] = [
            [1.22493947, -0.0420472585, -0.0196422264],
            [-0.224948391, 1.04206085, -0.078627646],
            [0.0000153022465, -0.0000161728094, 1.09827769],
            [-0.224933088, 1.04204464, 1.01964998],
            [1.22495472, -0.0420634337, 1.07863545],
            [0.999991059, 1.00001359, -0.0982698724],
            [0.010667868, 0.137058184, 0.338752538],
            [1.00000644, 0.999997377, 1.00000775],
            [0.0029649022, 0.00335985888, 0.0037153177],
            [0.00341690634, 0.00302189123, 0.00269557559],
        ];
        const ADOBE: [[f32; 3]; 10] = [
            [1.39835644, -0.00000232951379, 0.00000708362313],
            [-0.398333818, 0.999988735, -0.0429379642],
            [-0.0000162381166, 0.0000109712355, 1.04293871],
            [-0.39835006, 0.999999702, 1.00000072],
            [1.39834023, 0.00000864172125, 1.04294574],
            [1.00002265, 0.99998641, -0.0429308824],
            [-0.0125137167, 0.133305907, 0.333405375],
            [1.00000644, 0.999997377, 1.00000775],
            [0.000731762324, 0.00099481456, 0.0012136274],
            [0.00106976007, 0.000806692697, 0.000632695912],
        ];
        for (name, icc, expected) in [
            (
                "DisplayP3-v4",
                &include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc")[..],
                P3,
            ),
            (
                "AdobeCompat-v2",
                &include_bytes!("../tests/fixtures/icc/AdobeCompat-v2.icc")[..],
                ADOBE,
            ),
        ] {
            let grid = Grid {
                info: DecoderInfo {
                    depth: 8,
                    size: Size::new(10, 1).unwrap(),
                    layout: ChannelLayout::Rgb,
                    sample: SampleType::U8,
                    access: DecodeAccess::CodecRegionRequest,
                    alpha: AlphaMode::Opaque,
                    rendered_icc: Some(icc.to_vec()),
                    rendered_cicp: None,
                },
                row: INPUT.to_vec(),
            };
            let mut source = IccSrgbSource::new(Box::new(grid)).unwrap();
            let actual = read_rows(&mut source, 10).pop().unwrap();
            let mut worst_in_gamut = 0.0_f32;
            let mut worst_out_of_gamut = 0.0_f32;
            let mut worst_knee = 0.0_f32;
            for (index, (pixel, oracle)) in
                actual.as_chunks::<3>().0.iter().zip(expected).enumerate()
            {
                let out_of_gamut = oracle.iter().any(|&v| !(0.0..=1.0).contains(&v));
                for (channel, (&actual, expected)) in pixel.iter().zip(oracle).enumerate() {
                    let difference = (actual - expected).abs();
                    if out_of_gamut {
                        worst_out_of_gamut = worst_out_of_gamut.max(difference);
                    } else {
                        worst_in_gamut = worst_in_gamut.max(difference);
                    }
                    if index >= 8 {
                        worst_knee = worst_knee.max(difference);
                    }
                    let limit = if index >= 8 { 1.0e-5 } else { 8.0e-4 };
                    assert!(
                        difference < limit,
                        "{name} pixel {index} channel {channel}: {actual} vs {expected} (limit {limit})"
                    );
                }
            }
            eprintln!(
                "{name}: in gamut {worst_in_gamut:e}, out of gamut {worst_out_of_gamut:e}, knee {worst_knee:e}"
            );
            assert!(actual.iter().any(|&v| v < -0.01));
            assert!(actual.iter().any(|&v| v > 1.01));
        }
    }

    /// An 8-bit RGBA source with straight alpha in the Display P3 profile.
    fn p3_rgba_grid(row: Vec<u8>) -> Grid {
        let mut grid = p3_grid(row);
        grid.info.size = Size::new(u32::try_from(grid.row.len() / 4).unwrap(), 1).unwrap();
        grid.info.layout = ChannelLayout::Rgba;
        grid.info.alpha = AlphaMode::Straight;
        grid
    }

    /// RGBA test rows: every alpha class (0, partial, 255) against varied
    /// color, including saturated color under zero alpha.
    fn rgba_row() -> Vec<u8> {
        (0..=255_u8)
            .flat_map(|v| [v, 255 - v, v / 3, [0, 1, 128, 254, 255][usize::from(v) % 5]])
            .collect()
    }

    /// The color of an RGBA row converted as opaque RGB.
    fn rgb_reference(rgba: &[u8]) -> Vec<f32> {
        let rgb: Vec<u8> = rgba
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[1], p[2]])
            .collect();
        let width = u32::try_from(rgb.len() / 3).unwrap();
        read_rows(
            &mut IccSrgbSource::new(Box::new(p3_grid(rgb))).unwrap(),
            width,
        )
        .pop()
        .unwrap()
    }

    /// Issue #222: straight RGBA converts its color exactly as opaque RGB
    /// does (bit for bit, on the 8-bit table path, the 8-bit moxcms path
    /// and the float moxcms path) and carries alpha through unchanged; the
    /// adapter still declares straight alpha, and deferral changes nothing.
    #[test]
    #[allow(clippy::too_many_lines)] // Three conversion paths, one fixture.
    fn straight_rgba_converts_color_and_carries_alpha_unchanged() {
        let row = rgba_row();
        let width = u32::try_from(row.len() / 4).unwrap();
        let reference = rgb_reference(&row);
        let check = |out: &[f32], alpha: &dyn Fn(usize) -> f32, label: &str| {
            assert_eq!(out.len(), row.len(), "{label}");
            for (i, (pixel, rgb)) in out
                .as_chunks::<4>()
                .0
                .iter()
                .zip(reference.as_chunks::<3>().0)
                .enumerate()
            {
                assert_eq!(
                    pixel[..3].iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    rgb.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    "{label} pixel {i}"
                );
                assert_eq!(pixel[3].to_bits(), alpha(i).to_bits(), "{label} pixel {i}");
            }
        };
        let alpha8 = |i: usize| f32::from(row[i * 4 + 3]) / 255.0;

        // 8-bit input, channel-table path, direct and deferred.
        let mut source = IccSrgbSource::new(Box::new(p3_rgba_grid(row.clone()))).unwrap();
        assert!(source.conversion().rgb_table.is_some());
        assert_eq!(source.info().layout, ChannelLayout::Rgba);
        assert_eq!(source.info().alpha, AlphaMode::Straight);
        assert_eq!(source.info().sample, SampleType::F32);
        assert_eq!(source.info().rendered_cicp, Some([1, 8, 0, 1]));
        let direct = read_rows(&mut source, width).pop().unwrap();
        check(&direct, &alpha8, "u8 table");
        let mut deferred = IccSrgbSource::new(Box::new(p3_rgba_grid(row.clone()))).unwrap();
        let conversion = deferred.defer_conversion().unwrap();
        let mut scratch = vec![0.0; row.len()];
        let mut out = vec![0.0; row.len()];
        conversion
            .convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(out, direct);

        // 8-bit input through moxcms (no table), as for LUT-based profiles.
        let moxcms_source =
            IccSrgbSource::new_with_cache(Box::new(p3_rgba_grid(row.clone())), None, 0, None)
                .unwrap();
        let moxcms = moxcms_source.conversion();
        assert!(moxcms.rgb_table.is_none());
        let mut scratch = vec![f32::NAN; row.len()];
        let mut out = vec![f32::NAN; row.len()];
        moxcms
            .convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        // Same moxcms evaluation as the opaque float path, so compare with
        // a float RGB conversion of the same normalized samples.
        let rgb_float: Vec<f32> = row
            .as_chunks::<4>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[1], p[2]].map(|v| f32::from(v) / 255.0))
            .collect();
        let mut expected = vec![0.0_f32; rgb_float.len()];
        moxcms
            .transformer
            .as_ref()
            .unwrap()
            .transform(&rgb_float, &mut expected)
            .unwrap();
        for (i, (pixel, rgb)) in out
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<3>().0)
            .enumerate()
        {
            assert_eq!(pixel[..3], rgb[..], "u8 moxcms pixel {i}");
            assert_eq!(
                pixel[3].to_bits(),
                alpha8(i).to_bits(),
                "u8 moxcms pixel {i}"
            );
        }

        // Float input: moxcms path; alpha is copied bit for bit, including
        // values the executor would later refuse (it validates, not this
        // stage).
        let float_alpha = [0.0_f32, 0.25, 1.0, 0.999, f32::MIN_POSITIVE, 0.5];
        let float_row: Vec<f32> = rgb_float
            .as_chunks::<3>()
            .0
            .iter()
            .enumerate()
            .flat_map(|(i, p)| [p[0], p[1], p[2], float_alpha[i % float_alpha.len()]])
            .collect();
        let info = DecoderInfo {
            depth: 8,
            size: Size::new(width, 1).unwrap(),
            layout: ChannelLayout::Rgba,
            sample: SampleType::F32,
            access: DecodeAccess::CodecRegionRequest,
            alpha: AlphaMode::Straight,
            rendered_icc: Some(moxcms::ColorProfile::new_display_p3().encode().unwrap()),
            rendered_cicp: None,
        };
        let float = read_rows(
            &mut IccSrgbSource::new(Box::new(FloatRow {
                info,
                row: float_row,
            }))
            .unwrap(),
            width,
        )
        .pop()
        .unwrap();
        for (i, (pixel, rgb)) in float
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<3>().0)
            .enumerate()
        {
            assert_eq!(pixel[..3], rgb[..], "f32 pixel {i}");
            assert_eq!(
                pixel[3].to_bits(),
                float_alpha[i % float_alpha.len()].to_bits(),
                "f32 pixel {i}"
            );
        }

        // A row that is not whole RGBA pixels is rejected.
        assert_eq!(
            conversion.convert(
                DecodedRow::U8(&row[..row.len() - 2]),
                &mut scratch[..row.len() - 2],
                &mut out[..row.len() - 2]
            ),
            Err(Error::IncompleteDecoderOutput)
        );
    }

    /// Issue #222: only straight alpha is admitted. Premultiplied color
    /// would need division before the nonlinear profile curve and
    /// unspecified alpha has no meaning; neither is guessed. Layouts other
    /// than RGB, RGBA, gray and gray with alpha stay refused, and a profile
    /// of the other color space than the channels is refused (2026-11-15;
    /// before, gray was `UnsupportedLayout` here).
    #[test]
    fn non_straight_alpha_and_other_layouts_are_refused() {
        for alpha in [AlphaMode::Premultiplied, AlphaMode::Opaque] {
            let mut grid = p3_rgba_grid(rgba_row());
            grid.info.alpha = alpha;
            assert_eq!(
                IccSrgbSource::new(Box::new(grid)).map(|_| ()),
                Err(Error::UnsupportedAlphaMode),
                "{alpha:?}"
            );
        }
        for (layout, alpha, error) in [
            (
                ChannelLayout::Rgb,
                AlphaMode::Straight,
                Error::UnsupportedLayout,
            ),
            (
                ChannelLayout::Cmyk,
                AlphaMode::Opaque,
                Error::UnsupportedLayout,
            ),
            (
                ChannelLayout::GrayAlpha,
                AlphaMode::Premultiplied,
                Error::UnsupportedAlphaMode,
            ),
            // The Display P3 profile is an RGB profile.
            (
                ChannelLayout::Gray,
                AlphaMode::Opaque,
                Error::IccColorSpaceMismatch,
            ),
            (
                ChannelLayout::GrayAlpha,
                AlphaMode::Straight,
                Error::IccColorSpaceMismatch,
            ),
        ] {
            let mut grid = p3_grid(vec![1, 2, 3]);
            grid.info.layout = layout;
            grid.info.alpha = alpha;
            assert_eq!(
                IccSrgbSource::new(Box::new(grid)).map(|_| ()),
                Err(error),
                "{layout:?} {alpha:?}"
            );
        }
        let mut rgb = p3_grid(vec![1, 2, 3]);
        rgb.info.rendered_icc = Some(gray_profile());
        assert_eq!(
            IccSrgbSource::new(Box::new(rgb)).map(|_| ()),
            Err(Error::IccColorSpaceMismatch),
            "a gray profile beside RGB samples"
        );
    }

    /// The parity fixtures' gray profile: `GRAY`, one gamma-2.2 `kTRC`.
    fn gray_profile() -> Vec<u8> {
        include_bytes!("../tests/fixtures/parity/gray-gamma22.icc").to_vec()
    }

    /// Gray and gray with straight alpha through a gray profile (owner
    /// decision 2026-11-15): the rows equal moxcms called directly on the
    /// gray samples (Gray to RGB, linear sRGB destination, the stage's
    /// options) bit for bit, R = G = B, alpha carried over; the conversion
    /// is not deferred, because the rows grow.
    #[test]
    #[allow(clippy::too_many_lines)] // Direct, deferred, and fused checks for Gray and GrayAlpha.
    fn gray_profiles_convert_gray_into_rgb_like_moxcms() {
        let gray: Vec<u8> = (0..=255).collect();
        let mut linear = ColorProfile::new_srgb();
        let identity = ToneReprCurve::Lut(Vec::new());
        linear.red_trc = Some(identity.clone());
        linear.green_trc = Some(identity.clone());
        linear.blue_trc = Some(identity);
        linear.cicp = None;
        let direct = ColorProfile::new_from_slice(&gray_profile())
            .unwrap()
            .create_transform_f32(
                Layout::Gray,
                &linear,
                Layout::Rgb,
                TransformOptions {
                    rendering_intent: RenderingIntent::RelativeColorimetric,
                    allow_extended_range_rgb_xyz: true,
                    ..TransformOptions::default()
                },
            )
            .unwrap();
        let input: Vec<f32> = gray.iter().map(|&v| f32::from(v) / 255.0).collect();
        let mut expected = vec![0.0_f32; input.len() * 3];
        direct.transform(&input, &mut expected).unwrap();
        for (pixel, &v) in expected.as_chunks::<3>().0.iter().zip(&gray) {
            assert!(
                pixel.iter().all(|c| c.to_bits() == pixel[0].to_bits()),
                "{v}: {pixel:?}"
            );
            let independent = (f64::from(v) / 255.0).powf(2.199_218_75);
            assert!((f64::from(pixel[0]) - independent).abs() < 1e-5, "{v}");
        }
        let mut opaque = p3_grid(gray.clone());
        opaque.info.size = Size::new(256, 1).unwrap();
        opaque.info.layout = ChannelLayout::Gray;
        opaque.info.rendered_icc = Some(gray_profile());
        let mut source = IccSrgbSource::new(Box::new(opaque.clone())).unwrap();
        assert_eq!(source.info().layout, ChannelLayout::Rgb);
        assert!(source.conversion().gray_table.is_some());
        let rows = read_rows(&mut source, 256);
        assert_eq!(
            rows[0].iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        // Gray ICC defers to worker threads and fuses directly into linear RGBA.
        let mut deferred_gray = IccSrgbSource::new(Box::new(opaque)).unwrap();
        let conv = deferred_gray
            .defer_conversion()
            .expect("gray ICC defers to worker threads");
        assert_eq!(conv.input_channels(), 1);
        assert_eq!(conv.scratch_channels(), (0, 0));
        let mut linear = vec![0.0_f32; 256 * 4];
        assert!(
            conv.convert_to_linear_rgba(DecodedRow::U8(&gray), &mut linear)
                .unwrap()
        );
        for (px, rgb) in linear
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<3>().0)
        {
            assert_eq!(
                [px[0], px[1], px[2]].map(f32::to_bits),
                rgb.map(f32::to_bits)
            );
            assert_eq!(px[3].to_bits(), 0.0_f32.to_bits());
        }
        let with_alpha: Vec<u8> = gray.iter().flat_map(|&v| [v, 255 - v]).collect();
        let mut translucent = p3_grid(with_alpha.clone());
        translucent.info.size = Size::new(256, 1).unwrap();
        translucent.info.layout = ChannelLayout::GrayAlpha;
        translucent.info.alpha = AlphaMode::Straight;
        translucent.info.rendered_icc = Some(gray_profile());
        let mut source = IccSrgbSource::new(Box::new(translucent.clone())).unwrap();
        assert_eq!(source.info().layout, ChannelLayout::Rgba);
        assert!(source.conversion().gray_table.is_some());
        let rows = read_rows(&mut source, 256);
        for ((pixel, rgb), &v) in rows[0]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<3>().0)
            .zip(&gray)
        {
            assert_eq!(
                pixel[..3].iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                rgb.map(f32::to_bits)
            );
            assert_eq!(pixel[3].to_bits(), (f32::from(255 - v) / 255.0).to_bits());
        }
        let mut deferred_ga = IccSrgbSource::new(Box::new(translucent)).unwrap();
        let conv = deferred_ga
            .defer_conversion()
            .expect("gray+alpha ICC defers to worker threads");
        assert_eq!(conv.input_channels(), 2);
        assert_eq!(conv.scratch_channels(), (0, 0));
        let mut linear = vec![0.0_f32; 256 * 4];
        assert!(
            conv.convert_to_linear_rgba(DecodedRow::U8(&with_alpha), &mut linear)
                .unwrap()
        );
        for ((px, rgb), &v) in linear
            .as_chunks::<4>()
            .0
            .iter()
            .zip(expected.as_chunks::<3>().0)
            .zip(&gray)
        {
            let alpha = f32::from(255 - v) / 255.0;
            assert_eq!(
                px.map(f32::to_bits),
                [rgb[0] * alpha, rgb[1] * alpha, rgb[2] * alpha, alpha].map(f32::to_bits)
            );
        }
    }

    /// The five matrix-shaper profiles of the channel-table evidence (issue
    /// #219): two sRGB serializations, Display P3 with the sRGB curve,
    /// Adobe RGB and Display P3 with gamma 2.2.
    fn table_profiles() -> [(&'static str, Vec<u8>); 5] {
        [
            (
                "moxcms sRGB",
                moxcms::ColorProfile::new_srgb().encode().unwrap(),
            ),
            (
                "paris sRGB",
                include_bytes!("../tests/fixtures/icc/paris-srgb.icc").to_vec(),
            ),
            (
                "Display P3 v4",
                include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec(),
            ),
            (
                "Adobe RGB",
                include_bytes!("../tests/fixtures/icc/adobe-rgb-1998.icc").to_vec(),
            ),
            (
                "P3 gamma 2.2",
                include_bytes!("../tests/fixtures/icc/display-p3-gamma22.icc").to_vec(),
            ),
        ]
    }

    fn table_conversion(icc: Vec<u8>, sample: SampleType) -> std::sync::Arc<super::IccConversion> {
        let mut grid = p3_grid(vec![0; 3]);
        grid.info.rendered_icc = Some(icc);
        grid.info.sample = sample;
        IccSrgbSource::new(Box::new(grid))
            .unwrap()
            .conversion()
            .clone()
    }

    /// Largest |table - moxcms| over `triplets`, through `convert` (the
    /// production row path) against moxcms's own transform.
    fn table_deviation(icc: &[u8], conversion: &super::IccConversion, triplets: &[[u8; 3]]) -> f32 {
        let row: Vec<u8> = triplets.iter().flatten().copied().collect();
        let mut scratch = vec![0.0_f32; row.len()];
        let mut actual = vec![0.0_f32; row.len()];
        conversion
            .convert(DecodedRow::U8(&row), &mut scratch, &mut actual)
            .unwrap();
        let input: Vec<f32> = row.iter().map(|&v| f32::from(v) / 255.0).collect();
        let mut exact = vec![0.0_f32; row.len()];
        let (transformer, _) = super::build_moxcms_transform(icc, false, None).unwrap();
        transformer.transform(&input, &mut exact).unwrap();
        actual
            .iter()
            .zip(&exact)
            .map(|(a, e)| (a - e).abs())
            .fold(0.0, f32::max)
    }

    /// Issue #219: for 8-bit matrix-shaper input the per-channel table
    /// stays within `CHANNEL_TABLE_BOUND` of moxcms on a dense deterministic
    /// sample (every R value against 64 G and 17 B values, plus 65,536
    /// pseudo-random triplets), for each of the five profiles.
    ///
    /// Why a sample bounds the whole cube: each table entry is moxcms's own
    /// output for one channel, `M[:, c] · trc_c(v)`, and is monotone in `v`
    /// per output component (checked here: the curves are monotone and the
    /// matrix is fixed). The table's value at any triplet is therefore the
    /// exact separable sum; its difference from moxcms is only the f32
    /// rounding of moxcms's own matrix product versus three additions,
    /// a few units in the last place of the output, not a function of
    /// position that could peak between probes. The exhaustive 2^24 check
    /// is `channel_table_exhaustive_deviation` (ignored; run in release).
    #[test]
    fn channel_table_stays_within_the_bound_of_moxcms() {
        let mut triplets = Vec::new();
        for r in 0..=255_u8 {
            for g in (0..=255_u8).step_by(4).chain([255]) {
                for b in (0..=255_u8).step_by(16).chain([255]) {
                    triplets.push([r, g, b]);
                }
            }
        }
        let mut state = 0x9e37_79b9_u32;
        for _ in 0..65_536 {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let [r, g, b, _] = state.to_le_bytes();
            triplets.push([r, g, b]);
        }
        for (name, icc) in table_profiles() {
            let conversion = table_conversion(icc.clone(), SampleType::U8);
            let table = conversion.rgb_table.as_ref().expect(name);
            for channel in 0..3 {
                for component in [0_usize, 1, 2] {
                    let column: Vec<f32> = (0..256)
                        .map(|v| table.entries[channel * 256 + v][component])
                        .collect();
                    let rising = column.windows(2).all(|w| w[0] <= w[1]);
                    let falling = column.windows(2).all(|w| w[0] >= w[1]);
                    assert!(
                        rising || falling,
                        "{name} channel {channel} component {component}"
                    );
                }
            }
            let worst = table_deviation(&icc, &conversion, &triplets);
            eprintln!(
                "{name}: max |table - moxcms| over {} triplets = {worst:e}",
                triplets.len()
            );
            assert!(worst <= super::CHANNEL_TABLE_BOUND, "{name}: {worst:e}");
        }
    }

    /// The exhaustive form of the bound: every one of the 2^24 8-bit
    /// triplets for each of the five profiles. Slow in debug builds; run
    /// with `cargo test --release -p ferrisma --lib channel_table_exhaustive
    /// -- --ignored --nocapture`.
    #[test]
    #[ignore = "exhaustive 2^24 check per profile; run in release"]
    fn channel_table_exhaustive_deviation() {
        for (name, icc) in table_profiles() {
            let conversion = table_conversion(icc.clone(), SampleType::U8);
            assert!(conversion.rgb_table.is_some(), "{name}");
            let mut worst = 0.0_f32;
            let mut plane = Vec::with_capacity(65_536);
            for r in 0..=255_u8 {
                plane.clear();
                for g in 0..=255_u8 {
                    for b in 0..=255_u8 {
                        plane.push([r, g, b]);
                    }
                }
                worst = worst.max(table_deviation(&icc, &conversion, &plane));
            }
            eprintln!("{name}: max |table - moxcms| over all 2^24 triplets = {worst:e}");
            assert!(worst <= super::CHANNEL_TABLE_BOUND, "{name}: {worst:e}");
        }
    }

    /// Only 8-bit input of the extended matrix-shaper path gets a table
    /// (LUT-based profiles never enter that path; see `IccSrgbSource::new`):
    /// float input stays on moxcms, and the table path reproduces its own
    /// `apply` exactly through the row interface.
    #[test]
    fn channel_table_is_limited_to_eight_bit_matrix_shaper_input() {
        for (name, icc) in table_profiles() {
            assert!(
                table_conversion(icc, SampleType::F32).rgb_table.is_none(),
                "{name}"
            );
        }
        let p3 = table_conversion(
            moxcms::ColorProfile::new_display_p3().encode().unwrap(),
            SampleType::U8,
        );
        let table = p3.rgb_table.as_ref().unwrap();
        let row = [0_u8, 10, 11, 128, 200, 255];
        let mut scratch = [0.0_f32; 6];
        let mut out = [0.0_f32; 6];
        p3.convert(DecodedRow::U8(&row), &mut scratch, &mut out)
            .unwrap();
        assert_eq!(out[..3], table.apply([0, 10, 11]));
        assert_eq!(out[3..], table.apply([128, 200, 255]));
    }

    /// The repository has no LUT-based profile fixture (#17, #211), so this
    /// profile is built with moxcms' own writer and read back: sRGB with an
    /// `A2B0` (perceptual) and an `A2B1` (colorimetric) lut16 table that
    /// map RGB to XYZ through two different 2x2x2 grids, the perceptual one
    /// at half the colorimetric one. The engine must convert with the
    /// colorimetric table (owner decision 2026-11-15, ADR-0016): its output
    /// equals moxcms with `RelativeColorimetric` and differs from moxcms
    /// with `Perceptual`, the library default used before.
    #[test]
    fn lut_profiles_convert_with_the_relative_colorimetric_table() {
        use moxcms::{
            ColorProfile, Layout, LutDataType, LutStore, LutType, LutWarehouse, Matrix3d,
            RenderingIntent, ToneReprCurve,
        };
        let table = |scale: f64| {
            // D50 XYZ of the eight RGB corners of sRGB, as u1Fixed15
            // (1.0 = 0x8000) of the lut16 PCS encoding.
            let srgb = ColorProfile::new_srgb();
            let corners: Vec<u16> = (0..8_u8)
                .flat_map(|corner| {
                    let rgb = [corner >> 2 & 1, corner >> 1 & 1, corner & 1].map(f64::from);
                    let xyz = [srgb.red_colorant, srgb.green_colorant, srgb.blue_colorant]
                        .iter()
                        .zip(rgb)
                        .fold([0.0_f64; 3], |sum, (colorant, weight)| {
                            [
                                sum[0] + colorant.x * weight,
                                sum[1] + colorant.y * weight,
                                sum[2] + colorant.z * weight,
                            ]
                        });
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    xyz.map(|value| (value * scale * 32_768.0).round().min(65_535.0) as u16)
                })
                .collect();
            LutWarehouse::Lut(LutDataType {
                num_input_channels: 3,
                num_output_channels: 3,
                num_clut_grid_points: 2,
                matrix: Matrix3d::IDENTITY,
                num_input_table_entries: 2,
                num_output_table_entries: 2,
                input_table: LutStore::Store16(vec![0, 65_535, 0, 65_535, 0, 65_535]),
                clut_table: LutStore::Store16(corners),
                output_table: LutStore::Store16(vec![0, 65_535, 0, 65_535, 0, 65_535]),
                lut_type: LutType::Lut16,
            })
        };
        let mut profile = ColorProfile::new_srgb();
        profile.cicp = None;
        profile.lut_a_to_b_perceptual = Some(table(0.5));
        profile.lut_a_to_b_colorimetric = Some(table(1.0));
        let icc = profile.encode().unwrap();
        let parsed = ColorProfile::new_from_slice(&icc).unwrap();
        assert!(parsed.lut_a_to_b_perceptual.is_some());
        assert!(parsed.lut_a_to_b_colorimetric.is_some());

        let row: Vec<u8> = [[255, 255, 255], [128, 128, 128], [200, 40, 90]].concat();
        let mut grid = p3_grid(row.clone());
        grid.info.rendered_icc = Some(icc);
        let mut source = IccSrgbSource::new(Box::new(grid)).unwrap();
        let engine = read_rows(&mut source, 3).remove(0);

        let mut linear = ColorProfile::new_srgb();
        let identity = ToneReprCurve::Lut(Vec::new());
        linear.red_trc = Some(identity.clone());
        linear.green_trc = Some(identity.clone());
        linear.blue_trc = Some(identity);
        linear.cicp = None;
        let direct = |rendering_intent| {
            let transform = parsed
                .create_transform_f32(
                    Layout::Rgb,
                    &linear,
                    Layout::Rgb,
                    moxcms::TransformOptions {
                        rendering_intent,
                        ..moxcms::TransformOptions::default()
                    },
                )
                .unwrap();
            let input: Vec<f32> = row.iter().map(|&value| f32::from(value) / 255.0).collect();
            let mut output = vec![0.0_f32; input.len()];
            transform.transform(&input, &mut output).unwrap();
            output
        };
        let relative = direct(RenderingIntent::RelativeColorimetric);
        let perceptual = direct(RenderingIntent::Perceptual);
        assert_eq!(engine, relative, "the colorimetric table is used");
        let largest = engine
            .iter()
            .zip(&perceptual)
            .map(|(engine, perceptual)| (engine - perceptual).abs())
            .fold(0.0_f32, f32::max);
        assert!(largest > 0.1, "the perceptual table differs by {largest}");
    }

    /// Issue #219: `IccTableCache` reuses verified tables on exact byte
    /// matches, rejects mismatches and contradicting decoder CICP on hits,
    /// never conflates profiles differing by one byte, skips caching
    /// ineligible LUT profiles so they cannot evict reusable tables, and
    /// accounts for every retained allocation (`Vec` capacity, `Arc` control
    /// blocks, keys, and tables) inside its `MemoryBudget`.
    #[test]
    #[allow(clippy::too_many_lines)] // Positive, mismatch, CICP, byte-equality, LUT non-eviction, and MemoryBudget cases.
    fn icc_table_cache_reuses_positive_and_negative_entries_and_checks_exact_bytes() {
        use super::{
            CachedProfile, GRAY_TABLE_MIN_PIXELS, IccTableCache, MAX_CACHE_ENTRIES,
            RGB_TABLE_MIN_PIXELS,
        };
        use std::sync::Arc;

        let cache = IccTableCache::new();
        let p3 = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let make_rgb = |icc: Vec<u8>, cicp: Option<[u8; 4]>| {
            let mut grid = p3_grid(vec![10, 20, 30]);
            grid.info.size = Size::new(100, 100).unwrap();
            grid.info.rendered_icc = Some(icc);
            grid.info.rendered_cicp = cicp;
            grid
        };

        // First large RGB call builds and caches ChannelTable without
        // retaining a `moxcms` executor.
        let first = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3.clone(), None)),
            Some(&cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(first.conversion().transformer.is_none());
        let first_table = first
            .conversion()
            .rgb_table
            .as_ref()
            .expect("built")
            .clone();
        let second = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3.clone(), None)),
            Some(&cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(second.conversion().transformer.is_none());
        assert!(Arc::ptr_eq(
            &first_table,
            second.conversion().rgb_table.as_ref().expect("cached hit")
        ));

        // Agreeing decoder CICP passes on cache hit; contradicting CICP fails.
        assert!(
            IccSrgbSource::new_with_cache(
                Box::new(make_rgb(p3.clone(), Some([12, 13, 0, 1]))),
                Some(&cache),
                RGB_TABLE_MIN_PIXELS,
                None,
            )
            .is_ok()
        );
        assert!(matches!(
            IccSrgbSource::new_with_cache(
                Box::new(make_rgb(p3.clone(), Some([1, 13, 0, 1]))),
                Some(&cache),
                RGB_TABLE_MIN_PIXELS,
                None,
            ),
            Err(Error::CicpBesideIcc(_))
        ));

        // A cached RGB profile beside gray samples is still rejected.
        let mut wrong_layout = make_rgb(p3.clone(), None);
        wrong_layout.info.layout = ChannelLayout::Gray;
        assert!(matches!(
            IccSrgbSource::new_with_cache(
                Box::new(wrong_layout),
                Some(&cache),
                RGB_TABLE_MIN_PIXELS,
                None,
            ),
            Err(Error::IccColorSpaceMismatch)
        ));

        // Exact-byte match: distinct or 1-byte-altered profiles miss the cache.
        let srgb = ColorProfile::new_srgb().encode().unwrap();
        let srgb_src = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(srgb.clone(), None)),
            Some(&cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        let srgb_table = srgb_src.conversion().rgb_table.as_ref().unwrap();
        let paris = include_bytes!("../tests/fixtures/icc/paris-srgb.icc").to_vec();
        let paris_src = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(paris, None)),
            Some(&cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(!Arc::ptr_eq(
            srgb_table,
            paris_src.conversion().rgb_table.as_ref().unwrap()
        ));
        // Exact-byte match: changing one informational creator header byte
        // (bytes 80..84 in the ICC header) misses the cache.
        let mut altered = srgb.clone();
        altered[80] ^= 1;
        let altered_src = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(altered, None)),
            Some(&cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(!Arc::ptr_eq(
            srgb_table,
            altered_src.conversion().rgb_table.as_ref().unwrap()
        ));

        // Ineligible LUT-based RGB profiles are NOT cached: they consume zero
        // cache budget and cannot evict an already-cached reusable table.
        let mut lut_profile = ColorProfile::new_srgb();
        lut_profile.cicp = None;
        lut_profile.lut_a_to_b_colorimetric =
            Some(moxcms::LutWarehouse::Lut(moxcms::LutDataType {
                num_input_channels: 3,
                num_output_channels: 3,
                num_clut_grid_points: 2,
                matrix: moxcms::Matrix3d::IDENTITY,
                num_input_table_entries: 2,
                num_output_table_entries: 2,
                input_table: moxcms::LutStore::Store16(vec![0, 65_535, 0, 65_535, 0, 65_535]),
                clut_table: moxcms::LutStore::Store16(vec![0; 8 * 3]),
                output_table: moxcms::LutStore::Store16(vec![0, 65_535, 0, 65_535, 0, 65_535]),
                lut_type: moxcms::LutType::Lut16,
            }));
        let lut_bytes = lut_profile.encode().unwrap();
        let one_rgb_entry_bytes =
            p3.len() + super::rgb_table_retained_bytes() + std::mem::size_of::<CachedProfile>();
        let lut_check_cache = IccTableCache::with_max_bytes(one_rgb_entry_bytes);
        let p3_cached = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3.clone(), None)),
            Some(&lut_check_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        let p3_cached_table = p3_cached.conversion().rgb_table.as_ref().unwrap().clone();
        let lut1 = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(lut_bytes.clone(), None)),
            Some(&lut_check_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        let lut2 = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(lut_bytes, None)),
            Some(&lut_check_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(lut1.conversion().rgb_table.is_none());
        assert!(lut2.conversion().rgb_table.is_none());
        assert!(!Arc::ptr_eq(
            lut1.conversion().transformer.as_ref().unwrap(),
            lut2.conversion().transformer.as_ref().unwrap()
        ));
        // The LUT profile neither inserted an entry nor evicted `p3_cached`.
        assert_eq!(lut_check_cache.state.lock().unwrap().entries.len(), 1);
        let p3_still_cached = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3.clone(), None)),
            Some(&lut_check_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(Arc::ptr_eq(
            &p3_cached_table,
            p3_still_cached.conversion().rgb_table.as_ref().unwrap()
        ));

        // Gray profile caches GrayTable (without retaining a moxcms executor),
        // reuses it at `>= GRAY_TABLE_MIN_PIXELS`, and skips it below the
        // cutoff so budget admission is identical to a cache miss.
        let mut gray = p3_grid(vec![0, 128, 255]);
        gray.info.layout = ChannelLayout::Gray;
        gray.info.size = Size::new(3, 1).unwrap();
        gray.info.rendered_icc = Some(gray_profile());
        let g1 = IccSrgbSource::new_with_cache(
            Box::new(gray.clone()),
            Some(&cache),
            GRAY_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(g1.conversion().transformer.is_none());
        let g1_table = g1.conversion().gray_table.as_ref().expect("built").clone();
        let g2 = IccSrgbSource::new_with_cache(
            Box::new(gray.clone()),
            Some(&cache),
            GRAY_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(g2.conversion().transformer.is_none());
        assert!(Arc::ptr_eq(
            &g1_table,
            g2.conversion().gray_table.as_ref().expect("reused")
        ));
        let g_tiny = IccSrgbSource::new_with_cache(Box::new(gray), Some(&cache), 16, None).unwrap();
        assert!(g_tiny.conversion().gray_table.is_none());
        assert!(g_tiny.conversion().transformer.is_some());

        // Release prior conversion handles before checking steady-state
        // eviction accounting (held-across-eviction accounting is tested below).
        drop((
            first,
            first_table,
            second,
            srgb_src,
            paris_src,
            altered_src,
            g1,
            g1_table,
            g2,
        ));

        // Bounded capacity & exact MemoryBudget accounting: inserting more
        // than `MAX_CACHE_ENTRIES` distinct profiles evicts old entries, and
        // live/peak bytes match `Vec` capacity + `Arc` headers + keys + tables.
        let mut base = ColorProfile::new_srgb().encode().unwrap();
        for i in 0..(MAX_CACHE_ENTRIES + 8) {
            base[80] = u8::try_from(i & 0xff).unwrap();
            base[81] = u8::try_from((i >> 8) & 0xff).unwrap();
            let _ = IccSrgbSource::new_with_cache(
                Box::new(make_rgb(base.clone(), None)),
                Some(&cache),
                RGB_TABLE_MIN_PIXELS,
                None,
            )
            .unwrap();
        }
        {
            let state = cache.state.lock().unwrap();
            assert!(state.entries.len() <= MAX_CACHE_ENTRIES);
            let exact_entries: usize = state
                .entries
                .iter()
                .map(CachedProfile::retained_bytes)
                .sum();
            let exact_vec = state.entries.capacity() * std::mem::size_of::<CachedProfile>();
            assert_eq!(cache.live_bytes(), exact_entries + exact_vec);
            assert!(cache.peak_bytes() <= cache.max_bytes);
            assert!(cache.peak_bytes() >= cache.live_bytes());
        }

        // Strict per-batch byte budget: a zero cap skips caching altogether,
        // and a one-entry cap (including Vec slot + Arc headers) evicts the
        // previous entry when a second arrives without exceeding `max_bytes`.
        let zero_cache = IccTableCache::with_max_bytes(0);
        let _ = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3.clone(), None)),
            Some(&zero_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(zero_cache.state.lock().unwrap().entries.is_empty());
        assert_eq!(zero_cache.peak_bytes(), 0);

        let tight_cache = IccTableCache::with_max_bytes(one_rgb_entry_bytes);
        let first_held = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3.clone(), None)),
            Some(&tight_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert_eq!(tight_cache.state.lock().unwrap().entries.len(), 1);
        assert_eq!(tight_cache.live_bytes(), one_rgb_entry_bytes);

        // While a concurrent worker holds a cloned `Arc<ChannelTable>` from
        // the first entry, its table is in use (`strong_count > 1`) and cannot
        // free enough cache budget, so inserting a second distinct profile
        // skips caching without evicting the active first entry or exceeding
        // `max_bytes`.
        let mut p3_alt = p3.clone();
        p3_alt[80] ^= 1;
        let second_while_held = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3_alt.clone(), None)),
            Some(&tight_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(second_while_held.conversion().rgb_table.is_some());
        assert!(tight_cache.peak_bytes() <= one_rgb_entry_bytes);
        assert_eq!(tight_cache.live_bytes(), one_rgb_entry_bytes);
        assert_eq!(tight_cache.state.lock().unwrap().entries.len(), 1);

        // Dropping the held conversion makes the first entry idle/evictable,
        // allowing the second profile to evict it and be cached cleanly.
        drop(first_held);
        let second_cached = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3_alt.clone(), None)),
            Some(&tight_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        let second_table = second_cached
            .conversion()
            .rgb_table
            .as_ref()
            .unwrap()
            .clone();
        let second_hit = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(p3_alt, None)),
            Some(&tight_cache),
            RGB_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(Arc::ptr_eq(
            &second_table,
            second_hit.conversion().rgb_table.as_ref().unwrap()
        ));
        {
            let state = tight_cache.state.lock().unwrap();
            assert_eq!(state.entries.len(), 1);
            assert_eq!(tight_cache.live_bytes(), one_rgb_entry_bytes);
            assert!(tight_cache.peak_bytes() <= one_rgb_entry_bytes);
        }
    }

    /// Issue #219: RGB table selection depends only on full source dimensions
    /// (independent of crop pixel count and cache warmth); gray table
    /// selection applies its cutoff identically on cache hits and misses.
    #[test]
    fn small_image_cutoff_skips_tables_below_threshold() {
        use super::{GRAY_TABLE_MIN_PIXELS, IccTableCache, RGB_TABLE_MIN_PIXELS};

        let cache = IccTableCache::new();
        let p3 = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let make_rgb = |width: u32, height: u32| {
            let mut grid = p3_grid(vec![10, 20, 30]);
            grid.info.size = Size::new(width, height).unwrap();
            grid.info.rendered_icc = Some(p3.clone());
            grid
        };
        // Full image below 10,000 pixels stays on moxcms.
        let tiny_uncached = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(99, 100)),
            None,
            RGB_TABLE_MIN_PIXELS - 1,
            None,
        )
        .unwrap();
        assert!(tiny_uncached.conversion().rgb_table.is_none());
        assert!(tiny_uncached.conversion().transformer.is_some());

        // Full image >= 10,000 pixels uses ChannelTable even when crop hint is tiny.
        let large =
            IccSrgbSource::new_with_cache(Box::new(make_rgb(100, 100)), Some(&cache), 16, None)
                .unwrap();
        assert!(large.conversion().rgb_table.is_some());
        assert!(large.conversion().transformer.is_none());

        // A genuinely small RGB image (< 10,000 source pixels) still stays on
        // a fresh moxcms executor even after the large image warmed the cache.
        let tiny_cached = IccSrgbSource::new_with_cache(
            Box::new(make_rgb(99, 100)),
            Some(&cache),
            RGB_TABLE_MIN_PIXELS - 1,
            None,
        )
        .unwrap();
        assert!(tiny_cached.conversion().rgb_table.is_none());
        assert!(tiny_cached.conversion().transformer.is_some());

        // Gray cutoff applies identically on misses and warm cache hits.
        let mut gray = p3_grid(vec![10, 20, 30]);
        gray.info.layout = ChannelLayout::Gray;
        gray.info.size = Size::new(100, 100).unwrap();
        gray.info.rendered_icc = Some(gray_profile());
        let tiny_gray = IccSrgbSource::new_with_cache(
            Box::new(gray.clone()),
            None,
            GRAY_TABLE_MIN_PIXELS - 1,
            None,
        )
        .unwrap();
        assert!(tiny_gray.conversion().gray_table.is_none());
        assert!(tiny_gray.conversion().transformer.is_some());
        let big_gray = IccSrgbSource::new_with_cache(
            Box::new(gray.clone()),
            Some(&cache),
            GRAY_TABLE_MIN_PIXELS,
            None,
        )
        .unwrap();
        assert!(big_gray.conversion().gray_table.is_some());
        assert!(big_gray.conversion().transformer.is_none());
        let tiny_gray_cached = IccSrgbSource::new_with_cache(
            Box::new(gray),
            Some(&cache),
            GRAY_TABLE_MIN_PIXELS - 1,
            None,
        )
        .unwrap();
        assert!(tiny_gray_cached.conversion().gray_table.is_none());
        assert!(tiny_gray_cached.conversion().transformer.is_some());
    }

    /// Issue #219: fused `convert_to_linear_rgba` for RGB, RGBA, Gray and
    /// `GrayAlpha` is bit-identical to `convert` followed by `linearize_row`,
    /// and reports zero scratch channels.
    #[test]
    fn fused_channel_and_gray_tables_are_bit_identical_to_two_pass_linearize() {
        use crate::fast_execute::{Primaries, linearize_row};
        use ferrisma_kernels::Kernels;

        let kernels = Kernels::best();
        let identity_p3 = [[1.0_f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        // Opaque RGB
        let rgb: Vec<u8> = (0..=255_u8).flat_map(|v| [v, 255 - v, v / 2]).collect();
        let mut src = IccSrgbSource::new(Box::new(p3_grid(rgb.clone()))).unwrap();
        let conv = src.defer_conversion().unwrap();
        assert_eq!(conv.scratch_channels(), (0, 0));
        let mut fused = vec![0.0_f32; 256 * 4];
        assert!(
            conv.convert_to_linear_rgba(DecodedRow::U8(&rgb), &mut fused)
                .unwrap()
        );
        let mut converted = vec![0.0_f32; rgb.len()];
        conv.convert(DecodedRow::U8(&rgb), &mut [], &mut converted)
            .unwrap();
        let mut two_pass = vec![0.0_f32; 256 * 4];
        linearize_row::<3>(
            DecodedRow::F32(&converted),
            Primaries::LinearSrgb,
            &identity_p3,
            &kernels,
            &mut two_pass,
        )
        .unwrap();
        assert_eq!(
            fused.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            two_pass.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );

        // Straight RGBA
        let rgba = rgba_row();
        let mut src = IccSrgbSource::new(Box::new(p3_rgba_grid(rgba.clone()))).unwrap();
        let conv = src.defer_conversion().unwrap();
        assert_eq!(conv.scratch_channels(), (0, 0));
        let mut fused = vec![0.0_f32; 256 * 4];
        assert!(
            conv.convert_to_linear_rgba(DecodedRow::U8(&rgba), &mut fused)
                .unwrap()
        );
        let mut converted = vec![0.0_f32; rgba.len()];
        conv.convert(DecodedRow::U8(&rgba), &mut [], &mut converted)
            .unwrap();
        let mut two_pass = vec![0.0_f32; 256 * 4];
        linearize_row::<4>(
            DecodedRow::F32(&converted),
            Primaries::LinearSrgb,
            &identity_p3,
            &kernels,
            &mut two_pass,
        )
        .unwrap();
        assert_eq!(
            fused.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            two_pass.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    /// Regression for PR #297 Round-5 review (Finding #1): concurrent
    /// double-miss on the same profile under a 1-entry cache allowance reuses
    /// the already-cached `Arc` rather than evicting it and emptying the cache.
    #[test]
    fn icc_table_cache_reuses_existing_table_on_concurrent_double_miss() {
        use super::{
            CachedProfile, CachedTable, ChannelTable, GrayTable, IccTableCache,
            build_moxcms_transform, gray_table_retained_bytes, hash_profile_bytes,
            rgb_table_retained_bytes,
        };
        use std::sync::Arc;

        // RGB concurrent double-miss under a 1-entry allowance.
        let p3 = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let rgb_cap = p3.len() + rgb_table_retained_bytes() + std::mem::size_of::<CachedProfile>();
        let rgb_cache = IccTableCache::with_max_bytes(rgb_cap);
        let (rgb_xf, _) = build_moxcms_transform(&p3, false, None).unwrap();
        let rgb_hash = hash_profile_bytes(&p3);
        let first_rgb = rgb_cache.insert_rgb(
            rgb_hash,
            &p3,
            ChannelTable::build(rgb_xf.as_ref()).expect("first RGB table built"),
        );
        // Second worker finishes building the same profile while `first_rgb` is held.
        let second_rgb = rgb_cache.insert_rgb(
            rgb_hash,
            &p3,
            ChannelTable::build(rgb_xf.as_ref()).expect("second RGB table built"),
        );
        assert!(Arc::ptr_eq(&first_rgb, &second_rgb));
        assert_eq!(rgb_cache.state.lock().unwrap().entries.len(), 1);
        assert_eq!(rgb_cache.live_bytes(), rgb_cap);
        assert!(rgb_cache.peak_bytes() <= rgb_cap);
        let hit = rgb_cache
            .get(rgb_hash, &p3)
            .expect("entry remains cached after double-miss");
        let CachedTable::Rgb(Some(hit_rgb)) = hit.table else {
            panic!("expected cached RGB table");
        };
        assert!(Arc::ptr_eq(&first_rgb, &hit_rgb));

        // Gray concurrent double-miss under a 1-entry allowance.
        let gray_icc = gray_profile();
        let gray_cap =
            gray_icc.len() + gray_table_retained_bytes() + std::mem::size_of::<CachedProfile>();
        let gray_cache = IccTableCache::with_max_bytes(gray_cap);
        let (gray_xf, _) = build_moxcms_transform(&gray_icc, true, None).unwrap();
        let gray_hash = hash_profile_bytes(&gray_icc);
        let first_gray = gray_cache.insert_gray(
            gray_hash,
            &gray_icc,
            GrayTable::build(gray_xf.as_ref()).expect("first Gray table built"),
        );
        let second_gray = gray_cache.insert_gray(
            gray_hash,
            &gray_icc,
            GrayTable::build(gray_xf.as_ref()).expect("second Gray table built"),
        );
        assert!(Arc::ptr_eq(&first_gray, &second_gray));
        assert_eq!(gray_cache.state.lock().unwrap().entries.len(), 1);
        assert_eq!(gray_cache.live_bytes(), gray_cap);
        assert!(gray_cache.peak_bytes() <= gray_cap);
        let hit = gray_cache
            .get(gray_hash, &gray_icc)
            .expect("gray entry remains cached after double-miss");
        let CachedTable::Gray(Some(hit_gray)) = hit.table else {
            panic!("expected cached Gray table");
        };
        assert!(Arc::ptr_eq(&first_gray, &hit_gray));
    }

    /// Regression for PR #297 Round-5 review (Finding #2): eligible 8-bit gray
    /// conversions reserve `GRAY_TABLE_BYTES` against the image's `MemoryBudget`
    /// identically on standalone misses, batch misses, and warm batch hits.
    #[test]
    fn gray_table_reserves_image_budget_identically_on_miss_and_cache_hit() {
        use super::{GRAY_TABLE_BYTES, GRAY_TABLE_MIN_PIXELS, IccTableCache};

        let mut gray = p3_grid(vec![0, 128, 255]);
        gray.info.layout = ChannelLayout::Gray;
        gray.info.size = Size::new(100, 100).unwrap();
        gray.info.rendered_icc = Some(gray_profile());

        let cache = IccTableCache::new();
        // Standalone miss below `GRAY_TABLE_BYTES` fails before building.
        let too_small = MemoryBudget::new(GRAY_TABLE_BYTES - 1);
        assert!(matches!(
            IccSrgbSource::new_with_cache(
                Box::new(gray.clone()),
                None,
                GRAY_TABLE_MIN_PIXELS,
                Some(&too_small),
            ),
            Err(Error::BudgetExceeded { .. })
        ));
        assert_eq!(too_small.snapshot().live, 0);

        // Cold batch miss reserves `GRAY_TABLE_BYTES` and releases on drop.
        let miss_budget = MemoryBudget::new(GRAY_TABLE_BYTES);
        let miss_src = IccSrgbSource::new_with_cache(
            Box::new(gray.clone()),
            Some(&cache),
            GRAY_TABLE_MIN_PIXELS,
            Some(&miss_budget),
        )
        .unwrap();
        assert!(miss_src.conversion().gray_table.is_some());
        assert_eq!(miss_budget.snapshot().live, GRAY_TABLE_BYTES);
        assert_eq!(miss_budget.snapshot().peak, GRAY_TABLE_BYTES);
        drop(miss_src);
        assert_eq!(miss_budget.snapshot().live, 0);

        // Warm cache hit enforces the exact same budget admission & peak.
        let hit_too_small = MemoryBudget::new(GRAY_TABLE_BYTES - 1);
        assert!(matches!(
            IccSrgbSource::new_with_cache(
                Box::new(gray.clone()),
                Some(&cache),
                GRAY_TABLE_MIN_PIXELS,
                Some(&hit_too_small),
            ),
            Err(Error::BudgetExceeded { .. })
        ));
        let hit_budget = MemoryBudget::new(GRAY_TABLE_BYTES);
        let hit_src = IccSrgbSource::new_with_cache(
            Box::new(gray),
            Some(&cache),
            GRAY_TABLE_MIN_PIXELS,
            Some(&hit_budget),
        )
        .unwrap();
        assert!(hit_src.conversion().gray_table.is_some());
        assert_eq!(hit_budget.snapshot().live, GRAY_TABLE_BYTES);
        assert_eq!(hit_budget.snapshot().peak, GRAY_TABLE_BYTES);
        drop(hit_src);
        assert_eq!(hit_budget.snapshot().live, 0);
    }

    /// Regression for PR #297 Round-8 review (Finding #1):
    /// 1. On a cache miss with a budget below `RGB_TABLE_BYTES` (e.g. 4,000 B),
    ///    `IccSrgbSource` rejects with `Error::BudgetExceeded` before
    ///    allocating the table (`live == 0`, `peak == 0`).
    /// 2. Stack-chunked `ChannelTable::build` and `GrayTable::build` match
    ///    direct `moxcms` evaluation identically across every single-channel
    ///    code and within `CHANNEL_TABLE_BOUND` across sampled RGB triplets.
    #[test]
    fn rgb_table_reserves_budget_before_building_and_chunked_tables_match() {
        use super::{
            CHANNEL_TABLE_BOUND, ChannelTable, GrayTable, IccTableCache, RGB_TABLE_BYTES,
            build_moxcms_transform,
        };

        let mut p3 = p3_grid(vec![7, 19, 202]);
        p3.info.size = Size::new(100, 100).unwrap();

        // Miss with budget below `RGB_TABLE_BYTES` rejects before allocating
        // and leaves both `live` and `peak` at 0.
        let cache = IccTableCache::new();
        let too_small = MemoryBudget::new(4_000);
        assert!(matches!(
            IccSrgbSource::new_with_cache(
                Box::new(p3.clone()),
                Some(&cache),
                10_000,
                Some(&too_small),
            ),
            Err(Error::BudgetExceeded { .. })
        ));
        assert_eq!(too_small.snapshot().live, 0);
        assert_eq!(too_small.snapshot().peak, 0);
        // Cache remains empty because table build was never entered.
        assert_eq!(cache.live_bytes(), 0);

        // Exact `RGB_TABLE_BYTES` budget admits the table on miss and warm hit.
        let exact = MemoryBudget::new(RGB_TABLE_BYTES);
        let src =
            IccSrgbSource::new_with_cache(Box::new(p3.clone()), Some(&cache), 10_000, Some(&exact))
                .unwrap();
        assert!(src.conversion().rgb_table.is_some());
        assert_eq!(exact.snapshot().live, RGB_TABLE_BYTES);
        assert_eq!(exact.snapshot().peak, RGB_TABLE_BYTES);
        drop(src);
        assert_eq!(exact.snapshot().live, 0);

        // Verify stack-chunked `ChannelTable::build` matches direct `moxcms`
        // ramp entries bit-for-bit and arbitrary RGB triplets within bound.
        let p3_icc = ColorProfile::new_display_p3().encode().unwrap();
        let (rgb_xf, _) = build_moxcms_transform(&p3_icc, false, None).unwrap();
        let rgb_table = ChannelTable::build(rgb_xf.as_ref()).expect("Display P3 builds table");
        for code in 0..=255_u8 {
            let v = f32::from(code) / 255.0;
            for (input, rgb_u8) in [
                ([v, 0.0, 0.0], [code, 0, 0]),
                ([0.0, v, 0.0], [0, code, 0]),
                ([0.0, 0.0, v], [0, 0, code]),
            ] {
                let mut expected = [0.0_f32; 3];
                rgb_xf.transform(&input, &mut expected).unwrap();
                assert_eq!(rgb_table.apply(rgb_u8), expected);
            }
        }
        for r in (0..=255_u8).step_by(51) {
            for g in (0..=255_u8).step_by(51) {
                for b in (0..=255_u8).step_by(51) {
                    let mut expected = [0.0_f32; 3];
                    rgb_xf
                        .transform(
                            &[
                                f32::from(r) / 255.0,
                                f32::from(g) / 255.0,
                                f32::from(b) / 255.0,
                            ],
                            &mut expected,
                        )
                        .unwrap();
                    let got = rgb_table.apply([r, g, b]);
                    for (a, e) in got.iter().zip(expected) {
                        assert!((a - e).abs() <= CHANNEL_TABLE_BOUND);
                    }
                }
            }
        }

        // Verify stack-chunked `GrayTable::build` matches direct `moxcms`
        // bit-for-bit for all 256 codes.
        let gray_icc = gray_profile();
        let (gray_xf, _) = build_moxcms_transform(&gray_icc, true, None).unwrap();
        assert!(GrayTable::probe_finite(gray_xf.as_ref()));
        let gray_table = GrayTable::build(gray_xf.as_ref()).expect("gray profile builds table");
        for code in 0..=255_u8 {
            let mut expected = [0.0_f32; 3];
            gray_xf
                .transform(&[f32::from(code) / 255.0], &mut expected)
                .unwrap();
            assert_eq!(gray_table.apply(code), expected);
        }
    }

    /// Regression for PR #297 Round-9 review (Finding #1): a valid
    /// matrix-shaper RGB profile that fails the additivity probe (here mapping
    /// black to a nonzero value) must be detected strictly on the stack
    /// *before* reserving `RGB_TABLE_BYTES`, so a cold miss under a 4,000-byte
    /// budget reserves 0 bytes and falls back to `moxcms` identically to a
    /// warm negative cache hit.
    #[test]
    fn non_additive_matrix_shaper_skips_rgb_table_budget_reservation() {
        use super::{ChannelTable, IccTableCache, build_moxcms_transform};

        let mut profile = ColorProfile::new_srgb();
        profile.cicp = None;
        let nonzero_black_trc = ToneReprCurve::Lut((0_u16..16).map(|i| 4096 + i * 4095).collect());
        profile.red_trc = Some(nonzero_black_trc.clone());
        profile.green_trc = Some(nonzero_black_trc.clone());
        profile.blue_trc = Some(nonzero_black_trc);
        let non_additive_icc = profile.encode().unwrap();

        let (xf, extended_matrix_shaper) =
            build_moxcms_transform(&non_additive_icc, false, None).unwrap();
        assert!(extended_matrix_shaper);
        assert!(!ChannelTable::probe_additive(xf.as_ref()));

        let mut source = p3_grid(vec![7, 19, 202]);
        source.info.size = Size::new(100, 100).unwrap();
        source.info.rendered_icc = Some(non_additive_icc);

        let cache = IccTableCache::new();
        // Cold miss under a 4,000-byte budget (< RGB_TABLE_BYTES): probe runs
        // on the stack, fails, reserves 0 bytes, and caches the negative entry.
        let miss_budget = MemoryBudget::new(4_000);
        let miss_src = IccSrgbSource::new_with_cache(
            Box::new(source.clone()),
            Some(&cache),
            1,
            Some(&miss_budget),
        )
        .expect("non-additive profile falls back to moxcms without reserving 12 KiB");
        assert!(miss_src.conversion().rgb_table.is_none());
        assert!(miss_src.conversion().transformer.is_some());
        assert_eq!(miss_budget.snapshot().live, 0);
        assert_eq!(miss_budget.snapshot().peak, 0);

        // Warm negative cache hit behaves identically (0 bytes reserved).
        let hit_budget = MemoryBudget::new(4_000);
        let hit_src =
            IccSrgbSource::new_with_cache(Box::new(source), Some(&cache), 1, Some(&hit_budget))
                .expect("warm negative hit also reserves 0 bytes");
        assert!(hit_src.conversion().rgb_table.is_none());
        assert!(hit_src.conversion().transformer.is_some());
        assert_eq!(hit_budget.snapshot().live, 0);
        assert_eq!(hit_budget.snapshot().peak, 0);
    }

    /// Regression for PR #297 Round-10 review (Finding #1): when all cached
    /// tables in a bounded 2-entry cache are actively held by concurrent
    /// workers (`Arc::strong_count > 1`), inserting a third profile must skip
    /// caching rather than evicting the in-use tables and leaving the cache
    /// empty.
    #[test]
    fn in_use_cached_tables_are_retained_when_eviction_cannot_free_space() {
        use super::{
            CachedProfile, CachedTable, ChannelTable, IccTableCache, build_moxcms_transform,
            hash_profile_bytes, rgb_table_retained_bytes,
        };
        use std::sync::Arc;

        let p3_a = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
        let mut p3_b = p3_a.clone();
        p3_b[80] ^= 1;
        let mut p3_c = p3_a.clone();
        p3_c[80] ^= 2;

        let per_entry =
            p3_a.len() + rgb_table_retained_bytes() + std::mem::size_of::<CachedProfile>();
        let cap = 2 * per_entry;
        let cache = IccTableCache::with_max_bytes(cap);

        let (xf_a, _) = build_moxcms_transform(&p3_a, false, None).unwrap();
        let (xf_b, _) = build_moxcms_transform(&p3_b, false, None).unwrap();
        let (xf_c, _) = build_moxcms_transform(&p3_c, false, None).unwrap();
        let hash_a = hash_profile_bytes(&p3_a);
        let hash_b = hash_profile_bytes(&p3_b);
        let hash_c = hash_profile_bytes(&p3_c);

        // Hold both `arc_a` and `arc_b` (simulating two active batch workers).
        let arc_a = cache.insert_rgb(hash_a, &p3_a, ChannelTable::build(xf_a.as_ref()).unwrap());
        let arc_b = cache.insert_rgb(hash_b, &p3_b, ChannelTable::build(xf_b.as_ref()).unwrap());
        assert_eq!(cache.state.lock().unwrap().entries.len(), 2);
        assert_eq!(cache.live_bytes(), cap);

        // Inserting profile C cannot free space because both A and B are in
        // use; C is returned uncached while A and B both stay cached intact.
        let _arc_c = cache.insert_rgb(hash_c, &p3_c, ChannelTable::build(xf_c.as_ref()).unwrap());
        assert_eq!(cache.state.lock().unwrap().entries.len(), 2);
        assert_eq!(cache.live_bytes(), cap);
        assert!(cache.peak_bytes() <= cap);
        assert!(cache.get(hash_c, &p3_c).is_none());

        let hit_a = cache.get(hash_a, &p3_a).expect("profile A stays cached");
        let CachedTable::Rgb(Some(hit_arc_a)) = hit_a.table else {
            panic!("expected cached RGB table for A");
        };
        assert!(Arc::ptr_eq(&arc_a, &hit_arc_a));

        let hit_b = cache.get(hash_b, &p3_b).expect("profile B stays cached");
        let CachedTable::Rgb(Some(hit_arc_b)) = hit_b.table else {
            panic!("expected cached RGB table for B");
        };
        assert!(Arc::ptr_eq(&arc_b, &hit_arc_b));
        drop(hit_arc_a);
        drop(hit_arc_b);

        // Dropping B makes B idle/evictable while A remains in use; now C
        // evicts ONLY B and keeps active A cached.
        drop(arc_b);
        let arc_c2 = cache.insert_rgb(hash_c, &p3_c, ChannelTable::build(xf_c.as_ref()).unwrap());
        assert_eq!(cache.state.lock().unwrap().entries.len(), 2);
        assert!(cache.get(hash_b, &p3_b).is_none());
        let hit_a2 = cache.get(hash_a, &p3_a).expect("active A still cached");
        let CachedTable::Rgb(Some(hit_arc_a2)) = hit_a2.table else {
            panic!("expected cached RGB table for A");
        };
        assert!(Arc::ptr_eq(&arc_a, &hit_arc_a2));
        let hit_c = cache.get(hash_c, &p3_c).expect("C cached after evicting B");
        let CachedTable::Rgb(Some(hit_arc_c)) = hit_c.table else {
            panic!("expected cached RGB table for C");
        };
        assert!(Arc::ptr_eq(&arc_c2, &hit_arc_c));
        assert!(cache.peak_bytes() <= cap);
    }

    /// Regression for PR #297 Round-11 review (Findings #1 & #2):
    /// - Fallible `Vec<u8>` keys avoid panicking `Arc::from` copies and account
    ///   for exact `capacity()` even for non-8-byte-aligned ICC profile sizes
    ///   (e.g. 596-byte `paris-srgb.icc`).
    /// - `padded_arc_bytes<T>()` rounds `ArcInner<T>` up to
    ///   `align_of::<usize>().max(align_of::<T>())` so alignment padding is
    ///   never undercounted for any `T`.
    #[test]
    fn fallible_vec_key_and_padded_arc_bytes_account_exact_retained_bytes() {
        use super::{
            ARC_HEADER_BYTES, CachedProfile, ChannelTable, GrayTable, IccTableCache,
            build_moxcms_transform, gray_table_retained_bytes, hash_profile_bytes,
            padded_arc_bytes, rgb_table_retained_bytes, try_alloc_key,
        };

        // Alignment padding verification for small, normal, and over-aligned T.
        #[repr(align(32))]
        struct Aligned32 {
            _bytes: [u8; 32],
        }

        assert_eq!(
            padded_arc_bytes::<u8>(),
            ARC_HEADER_BYTES + std::mem::size_of::<usize>()
        );
        assert_eq!(padded_arc_bytes::<Aligned32>(), 64);
        assert!(rgb_table_retained_bytes().is_multiple_of(std::mem::align_of::<ChannelTable>()));
        assert!(gray_table_retained_bytes().is_multiple_of(std::mem::align_of::<GrayTable>()));

        // Fallible key helper allocates exact capacity without panicking.
        let paris = include_bytes!("../tests/fixtures/icc/paris-srgb.icc");
        assert!(!paris.len().is_multiple_of(8));
        let key = try_alloc_key(paris).expect("small key allocates");
        assert_eq!(key.as_slice(), paris);
        assert!(key.capacity() >= paris.len());

        // Exact cache byte accounting for a non-8-byte-aligned profile (596 B).
        let exact_cap =
            key.capacity() + rgb_table_retained_bytes() + std::mem::size_of::<CachedProfile>();
        let cache = IccTableCache::with_max_bytes(exact_cap);
        let (xf, _) = build_moxcms_transform(paris, false, None).unwrap();
        let hash = hash_profile_bytes(paris);
        let table = cache.insert_rgb(hash, paris, ChannelTable::build(xf.as_ref()).unwrap());
        {
            let state = cache.state.lock().unwrap();
            assert_eq!(state.entries.len(), 1);
            let entry_bytes = state.entries[0].retained_bytes();
            let vec_bytes = state.entries.capacity() * std::mem::size_of::<CachedProfile>();
            assert_eq!(cache.live_bytes(), entry_bytes + vec_bytes);
            assert_eq!(cache.peak_bytes(), exact_cap);
        }
        drop(table);
        assert!(cache.get(hash, paris).is_some());
    }
}

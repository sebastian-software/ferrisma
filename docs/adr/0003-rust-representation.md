# Map ownership and storage explicitly

- Status: accepted
- Created: 2026-10-31
- Updated: 2026-11-01

C++ pointer arithmetic, mutable visitors, and partially initialized storage
cannot be copied literally into safe Rust. Preserve their defined behavior with
explicit representations while retaining source algorithms.

- A bit reader borrows its bytes. Byte offsets replace pointers, including the
  source's end-minus-eight sentinel. Preserve its 64-bit refill buffer,
  zero-extension, deferred bounds failures, and explicit `close` checkpoint.
  Native Rust moves transfer the close obligation; no copying is provided.
  Debug destruction checks do not raise a second panic during Rust unwinding.
- `Status` maps to `Result<(), StatusCode>` and `StatusOr<T>` to
  `Result<T, StatusCode>`. Success is `Ok`; error codes retain upstream numeric
  values. Error diagnostics and crash-on-error instrumentation are not yet
  translated. Boolean control predicates remain booleans.
  In particular, `AllDefault` consumes a source Status as a branch condition:
  a failed read means false and field visitation continues. Returning early
  with `?` changes partial state and bit consumption; a differential regression
  covers this with an empty, truncated bundle.
- Field visitors mutate borrowed fields. Bundle encoding accepts a mutable
  bundle because upstream's const visitor casts away constness and may update
  `all_default`; do not clone complete bundles to hide that mutation.
- Serialized enums use raw `u32` wrappers implementing `FieldEnum`, so an
  invalid decoded value can be stored and then rejected in the same order as
  C++. Never create an invalid Rust enum discriminant. The templated Enum
  visitor helper is a free function to keep `Visitor` object-safe.
- ANS alias entries retain the eight-byte packed layout and original word
  lookup on little-endian targets. The unaligned word read is contained in
  the lookup with its layout, initialization, and borrow proof. Prefetch uses
  the source's Clang configuration: a T0 read hint on x86_64 with SSE2, disabled
  otherwise. Read top-level preprocessor guards as well as function bodies:
  Highway's SSE2 guard disables its apparent AArch64 builtin branch. Emitting
  an ARM hint would introduce an optimization absent from the reference.
- Entropy readers borrow their code tables for their entire lifetime and take
  a separate mutable bit-reader borrow for each operation. Aligned storage
  owns typed integer/alias slices; local proofs cover initialization, alignment,
  bounds, and exclusive mutable access. The LZ77 window starts initialized as
  required for safe Rust storage; defined decoder reads still follow the
  source's history/zero-distance rules. Keep all state and history counters
  32-bit with explicit wrapping, including the unhealthy-reader path for a
  wrapping decoded run length. Checkpoints restore entropy state and the same
  512 window entries; restoring the bit-reader position remains the caller's
  responsibility. Token bitfields use a private packed `u32` and accessors that
  retain truncation to the source's 31-bit context field. This does not yet
  establish a public C bitfield ABI.
- The inverse move-to-front transform initially translates the existing
  one-lane Highway scalar specialization. Its vector specializations remain
  separate future work; no SIMD or performance parity is inferred from it.
- ANS histogram lookup tables use `OnceLock` with initialized, contiguous
  Rust-owned arrays in place of C++ static initialization. Their formulas,
  numeric widths, and row order stay unchanged; initialization occurs on first
  use under the reference's default floating-point environment. This changes
  storage/initialization timing, not the search algorithm, and does not imply
  memory or performance parity. Keep `f64` rounding and `f32` cost-conversion
  points explicit. C++ `max_element`/`min_element` choose the first equal winner;
  preserve that with strict comparisons, rather than Rust's last-winner
  maximum iterator behavior. Internal writer traits express source Writer
  templates for actual output and bit-count-only estimation.
- Histogram clustering takes mutable input borrows because the C++ const
  interface writes a mutable entropy cache. Preserve those cache changes and
  first-winner ties. A Rust heap uses the original reversed `(cost, first,
  second, version)` ordering; maps retain canonical first-use reindexing. The
  scalar FastLog2f keeps the source coefficients as exact f32 bit patterns,
  integer range reduction, and separate polynomial multiplication/addition.
  Do not replace it with a library logarithm or fused operations. Disjoint
  slice borrows allow merging two histogram entries without cloning the source
  entry solely to satisfy the borrow checker.
- LZ77 retains the RLE, greedy ring-bucket, and optimal hash-chain algorithms,
  including their different special-distance collision winners and first-match
  ties. Owned vectors replace source pointers; initialized match records expose
  only fields assigned by the original dynamic program. The cost tables retain
  exact f32 bits. Upstream disables Clang auto-vectorization; on AArch64 the
  scalar histogram accumulation and final `6 - total_cost * inv_total`
  expression use fused rounding. The Rust mapping makes those rounding sites
  explicit. The x86_64 baseline has no FMA target; this is configuration-specific
  evidence, not cross-target bit parity.
  Differential regressions check all model float bits as well as chosen tokens.
- Huffman node/table pointers become indices into borrowed slices. Leaf nodes
  are sorted by population and descending symbol index, exactly as in C++;
  that comparator is a total order, so the standard library sort's internal
  implementation does not change ties. Temporary tables/arenas are initialized
  before borrowing, and only populated entries are observed. Keep signed
  16-bit node indices, unsigned count wrapping, and the original depth-limiting
  loop. Invalid internal preconditions panic rather than allowing invalid
  memory access; they do not add checks to accepted bitstream parsing. Failed
  parses preserve the source's table resize/mutation order. Symbol reads retain
  the caller's refill obligation and the reader's deferred bounds checkpoint.
- Padded storage preserves the minimum capacity, growth rule, padding, alignment,
  and rotating alignment groups. The initial backend uses Rust's allocator and
  initializes allocated bytes to zero so safe slices never expose uninitialized
  data. Additional initialization is an internal difference, not a promise of
  allocation or performance parity. Custom C allocator callbacks remain
  unported.
- Writer allotment parent pointers become indices in an owned stack. Preserve
  bit budgets, nested charging, and reclaim order. This adds Rust-managed stack
  storage; it does not change the bit-writing algorithm. The scoped callback
  receives reborrows of both the writer and optional accounting context, so
  nested writes can charge the same context without aliased mutable references.
- Complete entropy encoding retains optional output versus size-only writers,
  transposition, scalar token-cost counting, config candidate order, nested
  accounting, and prior histogram/context prefixes. The scalar MaxVectorSize
  value is four bytes; upstream adds that value to a count of u32 elements, and
  the Rust storage keeps the same padding. A newly encoded histogram writer is
  locally owned during table construction and appended even on error, retaining
  the source's observable partial state without cloning the encoding data.
  Streaming finalization accepts mutable codes because bundle visitors can
  mutate fields, following the existing bundle mapping above. The source's
  release-state fuzzer flag remains false; its debug-only global setter is not
  yet exposed. Ordinary Rust debug/release builds both match that reference.

- Color matrices use typed float/double arrays and a 16-byte aligned temporary,
  preserving source mixed-precision accumulation and inversion order. The
  translated callers have disjoint input/output storage; native borrows express
  that boundary. Ordinary Clang AArch64 expression contraction is explicit,
  including double results later narrowed through `copysignf`. C++ min/max
  operand order and unordered comparisons are retained instead of using Rust's
  different NaN rules. Opsin constants and the A2B cube keep constexpr float
  evaluation. ICC tag output remains byte-oriented, with the original profile
  date, aliasing of curve tags, padding, and unsigned MD5 wrapping. Rust's
  standard formatter supplies the source's default six-significant-digit `%g`
  output, with explicit fixed/exponent selection and C spellings; no C formatter
  is linked. Darwin omits a NaN sign, covered by an exact negative-NaN regression.
  These are reference-configuration results, not cross-platform or SIMD parity.

- Metadata names use byte vectors, because C++ strings accept arbitrary bytes
  even where comments call them UTF-8. Names and extra-channel vectors resize
  only on the source's reading branches; default visitation of a reused object
  must not clear them. Preserve unsigned field offsets before a failed visit,
  hidden color-storage synchronization, and mutations made before setter errors.
  The non-callback FromExternal color API deliberately ignores profile-generation
  failure, as the source does; field parsing propagates that failure instead.
  Static color encodings use OnceLock for the source's lazy static arrays.
  Their first-use timing is not performance evidence. Differential inputs to
  external C enums stay inside C++'s defined enum representation range; invalid
  decoded internal values still use the existing raw wrappers and validation.

Retain defined unsigned truncation and wrapping explicitly. Invalid programmer
preconditions must not cause undefined behavior in Rust. Record any discovered
source ambiguity before changing its interpretation. Do not add blanket unsafe
code or blanket lint suppression to imitate C++ syntax.

Frame and animation metadata pointers use optional shared borrows, preserving
the source's nullable cases and branch-local dereferences. Constructors assign
safe initial storage before running the original field visitor. Inactive pass
array entries have no defined initial C++ values; the oracle explicitly seeds
those entries as caller input before comparing state. Serialized fields and
the last-pass shift retain the source's mutation order. Chroma subsampling
keeps cached maxima when Set fails partway through: byte operands promote to
signed int before the difference converts to size_t. Unsigned rectangle
coordinates and group dimensions use pointer-width wrapping and the original
clamping, units, and padding; signed RectT and image-row views remain absent.
Filter validation promotes stored floats to double for the source's unsuffixed
`1e-8` threshold. Rounding that threshold to f32 incorrectly accepts the nearest
float below it; an exact differential rejection regression covers this case.
Bundle::AllDefault retains the release reference's accumulated predicate after
a failed visit; the source's JXL_DEBUG_ABORT is disabled in that configuration.
The enclosing CanEncode visit propagates its own validation error. Rust debug
and release both follow this release-state contract, with source crash-on-error
instrumentation still outside the translated boundary. The sigma regression
also checks this error path rather than converting it to a Rust panic.

Permutation coding retains the original Fenwick workspaces and their partial
mutations. Typed u16/u32 slices replace the source's permutation templates;
optional output borrows preserve validation without storing an order. A C++
integer loop can be empty when its starting index exceeds the end: retain that
with iterator take/skip rather than an invalid Rust subslice. Permutation token
counts still use unsigned subtraction and narrowing in that case. TOC vectors
clear, resize, and swap at the source's original checkpoints, including stale
permutation output when the flag is absent and unchanged offsets after failed
table reading. Rust slices correspond to non-null empty source spans; test
fixtures must not accidentally select a null reader's separate close behavior.

Plane/Image3 own the existing aligned allocation through an optional owner.
Sealed primitive numeric sample types guarantee valid byte representations,
no destructors, and alignment at most eight; locally proved typed slices borrow
initialized in-bounds storage. Mutable row/byte views require an exclusive
owner borrow. Physical row slices include padding, while copy/fill operations
use the valid width. Image3 preserves the source's first-plane stride and
per-plane failure order. Native moves transfer ownership; explicit image
cloning copies samples and transform vectors without implicit plane cloning.
Source overloads receive explicit Rust names where necessary.

Row sizing keeps the scalar source's four-byte MaxVectorSize, including rejection
of positive widths with eight-byte samples. Zero-sized planes have no allocation
context; channel reallocation inherits that null context and retains the source
failure rather than silently using a fresh allocator. Failed channel resize
keeps newly assigned logical dimensions and the old plane. C allocator callbacks
remain unported. Transform execution stays absent until its pixel kernels are
translated; initialized Rust storage for max_delta_error is an internal
difference from the source's indeterminate nonserialized member. Oracle callers
seed that member and observed pixel storage explicitly. No Rust sanitizer or
memory/performance parity is inferred from C++ instrumentation or these borrows.

Scalar RCT row pointers use disjoint mutable row borrows plus index maps for
input/output permutations. A private row-access trait expresses the source's
in-place and separate-buffer calls without aliased shared/mutable references:
load all three samples before storing any output. Whole-channel inverse moves
use swaps over the same three owners, preserving component tags without plane
copies or replacement allocations. Custom thread runners and arbitrary C
pointer alias contracts remain unported. The null-pool row order and the source
uint32 task-end narrowing stay explicit. Forward scalar i32 expressions retain
the source's nonoverflowing input requirement; inverse Highway Add/Sub use
explicit wrapping. Keep the in-place forward no-op's false Status distinct from
the separate-buffer and inverse no-ops' successful Status.

Prediction row pointers become a borrowed sample slice, a position, and a signed
row stride. The slice includes owned physical padding; relative reads retain
the source boundary/interior branches and cannot outlive storage. Weighted state
borrows its immutable header and owns the original two-row buffers and per-state
reciprocal table. Keep unsigned error sums, signed fixed-point operations,
narrowing, strict magnitude ties, and division-versus-shift rounding explicit.
Weighted header parameter reads synchronize their local u32 back to i32 only
after a successful visit, preserving partial failures.

FlatDecisionNode uses raw integer slots for the original unions, with typed
accessors and native-endian packing of the two i16 property indices. This avoids
inactive-union references and invalid enum discriminants while keeping the
24-byte layout and original twice-unrolled traversal. Rust initializes storage
that C++ leaves indeterminate, including PredictionResult::multiplier without a
tree; this gives that member no added non-tree meaning. The oracle must omit
such undefined observations rather than turning extra initialization into a
parity claim. Prediction-tree learning remains a separate boundary.

Tree parsing and tokenization retain breadth-first node/leaf indices, the source
integer narrowing points, iterative property-range validation, and error-time
mutation order. Rust VecDeque replaces the source queues; work stacks own their
records and retain the same traversal order. TreeLut keeps its fixed context
array; optional offset/multiplier arrays use vectors of exactly zero or 16,384
entries because stable Rust cannot express those conditional generic array
lengths. Rust initializes storage that the source leaves indeterminate. The
oracle seeds LUT storage before observing every entry and reads only the active
union members of flattened nodes. Header transforms resize only on reading,
including when an existing bundle receives defaults.

Review these mappings when adding the C ABI, custom allocators, nested bundle
extensions, SIMD, or concurrency. Image kernels must not assume that the current
pilot establishes their numeric or memory behavior.

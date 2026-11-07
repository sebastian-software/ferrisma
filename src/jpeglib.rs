// libjpeg-turbo 3.2.1, src/jpeglib.h:
//
// /*
//  * jpeglib.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1998, Thomas G. Lane.
//  * Modified 2002-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2009-2011, 2013-2014, 2016-2017, 2020, 2022-2024,
//              D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file defines the application interface for the JPEG library.
//  * Most applications using the library need only include this file,
//  * and perhaps jerror.h if they want to know the exact error codes.
//  */
//! Translation of `src/jpeglib.h` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! Translated for `JPEG_LIB_VERSION` 62, as `jconfig.h` defines it: the fields
//! and functions guarded by `JPEG_LIB_VERSION >= 70` or `>= 80` are absent.
//! The compression object, its parameter types (`jpeg_scan_info`,
//! `jpeg_destination_mgr`) and the 12- and 16-bit sample types are out of
//! scope (ADR 0001). Pointer types follow ADR 0005: a sample or block array is
//! one owned buffer with a list of row offsets ([`SampleArray`],
//! [`BlockArray`]), viewed through [`JSAMPROWS`] and [`JSAMPROWS_MUT`].
//!
//! `jpeg_decompress_struct` holds the fields that work package 1 can
//! represent on its own. A later package adds a field when it translates the
//! code that fills it: the module slots with their interfaces, and
//! `comp_info`, `cur_comp_info`, `coef_bits`, `colormap`,
//! `sample_range_limit` and `marker_list`, together with
//! `jpeg_component_info::dct_table`, `jpeg_marker_struct` and
//! `jpeg_marker_parser_method` (ADR 0005, "Module interfaces arrive with their
//! modules"). The function declarations of this header are translated in the
//! modules that define them.

use core::ffi::c_long;

use crate::jerror::{ErrorExit, jpeg_error_methods};
use crate::jmemmgr::my_memory_mgr;
use crate::jmorecfg::{
    C_BOOLEAN, C_INT, C_JDIMENSION, C_POINTER, CLayout, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT8,
    UINT16,
};

// upstream: src/jpeglib.h DCTSIZE
/// The basic DCT block is 8x8 samples.
pub const DCTSIZE: usize = 8;
// upstream: src/jpeglib.h DCTSIZE2
/// `DCTSIZE` squared; the number of elements in a block.
pub const DCTSIZE2: usize = 64;
// upstream: src/jpeglib.h NUM_QUANT_TBLS
/// Quantization tables are numbered 0..3.
pub const NUM_QUANT_TBLS: usize = 4;
// upstream: src/jpeglib.h NUM_HUFF_TBLS
/// Huffman tables are numbered 0..3.
pub const NUM_HUFF_TBLS: usize = 4;
// upstream: src/jpeglib.h NUM_ARITH_TBLS
/// Arith-coding tables are numbered 0..15.
pub const NUM_ARITH_TBLS: usize = 16;
// upstream: src/jpeglib.h MAX_COMPS_IN_SCAN
/// JPEG limit on the number of components in one scan.
pub const MAX_COMPS_IN_SCAN: usize = 4;
// upstream: src/jpeglib.h MAX_SAMP_FACTOR
/// JPEG limit on sampling factors.
pub const MAX_SAMP_FACTOR: i32 = 4;
// upstream: src/jpeglib.h D_MAX_BLOCKS_IN_MCU
/// The decompressor's limit on data units per MCU.
pub const D_MAX_BLOCKS_IN_MCU: usize = 10;

// upstream: src/jpeglib.h JBLOCK
/// One block of coefficients.
pub type JBLOCK = [JCOEF; DCTSIZE2];

/// `sizeof(JBLOCK)` on the target.
pub const SIZEOF_JBLOCK: usize = CLayout::of::<JCOEF>().array(DCTSIZE2).size;
/// `sizeof(JSAMPROW)`, a pointer, on the target.
pub const SIZEOF_JSAMPROW: usize = C_POINTER.size;
/// `sizeof(JBLOCKROW)`, a pointer, on the target.
pub const SIZEOF_JBLOCKROW: usize = C_POINTER.size;

/// A 2-D array of `T` as `alloc_sarray` or `alloc_barray` returns it: one
/// buffer with the row stride the allocator chose, and the offset of each row
/// in it, where C keeps an array of row pointers (ADR 0005, "Storage and row
/// lists"). Not an upstream item; [`SampleArray`] and [`BlockArray`] name its
/// two uses.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RowArray<T> {
    /// The storage of every row.
    pub buffer: Vec<T>,
    /// `rows[i]` is the offset of row `i` in `buffer`.
    pub rows: Vec<usize>,
}

/// A sample array from `alloc_sarray`; C's `JSAMPARRAY` with its storage.
pub type SampleArray = RowArray<JSAMPLE>;
/// A coefficient-block array from `alloc_barray`; C's `JBLOCKARRAY` with its
/// storage.
pub type BlockArray = RowArray<JBLOCK>;

impl<T> RowArray<T> {
    /// Row `i`: the storage from its offset to the end of the buffer, so that
    /// code may run past a row into the alignment padding as C does.
    #[must_use]
    pub fn row(&self, i: usize) -> &[T] {
        &self.buffer[self.rows[i]..]
    }

    /// Row `i`, mutably; see [`RowArray::row`].
    pub fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self.buffer[self.rows[i]..]
    }

    /// The whole array as input rows.
    #[must_use]
    pub fn as_rows(&self) -> RowList<'_, T> {
        RowList {
            storage: &self.buffer,
            rows: &self.rows,
        }
    }

    /// The whole array as output rows.
    pub fn as_rows_mut(&mut self) -> RowListMut<'_, '_, T> {
        RowListMut::Shared {
            storage: &mut self.buffer,
            rows: &self.rows,
        }
    }
}

/// Read access to rows of a [`RowArray`]: C's row-pointer array passed as
/// input (ADR 0005). Pointer arithmetic on the row array (`input_buf + n`)
/// is [`RowList::offset`]. Not an upstream item.
#[derive(Clone, Copy, Debug)]
pub struct RowList<'a, T> {
    /// The storage the rows lie in.
    pub storage: &'a [T],
    /// The offset of each row in `storage`.
    pub rows: &'a [usize],
}

/// Input sample rows, C's `JSAMPARRAY` as a parameter.
pub type JSAMPROWS<'a> = RowList<'a, JSAMPLE>;
/// Input block rows, C's `JBLOCKARRAY` as a parameter.
pub type JBLOCKROWS<'a> = RowList<'a, JBLOCK>;

impl<'a, T> RowList<'a, T> {
    /// The number of rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Whether there are no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Row `i`, from its start to the end of the storage.
    #[must_use]
    pub fn row(&self, i: usize) -> &'a [T] {
        &self.storage[self.rows[i]..]
    }

    /// The rows from row `n` on: C's `array + n`.
    #[must_use]
    pub fn offset(&self, n: usize) -> RowList<'a, T> {
        RowList {
            storage: self.storage,
            rows: &self.rows[n..],
        }
    }
}

/// Write access to rows: either rows of one [`RowArray`] (`Shared`), or rows
/// that an application passes and that may lie anywhere (`Separate`), as
/// `jpeg_read_scanlines` and `jpeg_read_raw_data` receive them (ADR 0005,
/// "Caller rows"). Not an upstream item.
#[derive(Debug)]
pub enum RowListMut<'a, 'r, T> {
    /// Rows at offsets into one storage.
    Shared {
        /// The storage the rows lie in.
        storage: &'a mut [T],
        /// The offset of each row in `storage`.
        rows: &'a [usize],
    },
    /// Separate row slices.
    Separate(&'a mut [&'r mut [T]]),
}

/// Output sample rows, C's `JSAMPARRAY` as an output parameter.
#[expect(non_camel_case_types, reason = "named after the C type")]
pub type JSAMPROWS_MUT<'a, 'r> = RowListMut<'a, 'r, JSAMPLE>;
/// Output block rows, C's `JBLOCKARRAY` as an output parameter.
#[expect(non_camel_case_types, reason = "named after the C type")]
pub type JBLOCKROWS_MUT<'a, 'r> = RowListMut<'a, 'r, JBLOCK>;

impl<'r, T: Copy> RowListMut<'_, 'r, T> {
    /// The number of rows.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            RowListMut::Shared { rows, .. } => rows.len(),
            RowListMut::Separate(rows) => rows.len(),
        }
    }

    /// Whether there are no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Row `i`; a `Shared` row extends to the end of the storage.
    #[must_use]
    pub fn row(&self, i: usize) -> &[T] {
        match self {
            RowListMut::Shared { storage, rows } => &storage[rows[i]..],
            RowListMut::Separate(rows) => rows[i],
        }
    }

    /// Row `i`, mutably.
    pub fn row_mut(&mut self, i: usize) -> &mut [T] {
        match self {
            RowListMut::Shared { storage, rows } => &mut storage[rows[i]..],
            RowListMut::Separate(rows) => rows[i],
        }
    }

    /// A shorter-lived view of the same rows, for passing on.
    pub fn reborrow(&mut self) -> RowListMut<'_, 'r, T> {
        match self {
            RowListMut::Shared { storage, rows } => RowListMut::Shared { storage, rows },
            RowListMut::Separate(rows) => RowListMut::Separate(rows),
        }
    }

    /// The rows from row `n` on: C's `array + n`.
    pub fn offset(&mut self, n: usize) -> RowListMut<'_, 'r, T> {
        match self {
            RowListMut::Shared { storage, rows } => RowListMut::Shared {
                storage,
                rows: &rows[n..],
            },
            RowListMut::Separate(rows) => RowListMut::Separate(&mut rows[n..]),
        }
    }

    /// Copies the first `count` elements of row `source` onto row `dest` of
    /// the same rows, as `jcopy_sample_rows` does when its input and output
    /// arrays are one array (ADR 0005, "Caller rows"). Rows that share storage
    /// may overlap, as `memcpy`'s callers allow upstream only for distinct
    /// rows; the copy behaves like `memmove`.
    pub fn copy_row(&mut self, source: usize, dest: usize, count: usize) {
        match self {
            RowListMut::Shared { storage, rows } => {
                let from = rows[source];
                storage.copy_within(from..from + count, rows[dest]);
            }
            RowListMut::Separate(rows) => {
                if source == dest {
                    return;
                }
                let (low, high) = rows.split_at_mut(source.max(dest));
                let (from, to): (&[T], &mut [T]) = if source < dest {
                    (low[source], high[0])
                } else {
                    (high[0], low[dest])
                };
                to[..count].copy_from_slice(&from[..count]);
            }
        }
    }
}

// upstream: src/jpeglib.h JQUANT_TBL
/// DCT coefficient quantization table.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JQUANT_TBL {
    /// The coefficient quantizers in natural array order.
    pub quantval: [UINT16; DCTSIZE2],
    /// Used only during compression; `false` in every new table.
    pub sent_table: bool,
}

impl Default for JQUANT_TBL {
    fn default() -> Self {
        JQUANT_TBL {
            quantval: [0; DCTSIZE2],
            sent_table: false,
        }
    }
}

/// `sizeof(JQUANT_TBL)` on the target.
pub const SIZEOF_JQUANT_TBL: usize =
    CLayout::structure(&[CLayout::of::<UINT16>().array(DCTSIZE2), C_BOOLEAN]).size;

// upstream: src/jpeglib.h JHUFF_TBL
/// Huffman coding table.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JHUFF_TBL {
    /// `bits[k]` is the number of symbols with codes of length `k` bits;
    /// `bits[0]` is unused.
    pub bits: [UINT8; 17],
    /// The symbols, in order of increasing code length.
    pub huffval: [UINT8; 256],
    /// Used only during compression; `false` in every new table.
    pub sent_table: bool,
}

impl Default for JHUFF_TBL {
    fn default() -> Self {
        JHUFF_TBL {
            bits: [0; 17],
            huffval: [0; 256],
            sent_table: false,
        }
    }
}

/// `sizeof(JHUFF_TBL)` on the target.
pub const SIZEOF_JHUFF_TBL: usize = CLayout::structure(&[
    CLayout::of::<UINT8>().array(17),
    CLayout::of::<UINT8>().array(256),
    C_BOOLEAN,
])
.size;

// upstream: src/jpeglib.h jpeg_component_info
/// Basic info about one component (color channel). `dct_table`, the IDCT's
/// private per-component storage, is added by work package 4.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct jpeg_component_info {
    /// Identifier for this component (0..255).
    pub component_id: i32,
    /// Its index in SOF or `cinfo->comp_info[]`.
    pub component_index: i32,
    /// Horizontal sampling factor (1..4).
    pub h_samp_factor: i32,
    /// Vertical sampling factor (1..4).
    pub v_samp_factor: i32,
    /// Quantization table selector (0..3).
    pub quant_tbl_no: i32,
    /// DC entropy table selector (0..3).
    pub dc_tbl_no: i32,
    /// AC entropy table selector (0..3).
    pub ac_tbl_no: i32,
    /// Component width in data units.
    pub width_in_blocks: JDIMENSION,
    /// Component height in data units.
    pub height_in_blocks: JDIMENSION,
    /// Size of a data unit in samples, after IDCT scaling.
    pub DCT_scaled_size: i32,
    /// Actual width in samples.
    pub downsampled_width: JDIMENSION,
    /// Actual height in samples.
    pub downsampled_height: JDIMENSION,
    /// Do we need the value of this component?
    pub component_needed: bool,
    /// Number of data units per MCU, horizontally.
    pub MCU_width: i32,
    /// Number of data units per MCU, vertically.
    pub MCU_height: i32,
    /// `MCU_width * MCU_height`.
    pub MCU_blocks: i32,
    /// MCU width in samples, `MCU_width * DCT_scaled_size`.
    pub MCU_sample_width: i32,
    /// Number of non-dummy data units across in the last MCU.
    pub last_col_width: i32,
    /// Number of non-dummy data units down in the last MCU.
    pub last_row_height: i32,
    /// Saved quantization table for the component; `None` if none yet saved.
    pub quant_table: Option<Box<JQUANT_TBL>>,
}

/// `sizeof(jpeg_component_info)` on the target, `dct_table` included.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_jpeg_component_info: usize = CLayout::structure(&[
    C_INT.array(7),
    C_JDIMENSION.array(2),
    C_INT,
    C_JDIMENSION.array(2),
    C_BOOLEAN,
    C_INT.array(6),
    C_POINTER,
    C_POINTER,
])
.size;

// upstream: src/jpeglib.h JCS_EXTENSIONS
pub const JCS_EXTENSIONS: i32 = 1;
// upstream: src/jpeglib.h JCS_ALPHA_EXTENSIONS
pub const JCS_ALPHA_EXTENSIONS: i32 = 1;

// upstream: src/jpeglib.h J_COLOR_SPACE
/// Known color spaces.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct J_COLOR_SPACE(pub u32);

#[expect(non_upper_case_globals, reason = "C enumerator names")]
impl J_COLOR_SPACE {
    /// Error or unspecified.
    pub const JCS_UNKNOWN: J_COLOR_SPACE = J_COLOR_SPACE(0);
    /// Monochrome.
    pub const JCS_GRAYSCALE: J_COLOR_SPACE = J_COLOR_SPACE(1);
    /// Red/green/blue as specified by the `RGB_*` macros.
    pub const JCS_RGB: J_COLOR_SPACE = J_COLOR_SPACE(2);
    /// Y/Cb/Cr (also known as YUV).
    pub const JCS_YCbCr: J_COLOR_SPACE = J_COLOR_SPACE(3);
    /// C/M/Y/K.
    pub const JCS_CMYK: J_COLOR_SPACE = J_COLOR_SPACE(4);
    /// Y/Cb/Cr/K.
    pub const JCS_YCCK: J_COLOR_SPACE = J_COLOR_SPACE(5);
    /// Red/green/blue.
    pub const JCS_EXT_RGB: J_COLOR_SPACE = J_COLOR_SPACE(6);
    /// Red/green/blue/x.
    pub const JCS_EXT_RGBX: J_COLOR_SPACE = J_COLOR_SPACE(7);
    /// Blue/green/red.
    pub const JCS_EXT_BGR: J_COLOR_SPACE = J_COLOR_SPACE(8);
    /// Blue/green/red/x.
    pub const JCS_EXT_BGRX: J_COLOR_SPACE = J_COLOR_SPACE(9);
    /// X/blue/green/red.
    pub const JCS_EXT_XBGR: J_COLOR_SPACE = J_COLOR_SPACE(10);
    /// X/red/green/blue.
    pub const JCS_EXT_XRGB: J_COLOR_SPACE = J_COLOR_SPACE(11);
    /// Red/green/blue/alpha.
    pub const JCS_EXT_RGBA: J_COLOR_SPACE = J_COLOR_SPACE(12);
    /// Blue/green/red/alpha.
    pub const JCS_EXT_BGRA: J_COLOR_SPACE = J_COLOR_SPACE(13);
    /// Alpha/blue/green/red.
    pub const JCS_EXT_ABGR: J_COLOR_SPACE = J_COLOR_SPACE(14);
    /// Alpha/red/green/blue.
    pub const JCS_EXT_ARGB: J_COLOR_SPACE = J_COLOR_SPACE(15);
    /// 5-bit red/6-bit green/5-bit blue, decompression only; out of scope
    /// (ADR 0001), but representable.
    pub const JCS_RGB565: J_COLOR_SPACE = J_COLOR_SPACE(16);
}

// upstream: src/jpeglib.h J_DCT_METHOD
/// DCT/IDCT algorithm options.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct J_DCT_METHOD(pub u32);

impl J_DCT_METHOD {
    /// Accurate integer method.
    pub const JDCT_ISLOW: J_DCT_METHOD = J_DCT_METHOD(0);
    /// Less accurate integer method (legacy feature).
    pub const JDCT_IFAST: J_DCT_METHOD = J_DCT_METHOD(1);
    /// Floating-point method (legacy feature).
    pub const JDCT_FLOAT: J_DCT_METHOD = J_DCT_METHOD(2);
}

// upstream: src/jpeglib.h JDCT_DEFAULT
pub const JDCT_DEFAULT: J_DCT_METHOD = J_DCT_METHOD::JDCT_ISLOW;
// upstream: src/jpeglib.h JDCT_FASTEST
pub const JDCT_FASTEST: J_DCT_METHOD = J_DCT_METHOD::JDCT_IFAST;

// upstream: src/jpeglib.h J_DITHER_MODE
/// Dithering options for decompression.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct J_DITHER_MODE(pub u32);

impl J_DITHER_MODE {
    /// No dithering.
    pub const JDITHER_NONE: J_DITHER_MODE = J_DITHER_MODE(0);
    /// Simple ordered dither.
    pub const JDITHER_ORDERED: J_DITHER_MODE = J_DITHER_MODE(1);
    /// Floyd-Steinberg error diffusion dither.
    pub const JDITHER_FS: J_DITHER_MODE = J_DITHER_MODE(2);
}

// upstream: src/jpeglib.h j_common_ptr
/// The common object is the decompression object while the encoder is out of
/// scope (ADR 0005, "The common object").
#[expect(non_camel_case_types, reason = "C type name")]
pub type j_common_ptr<'a, 'src> = &'a mut jpeg_decompress_struct<'src>;

// upstream: src/jpeglib.h j_decompress_ptr
#[expect(non_camel_case_types, reason = "C type name")]
pub type j_decompress_ptr<'a, 'src> = &'a mut jpeg_decompress_struct<'src>;

// upstream: src/jpeglib.h jpeg_decompress_struct
/// Master record for a decompression instance.
///
/// The common fields of `jpeg_common_fields` come first. `client_data` is
/// replaced by the context that each hook's trait object carries (ADR 0005,
/// "Public hooks"). See the module documentation for the fields that later
/// packages add.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[expect(clippy::struct_excessive_bools, reason = "C boolean fields")]
pub struct jpeg_decompress_struct<'src> {
    /// Error handler module.
    pub err: Box<jpeg_error_mgr<'src>>,
    /// Memory manager module; `None` before `jinit_memory_mgr` and after
    /// `jpeg_destroy`.
    pub mem: Option<Box<my_memory_mgr>>,
    /// Progress monitor, or `None` if none.
    pub progress: Option<Box<jpeg_progress_mgr<'src>>>,
    /// So common code can tell which is which; always `true` (ADR 0005).
    pub is_decompressor: bool,
    /// For checking call sequence validity.
    pub global_state: i32,

    /// Source of compressed data.
    pub src: Option<Box<jpeg_source_mgr<'src>>>,

    /// Nominal image width (from SOF marker).
    pub image_width: JDIMENSION,
    /// Nominal image height.
    pub image_height: JDIMENSION,
    /// Number of color components in JPEG image.
    pub num_components: i32,
    /// Colorspace of JPEG image.
    pub jpeg_color_space: J_COLOR_SPACE,

    /// Colorspace for output.
    pub out_color_space: J_COLOR_SPACE,
    /// Numerator of the fraction by which to scale image.
    pub scale_num: u32,
    /// Denominator of the fraction by which to scale image.
    pub scale_denom: u32,
    /// Image gamma wanted in output.
    pub output_gamma: f64,
    /// `true` = multiple output passes.
    pub buffered_image: bool,
    /// `true` = downsampled data wanted.
    pub raw_data_out: bool,
    /// IDCT algorithm selector.
    pub dct_method: J_DCT_METHOD,
    /// `true` = apply fancy upsampling.
    pub do_fancy_upsampling: bool,
    /// `true` = apply interblock smoothing.
    pub do_block_smoothing: bool,
    /// `true` = colormapped output wanted.
    pub quantize_colors: bool,
    /// Type of color dithering to use.
    pub dither_mode: J_DITHER_MODE,
    /// `true` = use two-pass color quantization.
    pub two_pass_quantize: bool,
    /// Max number of colors to use in created colormap.
    pub desired_number_of_colors: i32,
    /// Enable future use of 1-pass quantizer.
    pub enable_1pass_quant: bool,
    /// Enable future use of external colormap.
    pub enable_external_quant: bool,
    /// Enable future use of 2-pass quantizer.
    pub enable_2pass_quant: bool,

    /// Scaled image width.
    pub output_width: JDIMENSION,
    /// Scaled image height.
    pub output_height: JDIMENSION,
    /// Number of color components in `out_color_space`.
    pub out_color_components: i32,
    /// Number of color components returned.
    pub output_components: i32,
    /// Min recommended height of scanline buffer.
    pub rec_outbuf_height: i32,
    /// Number of colormap entries in use.
    pub actual_number_of_colors: i32,

    /// Row index of next scanline to be read from `jpeg_read_scanlines`.
    pub output_scanline: JDIMENSION,
    /// Number of SOS markers seen so far.
    pub input_scan_number: i32,
    /// Number of iMCU rows completed.
    pub input_iMCU_row: JDIMENSION,
    /// Nominal scan number being displayed.
    pub output_scan_number: i32,
    /// Number of iMCU rows read.
    pub output_iMCU_row: JDIMENSION,

    /// Coefficient quantization tables, or `None` if not defined.
    pub quant_tbl_ptrs: [Option<Box<JQUANT_TBL>>; NUM_QUANT_TBLS],
    /// DC Huffman coding tables, or `None` if not defined.
    pub dc_huff_tbl_ptrs: [Option<Box<JHUFF_TBL>>; NUM_HUFF_TBLS],
    /// AC Huffman coding tables, or `None` if not defined.
    pub ac_huff_tbl_ptrs: [Option<Box<JHUFF_TBL>>; NUM_HUFF_TBLS],

    /// Bits of precision in image data.
    pub data_precision: i32,

    /// `true` if `SOFn` specifies progressive mode.
    pub progressive_mode: bool,
    /// `true` = arithmetic coding, `false` = Huffman.
    pub arith_code: bool,
    /// L values for DC arith-coding tables.
    pub arith_dc_L: [UINT8; NUM_ARITH_TBLS],
    /// U values for DC arith-coding tables.
    pub arith_dc_U: [UINT8; NUM_ARITH_TBLS],
    /// Kx values for AC arith-coding tables.
    pub arith_ac_K: [UINT8; NUM_ARITH_TBLS],

    /// MCUs per restart interval, or 0 for no restart.
    pub restart_interval: u32,

    /// `true` iff a JFIF APP0 marker was found.
    pub saw_JFIF_marker: bool,
    /// JFIF version number, major part.
    pub JFIF_major_version: UINT8,
    /// JFIF version number, minor part.
    pub JFIF_minor_version: UINT8,
    /// JFIF code for pixel size units.
    pub density_unit: UINT8,
    /// Horizontal pixel density.
    pub X_density: UINT16,
    /// Vertical pixel density.
    pub Y_density: UINT16,
    /// `true` iff an Adobe APP14 marker was found.
    pub saw_Adobe_marker: bool,
    /// Color transform code from Adobe marker.
    pub Adobe_transform: UINT8,

    /// `true` = first samples are cosited.
    pub CCIR601_sampling: bool,

    /// Largest `h_samp_factor`.
    pub max_h_samp_factor: i32,
    /// Largest `v_samp_factor`.
    pub max_v_samp_factor: i32,
    /// Smallest `DCT_scaled_size` of any component.
    pub min_DCT_scaled_size: i32,
    /// Number of iMCU rows in image.
    pub total_iMCU_rows: JDIMENSION,

    /// Number of JPEG components in this scan.
    pub comps_in_scan: i32,
    /// Number of MCUs across the image.
    pub MCUs_per_row: JDIMENSION,
    /// Number of MCU rows in the image.
    pub MCU_rows_in_scan: JDIMENSION,
    /// Number of data units per MCU.
    pub blocks_in_MCU: i32,
    /// `MCU_membership[i]` is the index in `cur_comp_info` of the component
    /// owning the i'th data unit in an MCU.
    pub MCU_membership: [i32; D_MAX_BLOCKS_IN_MCU],
    /// Progressive JPEG spectral selection start.
    pub Ss: i32,
    /// Progressive JPEG spectral selection end.
    pub Se: i32,
    /// Progressive JPEG successive approximation, high bit.
    pub Ah: i32,
    /// Progressive JPEG successive approximation, low bit.
    pub Al: i32,

    /// Zero or the code of a JPEG marker that has been read from the data
    /// source but not yet processed.
    pub unread_marker: i32,
}

impl<'src> jpeg_decompress_struct<'src> {
    /// An object as an application holds it before `jpeg_create_decompress`:
    /// every field zero, empty or `false`, as after C's `memset`, with the
    /// error manager the application installs. `is_decompressor` is `true`
    /// from the start (ADR 0005, "The common object").
    #[must_use]
    pub fn new(err: Box<jpeg_error_mgr<'src>>) -> Self {
        jpeg_decompress_struct {
            err,
            mem: None,
            progress: None,
            is_decompressor: true,
            global_state: 0,
            src: None,
            image_width: 0,
            image_height: 0,
            num_components: 0,
            jpeg_color_space: J_COLOR_SPACE::default(),
            out_color_space: J_COLOR_SPACE::default(),
            scale_num: 0,
            scale_denom: 0,
            output_gamma: 0.0,
            buffered_image: false,
            raw_data_out: false,
            dct_method: J_DCT_METHOD::default(),
            do_fancy_upsampling: false,
            do_block_smoothing: false,
            quantize_colors: false,
            dither_mode: J_DITHER_MODE::default(),
            two_pass_quantize: false,
            desired_number_of_colors: 0,
            enable_1pass_quant: false,
            enable_external_quant: false,
            enable_2pass_quant: false,
            output_width: 0,
            output_height: 0,
            out_color_components: 0,
            output_components: 0,
            rec_outbuf_height: 0,
            actual_number_of_colors: 0,
            output_scanline: 0,
            input_scan_number: 0,
            input_iMCU_row: 0,
            output_scan_number: 0,
            output_iMCU_row: 0,
            quant_tbl_ptrs: Default::default(),
            dc_huff_tbl_ptrs: Default::default(),
            ac_huff_tbl_ptrs: Default::default(),
            data_precision: 0,
            progressive_mode: false,
            arith_code: false,
            arith_dc_L: [0; NUM_ARITH_TBLS],
            arith_dc_U: [0; NUM_ARITH_TBLS],
            arith_ac_K: [0; NUM_ARITH_TBLS],
            restart_interval: 0,
            saw_JFIF_marker: false,
            JFIF_major_version: 0,
            JFIF_minor_version: 0,
            density_unit: 0,
            X_density: 0,
            Y_density: 0,
            saw_Adobe_marker: false,
            Adobe_transform: 0,
            CCIR601_sampling: false,
            max_h_samp_factor: 0,
            max_v_samp_factor: 0,
            min_DCT_scaled_size: 0,
            total_iMCU_rows: 0,
            comps_in_scan: 0,
            MCUs_per_row: 0,
            MCU_rows_in_scan: 0,
            blocks_in_MCU: 0,
            MCU_membership: [0; D_MAX_BLOCKS_IN_MCU],
            Ss: 0,
            Se: 0,
            Ah: 0,
            Al: 0,
            unread_marker: 0,
        }
    }

    /// `cinfo->mem`.
    ///
    /// # Panics
    ///
    /// When the memory manager is absent, where C would dereference a null
    /// pointer (ADR 0005, "A method's own state").
    #[must_use]
    pub fn mem(&self) -> &my_memory_mgr {
        self.mem.as_deref().expect("cinfo->mem is NULL")
    }

    /// `cinfo->mem`, mutably.
    ///
    /// # Panics
    ///
    /// When the memory manager is absent; see [`jpeg_decompress_struct::mem`].
    pub fn mem_mut(&mut self) -> &mut my_memory_mgr {
        self.mem.as_deref_mut().expect("cinfo->mem is NULL")
    }
}

// upstream: src/jpeglib.h JMSG_LENGTH_MAX
/// Recommended size of the `format_message` buffer.
pub const JMSG_LENGTH_MAX: usize = 200;
// upstream: src/jpeglib.h JMSG_STR_PARM_MAX
pub const JMSG_STR_PARM_MAX: usize = 80;

/// `msg_parm`, the union of `int i[8]` and `char s[JMSG_STR_PARM_MAX]` in
/// `jpeg_error_mgr`: 80 bytes, with the integers read and written in native
/// byte order so that the union's aliasing is preserved (ADR 0005,
/// "Messages"). The C union has no type name.
#[expect(non_camel_case_types, reason = "named after the C field")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct jpeg_msg_parm(pub [u8; JMSG_STR_PARM_MAX]);

impl Default for jpeg_msg_parm {
    fn default() -> Self {
        jpeg_msg_parm([0; JMSG_STR_PARM_MAX])
    }
}

impl jpeg_msg_parm {
    /// `msg_parm.i[index]`.
    #[must_use]
    pub fn i(&self, index: usize) -> i32 {
        let start = index * 4;
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.0[start..start + 4]);
        i32::from_ne_bytes(bytes)
    }

    /// `msg_parm.i[index] = value`.
    pub fn set_i(&mut self, index: usize, value: i32) {
        let start = index * 4;
        self.0[start..start + 4].copy_from_slice(&value.to_ne_bytes());
    }

    /// `msg_parm.s`.
    #[must_use]
    pub fn s(&self) -> &[u8; JMSG_STR_PARM_MAX] {
        &self.0
    }

    /// `msg_parm.s`, mutably.
    pub fn s_mut(&mut self) -> &mut [u8; JMSG_STR_PARM_MAX] {
        &mut self.0
    }
}

/// A message table: entry `n` is the format string of message code `n`, or
/// `None` where C has a null pointer.
#[expect(non_camel_case_types, reason = "named after the C field")]
pub type jpeg_message_table<'a> = &'a [Option<&'a str>];

// upstream: src/jpeglib.h jpeg_error_mgr
/// Error handler object. The five methods are the trait object `methods`
/// (ADR 0005, "Public hooks"); the C fields follow.
#[expect(non_camel_case_types, reason = "C type name")]
pub struct jpeg_error_mgr<'src> {
    /// `error_exit`, `emit_message`, `output_message`, `format_message` and
    /// `reset_error_mgr`. `None` where C's pointers are null, as after
    /// `memset`; taken out for the duration of a call (see
    /// `jerror::call_error_exit`).
    pub methods: Option<Box<dyn jpeg_error_methods + 'src>>,
    /// The message ID code.
    pub msg_code: i32,
    /// The message parameters.
    pub msg_parm: jpeg_msg_parm,
    /// Max `msg_level` that will be displayed.
    pub trace_level: i32,
    /// Number of corrupt-data warnings.
    pub num_warnings: c_long,
    /// Library errors.
    pub jpeg_message_table: jpeg_message_table<'src>,
    /// Table contains strings `0..=last_jpeg_message`.
    pub last_jpeg_message: i32,
    /// Non-library errors, or `None`.
    pub addon_message_table: Option<jpeg_message_table<'src>>,
    /// Code for first string in addon table.
    pub first_addon_message: i32,
    /// Code for last string in addon table.
    pub last_addon_message: i32,
}

impl Default for jpeg_error_mgr<'_> {
    /// Every field zero or null, as after `memset`.
    fn default() -> Self {
        jpeg_error_mgr {
            methods: None,
            msg_code: 0,
            msg_parm: jpeg_msg_parm::default(),
            trace_level: 0,
            num_warnings: 0,
            jpeg_message_table: &[],
            last_jpeg_message: 0,
            addon_message_table: None,
            first_addon_message: 0,
            last_addon_message: 0,
        }
    }
}

/// `sizeof(struct jpeg_error_mgr)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_jpeg_error_mgr: usize = CLayout::structure(&[
    C_POINTER.array(5),
    C_INT,
    CLayout {
        size: JMSG_STR_PARM_MAX,
        align: C_INT.align,
    },
    C_INT,
    crate::jmorecfg::C_LONG,
    C_POINTER,
    C_INT,
    C_POINTER,
    C_INT,
    C_INT,
])
.size;

/// The method of a progress monitor (ADR 0005, "Public hooks"). Upstream has
/// no default; the application supplies it.
#[expect(non_camel_case_types, reason = "named after the C struct")]
pub trait jpeg_progress_methods {
    /// `progress_monitor`; may stop decoding with `Err`, where a C
    /// application could `longjmp`.
    ///
    /// # Errors
    ///
    /// Whatever the application returns.
    fn progress_monitor(&mut self, cinfo: &mut jpeg_decompress_struct<'_>)
    -> Result<(), ErrorExit>;
}

// upstream: src/jpeglib.h jpeg_progress_mgr
/// Progress monitor object.
#[expect(non_camel_case_types, reason = "C type name")]
pub struct jpeg_progress_mgr<'src> {
    /// `progress_monitor`, taken out for the duration of a call.
    pub methods: Option<Box<dyn jpeg_progress_methods + 'src>>,
    /// Work units completed in this pass.
    pub pass_counter: c_long,
    /// Total number of work units in this pass.
    pub pass_limit: c_long,
    /// Passes completed so far.
    pub completed_passes: i32,
    /// Total number of passes expected.
    pub total_passes: i32,
}

/// The buffer a source manager reads from (ADR 0005, "Source manager").
#[derive(Debug)]
pub enum InputBuffer<'src> {
    /// Bytes the application or the library lends, such as the input of
    /// `jpeg_mem_src` or its fake EOI.
    Borrowed(&'src [JOCTET]),
    /// Bytes a streaming source refills in place.
    Owned(Vec<JOCTET>),
}

impl InputBuffer<'_> {
    /// The bytes of the buffer.
    #[must_use]
    pub fn bytes(&self) -> &[JOCTET] {
        match self {
            InputBuffer::Borrowed(bytes) => bytes,
            InputBuffer::Owned(bytes) => bytes,
        }
    }
}

/// The methods of a data source (ADR 0005, "Public hooks"). Each returns
/// `Result`, so that a source can stop decoding where a C source could
/// `longjmp`; `fill_input_buffer` returns `Ok(false)` to suspend.
#[expect(non_camel_case_types, reason = "named after the C struct")]
pub trait jpeg_source_methods {
    /// `init_source`.
    ///
    /// # Errors
    ///
    /// A libjpeg error, recorded in `cinfo.err`.
    fn init_source(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit>;

    /// `fill_input_buffer`; `Ok(false)` suspends.
    ///
    /// # Errors
    ///
    /// A libjpeg error, recorded in `cinfo.err`.
    fn fill_input_buffer(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
    ) -> Result<bool, ErrorExit>;

    /// `skip_input_data`.
    ///
    /// # Errors
    ///
    /// A libjpeg error, recorded in `cinfo.err`.
    fn skip_input_data(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        num_bytes: c_long,
    ) -> Result<(), ErrorExit>;

    /// `resync_to_restart`.
    ///
    /// # Errors
    ///
    /// A libjpeg error, recorded in `cinfo.err`.
    fn resync_to_restart(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        desired: i32,
    ) -> Result<bool, ErrorExit>;

    /// `term_source`.
    ///
    /// # Errors
    ///
    /// A libjpeg error, recorded in `cinfo.err`.
    fn term_source(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) -> Result<(), ErrorExit>;
}

// upstream: src/jpeglib.h jpeg_source_mgr
/// Data source object for decompression.
#[expect(non_camel_case_types, reason = "C type name")]
pub struct jpeg_source_mgr<'src> {
    /// The offset of the next byte to read in `buffer`.
    pub next_input_byte: usize,
    /// Number of bytes remaining in `buffer`.
    pub bytes_in_buffer: usize,
    /// The bytes that `next_input_byte` points into.
    pub buffer: InputBuffer<'src>,
    /// `init_source`, `fill_input_buffer`, `skip_input_data`,
    /// `resync_to_restart` and `term_source`, taken out for the duration of a
    /// call.
    pub methods: Option<Box<dyn jpeg_source_methods + 'src>>,
}

// upstream: src/jpeglib.h JPOOL_PERMANENT
/// Lasts until master record is destroyed.
pub const JPOOL_PERMANENT: i32 = 0;
// upstream: src/jpeglib.h JPOOL_IMAGE
/// Lasts until done with image/datastream.
pub const JPOOL_IMAGE: i32 = 1;
// upstream: src/jpeglib.h JPOOL_NUMPOOLS
pub const JPOOL_NUMPOOLS: i32 = 2;

// upstream: src/jpeglib.h jvirt_barray_ptr
/// A virtual coefficient-block array: its index in the memory manager's
/// list of control blocks, which own the array (ADR 0005, "Virtual arrays").
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct jvirt_barray_ptr(pub usize);

/// A storage reservation that the memory manager makes where C's
/// `jpeg_get_small` or `jpeg_get_large` obtains memory: `true` when the
/// caller's storage could be reserved (ADR 0005, "Owned storage"). Not an
/// upstream item.
pub type Reservation<'a> = &'a mut dyn FnMut() -> bool;

// upstream: src/jpeglib.h jpeg_memory_mgr
/// Memory manager object. The method fields keep upstream's selection;
/// `request_virt_sarray` and `access_virt_sarray` are absent, because
/// virtual sample arrays serve only two-pass quantization (ADR 0005).
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug)]
pub struct jpeg_memory_mgr {
    /// Accounts for a small object; the storage is the caller's, reserved
    /// through the reservation (see `jmemmgr::alloc_small_box`).
    pub alloc_small: fn(j_common_ptr<'_, '_>, i32, usize, Reservation<'_>) -> Result<(), ErrorExit>,
    /// Accounts for a large object; the storage is the caller's, reserved
    /// through the reservation (see `jmemmgr::alloc_large_vec`).
    pub alloc_large: fn(j_common_ptr<'_, '_>, i32, usize, Reservation<'_>) -> Result<(), ErrorExit>,
    /// Allocates a 2-D sample array.
    pub alloc_sarray:
        fn(j_common_ptr<'_, '_>, i32, JDIMENSION, JDIMENSION) -> Result<SampleArray, ErrorExit>,
    /// Allocates a 2-D coefficient-block array.
    pub alloc_barray:
        fn(j_common_ptr<'_, '_>, i32, JDIMENSION, JDIMENSION) -> Result<BlockArray, ErrorExit>,
    /// Requests a virtual 2-D coefficient-block array.
    pub request_virt_barray: fn(
        j_common_ptr<'_, '_>,
        i32,
        bool,
        JDIMENSION,
        JDIMENSION,
        JDIMENSION,
    ) -> Result<jvirt_barray_ptr, ErrorExit>,
    /// Allocates the in-memory buffers of the unrealized virtual arrays.
    pub realize_virt_arrays: fn(j_common_ptr<'_, '_>) -> Result<(), ErrorExit>,
    /// Makes rows of a virtual block array accessible and returns the index
    /// of the first requested row in the array's in-memory buffer, the
    /// offset that C adds to `mem_buffer`.
    pub access_virt_barray: fn(
        j_common_ptr<'_, '_>,
        jvirt_barray_ptr,
        JDIMENSION,
        JDIMENSION,
        bool,
    ) -> Result<usize, ErrorExit>,
    /// Releases all objects belonging to a pool.
    pub free_pool: fn(j_common_ptr<'_, '_>, i32) -> Result<(), ErrorExit>,
    /// Closes up shop entirely.
    pub self_destruct: fn(j_common_ptr<'_, '_>) -> Result<(), ErrorExit>,
    /// Limit on memory allocation for this JPEG object.
    pub max_memory_to_use: c_long,
    /// Maximum allocation request accepted by `alloc_large`.
    pub max_alloc_chunk: c_long,
}

// upstream: src/jpeglib.h JPEG_SUSPENDED
/// Suspended due to lack of input data.
pub const JPEG_SUSPENDED: i32 = 0;
// upstream: src/jpeglib.h JPEG_HEADER_OK
/// Found valid image datastream.
pub const JPEG_HEADER_OK: i32 = 1;
// upstream: src/jpeglib.h JPEG_HEADER_TABLES_ONLY
/// Found valid table-specs-only datastream.
pub const JPEG_HEADER_TABLES_ONLY: i32 = 2;
// upstream: src/jpeglib.h JPEG_REACHED_SOS
/// Reached start of new scan.
pub const JPEG_REACHED_SOS: i32 = 1;
// upstream: src/jpeglib.h JPEG_REACHED_EOI
/// Reached end of image.
pub const JPEG_REACHED_EOI: i32 = 2;
// upstream: src/jpeglib.h JPEG_ROW_COMPLETED
/// Completed one iMCU row.
pub const JPEG_ROW_COMPLETED: i32 = 3;
// upstream: src/jpeglib.h JPEG_SCAN_COMPLETED
/// Completed last iMCU row of a scan.
pub const JPEG_SCAN_COMPLETED: i32 = 4;

// upstream: src/jpeglib.h JPEG_RST0
/// RST0 marker code.
pub const JPEG_RST0: i32 = 0xD0;
// upstream: src/jpeglib.h JPEG_EOI
/// EOI marker code.
pub const JPEG_EOI: i32 = 0xD9;
// upstream: src/jpeglib.h JPEG_APP0
/// APP0 marker code.
pub const JPEG_APP0: i32 = 0xE0;
// upstream: src/jpeglib.h JPEG_COM
/// COM marker code.
pub const JPEG_COM: i32 = 0xFE;

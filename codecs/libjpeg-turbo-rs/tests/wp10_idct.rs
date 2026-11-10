//! WP10: the x86-64 SSE2 IDCT kernels without AVX2 versions, against the
//! oracle's asm kernels (`simd/x86_64/jidctfst-sse2.asm`,
//! `jidctflt-sse2.asm` and `jidctred-sse2.asm`).
//!
//! The probe `tools/oracle/probes/wp10/idct.c` turns random quantization
//! tables into each kernel's multiplier table with `jddctmgr.c`'s
//! `start_pass` and calls the asm kernel by its symbol on random coefficient
//! blocks, at random output columns of padded output rows that the decoder's
//! column steps reach. This test draws the same inputs with
//! `support/kernels.rs`, builds the tables with the port's `start_pass`,
//! calls the port's kernels directly and compares the hashes of the tables
//! and of every output buffer, over its whole stride.

#![cfg(all(with_simd, target_arch = "x86_64"))]

#[path = "support/kernels.rs"]
mod kernels;
#[path = "support/oracle.rs"]
mod oracle;

use jpeg::jdapimin::{jpeg_create_decompress, jpeg_destroy_decompress};
use jpeg::jddctmgr::{jinit_inverse_dct, multiplier_table};
use jpeg::jerror::jpeg_std_error;
use jpeg::jidctflt_sse2::jsimd_idct_float_sse2;
use jpeg::jidctfst_sse2::jsimd_idct_ifast_sse2;
use jpeg::jidctred_sse2::{jsimd_idct_2x2_sse2, jsimd_idct_4x4_sse2};
use jpeg::jmemmgr::{alloc_small_box, alloc_small_vec};
use jpeg::jmorecfg::{JCOEF, JDIMENSION};
use jpeg::jpegint::idct_simd_ptr;
use jpeg::jpeglib::{
    DCTSIZE, DCTSIZE2, J_DCT_METHOD, JBLOCK, JPOOL_IMAGE, JQUANT_TBL, SIZEOF_JQUANT_TBL,
    SIZEOF_jpeg_component_info, jpeg_component_info, jpeg_decompress_struct, jpeg_error_mgr,
};
use kernels::{Rng, hash, rows, stride};
use oracle::Oracle;

/// `MAX_BLOCKS` of the probe: the widest output buffer, in blocks.
const MAX_BLOCKS: u32 = 40;

/// A kernel of the probe's `-kernel` option.
#[derive(Clone, Copy)]
struct Kernel {
    name: &'static str,
    kernel: idct_simd_ptr,
    method: J_DCT_METHOD,
    size: usize,
}

const KERNELS: [Kernel; 4] = [
    Kernel {
        name: "ifast",
        kernel: jsimd_idct_ifast_sse2,
        method: J_DCT_METHOD::JDCT_IFAST,
        size: DCTSIZE,
    },
    Kernel {
        name: "float",
        kernel: jsimd_idct_float_sse2,
        method: J_DCT_METHOD::JDCT_FLOAT,
        size: DCTSIZE,
    },
    Kernel {
        name: "4x4",
        kernel: jsimd_idct_4x4_sse2,
        method: J_DCT_METHOD::JDCT_ISLOW,
        size: 4,
    },
    Kernel {
        name: "2x2",
        kernel: jsimd_idct_2x2_sse2,
        method: J_DCT_METHOD::JDCT_ISLOW,
        size: 2,
    },
];

/// The probe's `AnyCoef`: a `JCOEF` of any 16-bit value.
fn any_coef(rng: &mut Rng) -> JCOEF {
    u16::try_from(rng.next() & 0xFFFF).unwrap().cast_signed()
}

/// A random index or count below `n`, as the probe's `Wp10Rand`.
fn rand(rng: &mut Rng, n: usize) -> usize {
    usize::try_from(rng.rand(u32::try_from(n).unwrap())).unwrap()
}

/// The probe's `MakeBlock`.
fn make_block(rng: &mut Rng, block: &mut JBLOCK, qtbl: &JQUANT_TBL) {
    const EDGES: [JCOEF; 5] = [-32768, 32767, -1, 0, 1];
    block.fill(0);
    match rng.rand(6) {
        0 => {
            for coef in block.iter_mut() {
                *coef = any_coef(rng);
            }
        }
        1 => {
            for coef in block.iter_mut() {
                *coef = if rng.rand(2) != 0 {
                    any_coef(rng)
                } else {
                    EDGES[rand(rng, 5)]
                };
            }
        }
        2 => {
            for coef in &mut block[..DCTSIZE] {
                *coef = any_coef(rng);
            }
            if rng.rand(2) != 0 {
                let k = DCTSIZE + rand(rng, DCTSIZE2 - DCTSIZE);
                block[k] = any_coef(rng);
            }
        }
        3 => {
            let half = rand(rng, 2);
            for (k, coef) in block.iter_mut().enumerate() {
                if k < DCTSIZE || (k % DCTSIZE) / 4 != half {
                    *coef = any_coef(rng);
                }
            }
        }
        4 => {
            for coef in block.iter_mut() {
                if rng.rand(8) == 0 {
                    *coef = any_coef(rng);
                }
            }
        }
        _ => {
            for (k, coef) in block.iter_mut().enumerate() {
                let lim = 1024 / (1 + k / DCTSIZE + k % DCTSIZE);
                if k == 0 || rng.rand(4) == 0 {
                    let value = i32::try_from(rand(rng, 2 * lim + 1)).unwrap()
                        - i32::try_from(lim).unwrap();
                    *coef = JCOEF::try_from(value / i32::from(qtbl.quantval[k])).unwrap();
                }
            }
        }
    }
}

/// The multiplier table that the port's `start_pass` builds for `qtbl` and
/// `kernel`, as the probe's decompression object does.
fn multiplier_table_for(kernel: &Kernel, qtbl: &JQUANT_TBL) -> multiplier_table {
    let mut err = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut err);
    let mut cinfo = jpeg_decompress_struct::new(err);
    jpeg_create_decompress(&mut cinfo).unwrap();
    cinfo.data_precision = 8;
    cinfo.num_components = 1;
    cinfo.dct_method = kernel.method;
    let mut comp_info = alloc_small_vec(
        &mut cinfo,
        JPOOL_IMAGE,
        SIZEOF_jpeg_component_info,
        1,
        jpeg_component_info::default(),
    )
    .unwrap();
    comp_info[0].component_needed = true;
    comp_info[0].DCT_scaled_size = i32::try_from(kernel.size).unwrap();
    cinfo.comp_info = Some(comp_info);
    let qtbl = alloc_small_box(&mut cinfo, JPOOL_IMAGE, SIZEOF_JQUANT_TBL, qtbl.clone()).unwrap();
    cinfo.comp_info_mut()[0].quant_table = Some(qtbl);
    jinit_inverse_dct(&mut cinfo).unwrap();
    let start_pass = cinfo.idct().start_pass;
    start_pass(&mut cinfo).unwrap();
    let table = cinfo.comp_info()[0].dct_table.as_deref().unwrap().clone();
    jpeg_destroy_decompress(&mut cinfo).unwrap();
    table
}

/// The bytes of the table that the probe hashes: 64 16-bit multipliers, or
/// 64 floats for the float kernel.
fn table_bytes(kernel: &Kernel, table: &multiplier_table) -> Vec<u8> {
    if kernel.method == J_DCT_METHOD::JDCT_FLOAT {
        (0..DCTSIZE2)
            .flat_map(|i| table.float_array(i).to_ne_bytes())
            .collect()
    } else {
        table
            .islow_array()
            .iter()
            .flat_map(|m| m.to_ne_bytes())
            .collect()
    }
}

/// Runs the probe's sequence on the port's kernel: the `table=` and `out=`
/// values.
fn run(kernel: &Kernel, seed: u64, tables: usize, blocks: usize, qmax: u32) -> (String, String) {
    let mut rng = Rng::new(seed);
    let mut table_hashes = Vec::new();
    let mut out_hashes = Vec::new();
    let mut block: JBLOCK = [0; DCTSIZE2];
    for _ in 0..tables {
        let mut qtbl = JQUANT_TBL::default();
        for value in &mut qtbl.quantval {
            *value = u16::try_from(1 + rng.rand(qmax)).unwrap();
        }
        let table = multiplier_table_for(kernel, &qtbl);
        table_hashes.push(format!("{:016x}", hash(&table_bytes(kernel, &table))));

        for _ in 0..blocks {
            let width = (1 + rand(&mut rng, MAX_BLOCKS as usize)) * kernel.size;
            let mut output = rows(kernel.size, width);
            kernels::fill_rows(&mut rng, &mut output);
            let output_col = rand(&mut rng, width / kernel.size) * kernel.size;
            make_block(&mut rng, &mut block, &qtbl);
            // SAFETY: the kernels are SSE2 code, and SSE2 is enabled on every
            // x86-64 target.
            unsafe {
                (kernel.kernel)(
                    &table,
                    &block,
                    output.as_rows_mut(),
                    JDIMENSION::try_from(output_col).unwrap(),
                );
            }
            assert_eq!(output.buffer.len(), kernel.size * stride(width));
            out_hashes.push(format!("{:016x}", hash(&output.buffer)));
        }
    }
    (table_hashes.join(","), out_hashes.join(","))
}

/// Compares the port's kernel with the probe for one run.
fn compare(oracle: &Oracle, kernel: &Kernel, seed: u64, tables: usize, blocks: usize, qmax: u32) {
    let args = [
        "-kernel".to_owned(),
        kernel.name.to_owned(),
        "-seed".to_owned(),
        seed.to_string(),
        "-tables".to_owned(),
        tables.to_string(),
        "-blocks".to_owned(),
        blocks.to_string(),
        "-qmax".to_owned(),
        qmax.to_string(),
    ];
    let fields = oracle
        .probe("wp10", "idct")
        .run(&args, &[])
        .success()
        .fields();
    let (table, out) = run(kernel, seed, tables, blocks, qmax);
    assert_eq!(
        fields.get("table"),
        Some(table.as_str()),
        "{} seed {seed} qmax {qmax}: multiplier tables differ",
        kernel.name
    );
    let expected = fields.get("out").expect("out=");
    if let Some(index) = expected
        .split(',')
        .zip(out.split(','))
        .position(|(e, a)| e != a)
    {
        panic!(
            "{} seed {seed} qmax {qmax}: block {index} (table {}, block {}) differs",
            kernel.name,
            index / blocks,
            index % blocks
        );
    }
    assert_eq!(expected, out, "{} seed {seed} qmax {qmax}", kernel.name);
}

/// Every kernel, with 8-bit and 16-bit quantization tables, small ones
/// (that keep image-like blocks in range) and full-range blocks.
fn check(test: &str, kernel_name: &str) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let kernel = KERNELS.iter().find(|k| k.name == kernel_name).unwrap();
    for (seed, qmax) in [(1, 1), (2, 16), (3, 255), (4, 65535), (5, 255), (6, 65535)] {
        compare(&oracle, kernel, seed, 25, 200, qmax);
    }
}

#[test]
fn ifast() {
    check("ifast", "ifast");
}

#[test]
fn float() {
    check("float", "float");
}

#[test]
fn reduced_4x4() {
    check("reduced_4x4", "4x4");
}

#[test]
fn reduced_2x2() {
    check("reduced_2x2", "2x2");
}

//! The kernels against the translated C kernels (`cdef_tmpl.c`) on the
//! inputs of the original's checkasm test (`tests/checkasm/cdef.c`):
//! the same buffers, fill types, strengths, dampings, directions and
//! edges, many random draws. A development check; the gate is checkasm
//! against the original's assembly and the whole-image decodes.

use core::ffi::{c_int, c_uint};

use super::*;

struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        // xorshift32
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

/// `init_tmp` of the checkasm test.
fn init_tmp<P>(rng: &mut Rng, buf: &mut [P], bitdepth_max: u16, from: impl Fn(u16) -> P) {
    let fill_type = rng.next() & 7;
    for v in buf {
        let r = rng.next() as u16;
        *v = from(match fill_type {
            0 => r & 1,
            1 => bitdepth_max - (r & 1),
            _ => r & bitdepth_max,
        });
    }
}

type CFilter8 = unsafe extern "C" fn(*mut u8, isize, *const [u8; 2], *const u8, *const u8, c_int, c_int, c_int, c_int, c_uint);
type CFilter16 =
    unsafe extern "C" fn(*mut u16, isize, *const [u16; 2], *const u16, *const u16, c_int, c_int, c_int, c_int, c_uint, c_int);

/// The C kernels of the translation: the DSP table with the CPU flags masked off.
fn c_kernels() -> (
    [CFilter8; 3],
    unsafe extern "C" fn(*const u8, isize, *mut c_uint) -> c_int,
    [CFilter16; 3],
    unsafe extern "C" fn(*const u16, isize, *mut c_uint, c_int) -> c_int,
) {
    use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
    let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    unsafe {
        dav1d_init_cpu();
        dav1d_set_cpu_flags_mask(0);
        let mut c8: crate::b8::cdef_tmpl::Dav1dCdefDSPContext = core::mem::zeroed();
        crate::b8::cdef_tmpl::dav1d_cdef_dsp_init_8bpc(&mut c8);
        let mut c16: crate::b16::cdef_tmpl::Dav1dCdefDSPContext = core::mem::zeroed();
        crate::b16::cdef_tmpl::dav1d_cdef_dsp_init_16bpc(&mut c16);
        dav1d_set_cpu_flags_mask(!0);
        (
            c8.fb.map(|f| core::mem::transmute::<_, CFilter8>(f.unwrap())),
            core::mem::transmute(c8.dir.unwrap()),
            c16.fb.map(|f| core::mem::transmute::<_, CFilter16>(f.unwrap())),
            core::mem::transmute(c16.dir.unwrap()),
        )
    }
}

const SIZES: [(usize, usize); 3] = [(8, 8), (4, 8), (4, 4)];

fn check_kernels(name: &str, draws: usize) {
    let (c_fb8, c_dir8, c_fb16, c_dir16) = c_kernels();
    let (k8, k16) = (kernels_8bpc().unwrap(), kernels_16bpc().unwrap());
    let (fb8, dir_8bpc, fb16, dir_16bpc) = (k8.fb, k8.dir, k16.fb, k16.dir);
    let mut rng = Rng(0x2545_f491);
    for draw in 0..draws {
        for (slot, &(w, _h)) in SIZES.iter().enumerate() {
            for s in 1..=3 {
                let dir = (draw % 8) as c_int;
                let edges = (rng.next() & 15) as c_uint;
                for bitdepth_max in [0xffu16, 0x3ff, 0xfff] {
                    let bitdepth_min_8 = (bitdepth_max as u32 + 1).ilog2() - 8;
                    let pri = if s & 2 != 0 { ((1 + rng.next() % 15) << bitdepth_min_8) as c_int } else { 0 };
                    let sec = if s & 1 != 0 { (1 << (rng.next() % 3 + bitdepth_min_8)) as c_int } else { 0 };
                    let damping = (3 + (rng.next() & 3) + bitdepth_min_8) as c_int
                        - (w == 4 || rng.next() & 1 != 0) as c_int;
                    let mut src = [0u16; 16 * 10 + 16];
                    let mut top = [0u16; 16 * 2 + 16];
                    let mut bot = [0u16; 16 * 2 + 16];
                    let mut left = [[0u16; 2]; 8];
                    init_tmp(&mut rng, &mut src, bitdepth_max, |v| v);
                    init_tmp(&mut rng, &mut top, bitdepth_max, |v| v);
                    init_tmp(&mut rng, &mut bot, bitdepth_max, |v| v);
                    init_tmp(&mut rng, left.as_flattened_mut(), bitdepth_max, |v| v);
                    let context = || {
                        format!("{name} {w}x{} bitdepth_max {bitdepth_max:#x} strength {pri}:{sec} dir {dir} damping {damping} edges {edges:#x}", SIZES[slot].1)
                    };
                    if bitdepth_max == 0xff {
                        let narrow = |b: &[u16]| b.iter().map(|&v| v as u8).collect::<Vec<u8>>();
                        let (mut a, mut b) = (narrow(&src), narrow(&src));
                        let (top, bot) = (narrow(&top), narrow(&bot));
                        let left: Vec<[u8; 2]> = left.iter().map(|p| [p[0] as u8, p[1] as u8]).collect();
                        unsafe {
                            let call = |f: CFilter8, buf: &mut [u8]| {
                                f(buf.as_mut_ptr().add(8), 16, left.as_ptr(), top.as_ptr().add(8), bot.as_ptr().add(8), pri, sec, dir, damping, edges)
                            };
                            call(c_fb8[slot], &mut a);
                            call(fb8[slot], &mut b);
                        }
                        assert_eq!(a, b, "{}", context());
                    } else {
                        let (mut a, mut b) = (src, src);
                        unsafe {
                            let call = |f: CFilter16, buf: &mut [u16]| {
                                f(buf.as_mut_ptr().add(8), 32, left.as_ptr(), top.as_ptr().add(8), bot.as_ptr().add(8), pri, sec, dir, damping, edges, bitdepth_max as c_int)
                            };
                            call(c_fb16[slot], &mut a);
                            call(fb16[slot], &mut b);
                        }
                        assert_eq!(a, b, "{}", context());
                    }
                }
            }
        }
        for bitdepth_max in [0xffu16, 0x3ff, 0xfff] {
            let mut src = [0u16; 64];
            init_tmp(&mut rng, &mut src, bitdepth_max, |v| v);
            let (mut var_c, mut var_v) = (0, 0);
            let (dir_c, dir_v) = unsafe {
                if bitdepth_max == 0xff {
                    let src: Vec<u8> = src.iter().map(|&v| v as u8).collect();
                    (c_dir8(src.as_ptr(), 8, &mut var_c), dir_8bpc(src.as_ptr(), 8, &mut var_v))
                } else {
                    let m = bitdepth_max as c_int;
                    (c_dir16(src.as_ptr(), 16, &mut var_c, m), dir_16bpc(src.as_ptr(), 16, &mut var_v, m))
                }
            };
            assert_eq!((dir_c, var_c), (dir_v, var_v), "{name} dir, bitdepth_max {bitdepth_max:#x}, {src:?}");
        }
    }
}

#[test]
fn kernels_match_the_c_kernels() {
    if kernels_8bpc().is_some() {
        check_kernels("cdef", 3000);
    } else {
        eprintln!("cdef: level not available, skipped");
    }
}

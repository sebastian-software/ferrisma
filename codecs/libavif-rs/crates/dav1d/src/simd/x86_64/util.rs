//! What the rebuilt x86-64 kernels share (engine ADR-0044 D2, libavif-rs
//! ADR 0062); the counterpart of `simd/arm64/util.rs`.
use core::arch::x86_64::*;
use core::mem::MaybeUninit;

/// Not in the original: writes zero to every byte of `*buf` and returns the
/// pointer to it. For stack buffers that the C code or the assembly leaves
/// uninitialized and whose kernels load whole blocks of elements: zeroed
/// whole before anything else touches it, no load from the buffer can find
/// an element without a value, whichever elements the kernels load. The
/// buffer is reached only through the returned pointer afterwards.
///
/// The stores are 32-byte stores (`vmovdqu [p], ymm`): one per 32 bytes,
/// and one more that ends at the buffer's last byte and overlaps the one
/// before when the size is not a multiple of 32. The size is a constant, so
/// small buffers are a few stores in a row and large ones a loop of stores.
///
/// # Safety
/// The CPU supports AVX2 (the callers are AVX2 kernels). `buf` is
/// exclusively borrowed and at least 32 bytes long (checked at compile
/// time).
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn zeroed<T>(buf: &mut MaybeUninit<T>) -> *mut T {
    const { assert!(core::mem::size_of::<T>() >= 32) };
    let bytes = core::mem::size_of::<T>();
    let p = buf.as_mut_ptr() as *mut u8;
    let zero = _mm256_setzero_si256();
    let mut at = 0;
    while at + 32 <= bytes {
        _mm256_storeu_si256(p.add(at).cast(), zero);
        at += 32;
    }
    if at < bytes {
        _mm256_storeu_si256(p.add(bytes - 32).cast(), zero);
    }
    buf.as_mut_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every byte is zero afterwards, for sizes that are and are not
    /// multiples of the store.
    #[test]
    fn zeroed_clears_every_byte() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }
        macro_rules! check {
            ($($n:expr),*) => {$(
                let mut buf = MaybeUninit::<[u8; $n]>::uninit();
                // SAFETY: fully written first, so the bytes may be read; AVX2 was detected.
                unsafe {
                    buf.as_mut_ptr().cast::<u8>().write_bytes(0xa5, $n);
                    let p = zeroed(&mut buf);
                    assert!((*p).iter().all(|&b| b == 0), "size {}", $n);
                }
            )*};
        }
        check!(32, 33, 63, 64, 65, 95, 96, 352, 673);
    }
}

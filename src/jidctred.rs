// libjpeg-turbo 3.2.1, src/jidctred.c:
//
// /*
//  * jidctred.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1998, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015, 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains inverse-DCT routines that produce reduced-size output:
//  * either 4x4, 2x2, or 1x1 samples from an 8x8 DCT block.
//  *
//  * The implementation is based on the Loeffler, Ligtenberg and Moschytz (LL&M)
//  * algorithm used in jidctint.c.  We simply replace each 8-to-8 1-D IDCT step
//  * with an 8-to-4 step that produces the four averages of two adjacent outputs
//  * (or an 8-to-2 step producing two averages of four outputs, for 2x2 output).
//  * These steps were derived by computing the corresponding values at the end
//  * of the normal LL&M code, then simplifying as much as possible.
//  *
//  * 1x1 is trivial: just take the DC coefficient divided by 8.
//  *
//  * See jidctint.c for additional comments.
//  */
//! Translation of `src/jidctred.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).

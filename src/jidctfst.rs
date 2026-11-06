// libjpeg-turbo 3.2.1, src/jidctfst.c:
//
// /*
//  * jidctfst.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1998, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015, 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains a fast, not so accurate integer implementation of the
//  * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
//  * must also perform dequantization of the input coefficients.
//  *
//  * A 2-D IDCT can be done by 1-D IDCT on each column followed by 1-D IDCT
//  * on each row (or vice versa, but it's more convenient to emit a row at
//  * a time).  Direct algorithms are also available, but they are much more
//  * complex and seem not to be any faster when reduced to code.
//  *
//  * This implementation is based on Arai, Agui, and Nakajima's algorithm for
//  * scaled DCT.  Their original paper (Trans. IEICE E-71(11):1095) is in
//  * Japanese, but the algorithm is described in the Pennebaker & Mitchell
//  * JPEG textbook (see REFERENCES section in file README.ijg).  The following
//  * code is based directly on figure 4-8 in P&M.
//  * While an 8-point DCT cannot be done in less than 11 multiplies, it is
//  * possible to arrange the computation so that many of the multiplies are
//  * simple scalings of the final outputs.  These multiplies can then be
//  * folded into the multiplications or divisions by the JPEG quantization
//  * table entries.  The AA&N method leaves only 5 multiplies and 29 adds
//  * to be done in the DCT itself.
//  * The primary disadvantage of this method is that with fixed-point math,
//  * accuracy is lost due to imprecise representation of the scaled
//  * quantization values.  The smaller the quantization table entry, the less
//  * precise the scaled value, so this implementation does worse with high-
//  * quality-setting files than with low-quality ones.
//  */
//! Translation of `src/jidctfst.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).

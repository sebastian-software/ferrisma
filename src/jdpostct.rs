// libjpeg-turbo 3.2.1, src/jdpostct.c:
//
// /*
//  * jdpostct.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022-2024, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the decompression postprocessing controller.
//  * This controller manages the upsampling, color conversion, and color
//  * quantization/reduction steps; specifically, it controls the buffering
//  * between upsample/color conversion and color quantization/reduction.
//  *
//  * If no color quantization/reduction is required, then this module has no
//  * work to do, and it just hands off to the upsample/color conversion code.
//  * An integrated upsample/convert/quantize process would replace this module
//  * entirely.
//  */
//! Translation of `src/jdpostct.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).

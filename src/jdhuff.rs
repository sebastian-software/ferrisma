// libjpeg-turbo 3.2.1, src/jdhuff.h:
//
// /*
//  * jdhuff.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010-2011, 2015-2016, 2021, D. R. Commander.
//  * Copyright (C) 2018, Matthias Räncker.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains declarations for Huffman entropy decoding routines
//  * that are shared between the sequential decoder (jdhuff.c), the progressive
//  * decoder (jdphuff.c), and the lossless decoder (jdlhuff.c).  No other modules
//  * need to see these.
//  */
// libjpeg-turbo 3.2.1, src/jdhuff.c:
//
// /*
//  * jdhuff.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2009-2011, 2016, 2018-2019, 2022, D. R. Commander.
//  * Copyright (C) 2018, Matthias Räncker.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains Huffman entropy decoding routines.
//  *
//  * Much of the complexity here has to do with supporting input suspension.
//  * If the data source module demands suspension, we want to be able to back
//  * up to the start of the current MCU.  To do this, we copy state variables
//  * into local working storage, and update them back to the permanent
//  * storage only upon successful completion of an MCU.
//  *
//  * NOTE: All referenced figures are from
//  * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
//  */
//! Translation of `src/jdhuff.h` and `src/jdhuff.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 3 (Entropy decoding).

// Copyright (c) 2006, 2008 Edward Rosten
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  *Redistributions of source code must retain the above copyright
//   notice, this list of conditions and the following disclaimer.
//
//  *Redistributions in binary form must reproduce the above copyright
//   notice, this list of conditions and the following disclaimer in the
//   documentation and/or other materials provided with the distribution.
//
//  *Neither the name of the University of Cambridge nor the names of
//   its contributors may be used to endorse or promote products derived
//   from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER
// OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// clang-format off
/*This is mechanically generated code*/

// Mechanically translated from upstream/aom/third_party/fastfeat/fast_9.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xy {
    pub x: core::ffi::c_int,
    pub y: core::ffi::c_int,
}
pub type byte = core::ffi::c_uchar;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
unsafe extern "C-unwind" fn fast9_corner_score(
    mut p: *const byte,
    mut pixel: *const core::ffi::c_int,
    mut bstart: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut bmin: core::ffi::c_int = bstart;
    let mut bmax: core::ffi::c_int = 255 as core::ffi::c_int;
    let mut b: core::ffi::c_int = (bmax + bmin) / 2 as core::ffi::c_int;
    loop {
        let mut cb: core::ffi::c_int = *p as core::ffi::c_int + b;
        let mut c_b: core::ffi::c_int = *p as core::ffi::c_int - b;
        if *p.offset(*pixel.offset(0 as core::ffi::c_int as isize) as isize)
            as core::ffi::c_int > cb
        {
            if *p.offset(*pixel.offset(1 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int > cb
            {
                if *p.offset(*pixel.offset(2 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(3 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(
                                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(
                                                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                        ) as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block = 0;
                                                                    } else {
                                                                        current_block = 1;
                                                                    }
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(
                                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(
                                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                        ) as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block = 0;
                                                                    } else {
                                                                        current_block = 1;
                                                                    }
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(
                                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(
                                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(
                                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(
                                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(
                                                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block = 0;
                                                                } else if (*p
                                                                    .offset(
                                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else if (*p
                                                                .offset(
                                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if *p
                                .offset(
                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else if (*p
                                                                .offset(
                                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else if (*p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if *p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else if (*p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else if (*p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if (*p
                        .offset(*pixel.offset(3 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if *p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else if (*p
                                                        .offset(
                                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else if (*p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if *p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if (*p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else if (*p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else if (*p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if (*p
                    .offset(*pixel.offset(2 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if *p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if (*p
                        .offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else if (*p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if *p
                    .offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if (*p
                    .offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else if (*p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else if (*p.offset(*pixel.offset(1 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int) < c_b
            {
                if *p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if (*p
                    .offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else if *p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int > cb
            {
                if *p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if *p
                            .offset(
                                *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else if (*p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int) < c_b
            {
                if (*p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else {
                current_block = 1;
            }
        } else if (*p.offset(*pixel.offset(0 as core::ffi::c_int as isize) as isize)
            as core::ffi::c_int) < c_b
        {
            if *p.offset(*pixel.offset(1 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int > cb
            {
                if *p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if (*p
                    .offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else if (*p.offset(*pixel.offset(1 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int) < c_b
            {
                if *p.offset(*pixel.offset(2 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else if *p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if (*p
                        .offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if (*p
                    .offset(*pixel.offset(2 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if *p.offset(*pixel.offset(3 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else if *p
                                                        .offset(
                                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else if *p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if (*p
                        .offset(*pixel.offset(3 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if *p
                            .offset(
                                *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else if *p
                                                                .offset(
                                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else if *p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if *p
                                .offset(
                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(
                                                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    current_block = 0;
                                                                } else if *p
                                                                    .offset(
                                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(
                                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else if (*p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(
                                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(
                                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(
                                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                        ) as core::ffi::c_int > cb
                                                                    {
                                                                        current_block = 0;
                                                                    } else {
                                                                        current_block = 1;
                                                                    }
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(
                                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(
                                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(
                                                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                        ) as core::ffi::c_int > cb
                                                                    {
                                                                        current_block = 0;
                                                                    } else {
                                                                        current_block = 1;
                                                                    }
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(
                                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                    ) as core::ffi::c_int > cb
                                                                {
                                                                    current_block = 0;
                                                                } else {
                                                                    current_block = 1;
                                                                }
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(
                                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else if *p
                                                                .offset(
                                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int > cb
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(
                                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                                ) as core::ffi::c_int) < c_b
                                                            {
                                                                current_block = 0;
                                                            } else {
                                                                current_block = 1;
                                                            }
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if *p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else if *p
                                                            .offset(
                                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else if *p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(
                                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int > cb
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(
                                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                                            ) as core::ffi::c_int) < c_b
                                                        {
                                                            current_block = 0;
                                                        } else {
                                                            current_block = 1;
                                                        }
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if *p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else if *p
                                                        .offset(
                                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else if *p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(
                                                            *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int > cb
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if (*p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(
                                                            *pixel.offset(9 as core::ffi::c_int as isize) as isize,
                                                        ) as core::ffi::c_int) < c_b
                                                    {
                                                        current_block = 0;
                                                    } else {
                                                        current_block = 1;
                                                    }
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if *p
                    .offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else if *p
                                                    .offset(
                                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(
                                                        *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int > cb
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else if (*p
                    .offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(
                                                        *pixel.offset(8 as core::ffi::c_int as isize) as isize,
                                                    ) as core::ffi::c_int) < c_b
                                                {
                                                    current_block = 0;
                                                } else {
                                                    current_block = 1;
                                                }
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else if *p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int > cb
            {
                if *p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else if *p
                                                .offset(
                                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else if *p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(
                                                    *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int > cb
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else if (*p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int) < c_b
            {
                if (*p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(6 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(
                                                    *pixel.offset(7 as core::ffi::c_int as isize) as isize,
                                                ) as core::ffi::c_int) < c_b
                                            {
                                                current_block = 0;
                                            } else {
                                                current_block = 1;
                                            }
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else {
                current_block = 1;
            }
        } else if *p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
            as core::ffi::c_int > cb
        {
            if *p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int > cb
            {
                if *p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(*pixel.offset(6 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(1 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else if *p
                                            .offset(
                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if *p
                                        .offset(
                                            *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if *p
                                    .offset(
                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if *p
                                .offset(
                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if *p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if *p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int > cb
                        {
                            if *p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int > cb
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else {
                current_block = 1;
            }
        } else if (*p.offset(*pixel.offset(7 as core::ffi::c_int as isize) as isize)
            as core::ffi::c_int) < c_b
        {
            if (*p.offset(*pixel.offset(8 as core::ffi::c_int as isize) as isize)
                as core::ffi::c_int) < c_b
            {
                if (*p.offset(*pixel.offset(9 as core::ffi::c_int as isize) as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(*pixel.offset(6 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(5 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(4 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(3 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(2 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(1 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else if (*p
                                            .offset(
                                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else if (*p
                                        .offset(
                                            *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else if (*p
                                    .offset(
                                        *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else if (*p
                                .offset(
                                    *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else if (*p
                            .offset(
                                *pixel.offset(10 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else if (*p
                        .offset(*pixel.offset(10 as core::ffi::c_int as isize) as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p
                            .offset(
                                *pixel.offset(11 as core::ffi::c_int as isize) as isize,
                            ) as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(
                                    *pixel.offset(12 as core::ffi::c_int as isize) as isize,
                                ) as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(
                                        *pixel.offset(13 as core::ffi::c_int as isize) as isize,
                                    ) as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(
                                            *pixel.offset(14 as core::ffi::c_int as isize) as isize,
                                        ) as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(
                                                *pixel.offset(15 as core::ffi::c_int as isize) as isize,
                                            ) as core::ffi::c_int) < c_b
                                        {
                                            current_block = 0;
                                        } else {
                                            current_block = 1;
                                        }
                                    } else {
                                        current_block = 1;
                                    }
                                } else {
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                        } else {
                            current_block = 1;
                        }
                    } else {
                        current_block = 1;
                    }
                } else {
                    current_block = 1;
                }
            } else {
                current_block = 1;
            }
        } else {
            current_block = 1;
        }
        match current_block {
            1 => {
                bmax = b;
            }
            _ => {
                bmin = b;
            }
        }
        if bmin == bmax - 1 as core::ffi::c_int || bmin == bmax {
            return bmin;
        }
        b = (bmin + bmax) / 2 as core::ffi::c_int;
    };
}
unsafe extern "C-unwind" fn make_offsets(
    mut pixel: *mut core::ffi::c_int,
    mut row_stride: core::ffi::c_int,
) {
    *pixel.offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
        + row_stride * 3 as core::ffi::c_int;
    *pixel.offset(1 as core::ffi::c_int as isize) = 1 as core::ffi::c_int
        + row_stride * 3 as core::ffi::c_int;
    *pixel.offset(2 as core::ffi::c_int as isize) = 2 as core::ffi::c_int
        + row_stride * 2 as core::ffi::c_int;
    *pixel.offset(3 as core::ffi::c_int as isize) = 3 as core::ffi::c_int
        + row_stride * 1 as core::ffi::c_int;
    *pixel.offset(4 as core::ffi::c_int as isize) = 3 as core::ffi::c_int
        + row_stride * 0 as core::ffi::c_int;
    *pixel.offset(5 as core::ffi::c_int as isize) = 3 as core::ffi::c_int
        + row_stride * -(1 as core::ffi::c_int);
    *pixel.offset(6 as core::ffi::c_int as isize) = 2 as core::ffi::c_int
        + row_stride * -(2 as core::ffi::c_int);
    *pixel.offset(7 as core::ffi::c_int as isize) = 1 as core::ffi::c_int
        + row_stride * -(3 as core::ffi::c_int);
    *pixel.offset(8 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
        + row_stride * -(3 as core::ffi::c_int);
    *pixel.offset(9 as core::ffi::c_int as isize) = -(1 as core::ffi::c_int)
        + row_stride * -(3 as core::ffi::c_int);
    *pixel.offset(10 as core::ffi::c_int as isize) = -(2 as core::ffi::c_int)
        + row_stride * -(2 as core::ffi::c_int);
    *pixel.offset(11 as core::ffi::c_int as isize) = -(3 as core::ffi::c_int)
        + row_stride * -(1 as core::ffi::c_int);
    *pixel.offset(12 as core::ffi::c_int as isize) = -(3 as core::ffi::c_int)
        + row_stride * 0 as core::ffi::c_int;
    *pixel.offset(13 as core::ffi::c_int as isize) = -(3 as core::ffi::c_int)
        + row_stride * 1 as core::ffi::c_int;
    *pixel.offset(14 as core::ffi::c_int as isize) = -(2 as core::ffi::c_int)
        + row_stride * 2 as core::ffi::c_int;
    *pixel.offset(15 as core::ffi::c_int as isize) = -(1 as core::ffi::c_int)
        + row_stride * 3 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fast9_score(
    mut i: *const byte,
    mut stride: core::ffi::c_int,
    mut corners: *const xy,
    mut num_corners: core::ffi::c_int,
    mut b: core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut scores: *mut core::ffi::c_int = malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(num_corners as size_t),
    ) as *mut core::ffi::c_int;
    let mut n: core::ffi::c_int = 0;
    let mut pixel: [core::ffi::c_int; 16] = [0; 16];
    if scores.is_null() {
        return 0 as *mut core::ffi::c_int;
    }
    make_offsets(pixel.as_mut_ptr(), stride);
    n = 0 as core::ffi::c_int;
    while n < num_corners {
        *scores.offset(n as isize) = fast9_corner_score(
            i
                .offset(((*corners.offset(n as isize)).y * stride) as isize)
                .offset((*corners.offset(n as isize)).x as isize),
            pixel.as_mut_ptr() as *const core::ffi::c_int,
            b,
        );
        n += 1;
    }
    return scores;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fast9_detect(
    mut im: *const byte,
    mut xsize: core::ffi::c_int,
    mut ysize: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut b: core::ffi::c_int,
    mut ret_num_corners: *mut core::ffi::c_int,
) -> *mut xy {
    let mut num_corners: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ret_corners: *mut xy = 0 as *mut xy;
    let mut rsize: core::ffi::c_int = 512 as core::ffi::c_int;
    let mut pixel: [core::ffi::c_int; 16] = [0; 16];
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    ret_corners = malloc(
        (::core::mem::size_of::<xy>() as size_t).wrapping_mul(rsize as size_t),
    ) as *mut xy;
    if ret_corners.is_null() {
        return 0 as *mut xy;
    }
    make_offsets(pixel.as_mut_ptr(), stride);
    y = 3 as core::ffi::c_int;
    while y < ysize - 3 as core::ffi::c_int {
        let mut current_block_218: u64;
        x = 3 as core::ffi::c_int;
        while x < xsize - 3 as core::ffi::c_int {
            let mut p: *const byte = im.offset((y * stride) as isize).offset(x as isize);
            let mut cb: core::ffi::c_int = *p as core::ffi::c_int + b;
            let mut c_b: core::ffi::c_int = *p as core::ffi::c_int - b;
            if *p.offset(pixel[0 as core::ffi::c_int as usize] as isize)
                as core::ffi::c_int > cb
            {
                if *p.offset(pixel[1 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(pixel[2 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        if (*p
                                                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                            as core::ffi::c_int) < c_b
                                                                        {
                                                                            current_block_218 = 2;
                                                                        } else {
                                                                            current_block_218 = 3;
                                                                        }
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if *p
                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        if (*p
                                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                            as core::ffi::c_int) < c_b
                                                                        {
                                                                            current_block_218 = 2;
                                                                        } else {
                                                                            current_block_218 = 3;
                                                                        }
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if *p
                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else if *p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else if (*p
                                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else if *p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else if (*p
                                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if *p
                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else if *p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else if (*p
                                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else if (*p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if *p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else if *p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else if (*p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else if (*p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if (*p
                            .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if *p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else if *p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else if (*p
                                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else if (*p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if (*p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if *p
                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else if *p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if (*p
                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else if (*p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else if (*p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if (*p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if (*p.offset(pixel[2 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else if *p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if (*p
                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else if (*p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if (*p
                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else if *p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if (*p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else if (*p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if (*p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else if (*p.offset(pixel[1 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int) < c_b
                {
                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else if *p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if (*p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else if (*p
                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else if *p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if *p
                                .offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else if (*p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else if (*p
                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else {
                    current_block_218 = 3;
                }
            } else if (*p.offset(pixel[0 as core::ffi::c_int as usize] as isize)
                as core::ffi::c_int) < c_b
            {
                if *p.offset(pixel[1 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else if *p
                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if (*p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else if (*p.offset(pixel[1 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int) < c_b
                {
                    if *p.offset(pixel[2 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else if *p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if *p
                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if (*p
                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if (*p.offset(pixel[2 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else if *p
                                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else if *p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if *p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if (*p
                            .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else if *p
                                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else if *p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if *p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else if *p
                                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if *p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else if (*p
                                                    .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    if (*p
                                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int) < c_b
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if (*p
                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if *p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        if *p
                                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                            as core::ffi::c_int > cb
                                                                        {
                                                                            current_block_218 = 2;
                                                                        } else {
                                                                            current_block_218 = 3;
                                                                        }
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        if *p
                                                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                            as core::ffi::c_int > cb
                                                                        {
                                                                            current_block_218 = 2;
                                                                        } else {
                                                                            current_block_218 = 3;
                                                                        }
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p
                                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    if *p
                                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                        as core::ffi::c_int > cb
                                                                    {
                                                                        current_block_218 = 2;
                                                                    } else {
                                                                        current_block_218 = 3;
                                                                    }
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else if *p
                                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int > cb
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                if (*p
                                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                                    as core::ffi::c_int) < c_b
                                                                {
                                                                    current_block_218 = 2;
                                                                } else {
                                                                    current_block_218 = 3;
                                                                }
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if *p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else if *p
                                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else if *p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            if *p
                                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int > cb
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            if (*p
                                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                                as core::ffi::c_int) < c_b
                                                            {
                                                                current_block_218 = 2;
                                                            } else {
                                                                current_block_218 = 3;
                                                            }
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if *p
                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else if *p
                                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else if *p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if *p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        if *p
                                                            .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int > cb
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if (*p
                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        if (*p
                                                            .offset(pixel[9 as core::ffi::c_int as usize] as isize)
                                                            as core::ffi::c_int) < c_b
                                                        {
                                                            current_block_218 = 2;
                                                        } else {
                                                            current_block_218 = 3;
                                                        }
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else if *p
                                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else if *p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    if *p
                                                        .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int > cb
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else if (*p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    if (*p
                                                        .offset(pixel[8 as core::ffi::c_int as usize] as isize)
                                                        as core::ffi::c_int) < c_b
                                                    {
                                                        current_block_218 = 2;
                                                    } else {
                                                        current_block_218 = 3;
                                                    }
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p.offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else if *p
                                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else if *p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                if *p
                                                    .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int > cb
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else if (*p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p.offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[6 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                if (*p
                                                    .offset(pixel[7 as core::ffi::c_int as usize] as isize)
                                                    as core::ffi::c_int) < c_b
                                                {
                                                    current_block_218 = 2;
                                                } else {
                                                    current_block_218 = 3;
                                                }
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else {
                    current_block_218 = 3;
                }
            } else if *p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                as core::ffi::c_int > cb
            {
                if *p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int > cb
                {
                    if *p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int > cb
                    {
                        if *p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p.offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p.offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p.offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p.offset(pixel[1 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else if *p
                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if *p
                                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if *p
                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if *p
                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if *p
                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if *p
                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int > cb
                        {
                            if *p.offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int > cb
                            {
                                if *p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int > cb
                                {
                                    if *p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int > cb
                                    {
                                        if *p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int > cb
                                        {
                                            if *p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int > cb
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else {
                    current_block_218 = 3;
                }
            } else if (*p.offset(pixel[7 as core::ffi::c_int as usize] as isize)
                as core::ffi::c_int) < c_b
            {
                if (*p.offset(pixel[8 as core::ffi::c_int as usize] as isize)
                    as core::ffi::c_int) < c_b
                {
                    if (*p.offset(pixel[9 as core::ffi::c_int as usize] as isize)
                        as core::ffi::c_int) < c_b
                    {
                        if (*p.offset(pixel[6 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p.offset(pixel[5 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[4 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[3 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[2 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[1 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else if (*p
                                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else if (*p
                                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else if (*p
                                        .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else if (*p
                                    .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else if (*p
                                .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else if (*p
                            .offset(pixel[10 as core::ffi::c_int as usize] as isize)
                            as core::ffi::c_int) < c_b
                        {
                            if (*p
                                .offset(pixel[11 as core::ffi::c_int as usize] as isize)
                                as core::ffi::c_int) < c_b
                            {
                                if (*p
                                    .offset(pixel[12 as core::ffi::c_int as usize] as isize)
                                    as core::ffi::c_int) < c_b
                                {
                                    if (*p
                                        .offset(pixel[13 as core::ffi::c_int as usize] as isize)
                                        as core::ffi::c_int) < c_b
                                    {
                                        if (*p
                                            .offset(pixel[14 as core::ffi::c_int as usize] as isize)
                                            as core::ffi::c_int) < c_b
                                        {
                                            if (*p
                                                .offset(pixel[15 as core::ffi::c_int as usize] as isize)
                                                as core::ffi::c_int) < c_b
                                            {
                                                current_block_218 = 2;
                                            } else {
                                                current_block_218 = 3;
                                            }
                                        } else {
                                            current_block_218 = 3;
                                        }
                                    } else {
                                        current_block_218 = 3;
                                    }
                                } else {
                                    current_block_218 = 3;
                                }
                            } else {
                                current_block_218 = 3;
                            }
                        } else {
                            current_block_218 = 3;
                        }
                    } else {
                        current_block_218 = 3;
                    }
                } else {
                    current_block_218 = 3;
                }
            } else {
                current_block_218 = 3;
            }
            match current_block_218 {
                2 => {
                    if num_corners == rsize {
                        rsize *= 2 as core::ffi::c_int;
                        let mut new_ret_corners: *mut xy = realloc(
                            ret_corners as *mut core::ffi::c_void,
                            (::core::mem::size_of::<xy>() as size_t)
                                .wrapping_mul(rsize as size_t),
                        ) as *mut xy;
                        if new_ret_corners.is_null() {
                            free(ret_corners as *mut core::ffi::c_void);
                            return 0 as *mut xy;
                        }
                        ret_corners = new_ret_corners;
                    }
                    (*ret_corners.offset(num_corners as isize)).x = x;
                    (*ret_corners.offset(num_corners as isize)).y = y;
                    num_corners += 1;
                }
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }
    *ret_num_corners = num_corners;
    return ret_corners;
}

extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}

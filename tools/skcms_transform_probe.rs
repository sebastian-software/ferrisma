// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path="../tests/common/skcms_profile_observation.rs"]
mod skcms_profile_observation;
#[path = "../tests/common/skcms_transform.rs"]
mod cases;
use std::io::{self,BufRead};
fn main(){for line in io::stdin().lock().lines(){println!("{}",cases::observe(&line.unwrap()));}}

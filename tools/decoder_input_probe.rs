// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
extern crate jxl;
#[path = "../tests/support/decoder_input.rs"]
mod driver;
use std::io::{self,BufRead};
fn main(){
 for line in io::stdin().lock().lines(){let line=line.unwrap();let mut args=line.split_whitespace();let kind=args.next().unwrap();let result=if kind=="di-short"{driver::observe_short(&mut args)}else{assert_eq!(kind,"di-run");driver::observe(&mut args)};println!("{result}");}
}

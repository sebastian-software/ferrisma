// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../tests/common/default_cms.rs"]
mod cases;
#[path = "../tests/common/default_cms_cases.rs"]
mod inputs;
#[path = "../tests/common/skcms_parser.rs"]
mod parser_inputs;
#[path = "../tests/common/skcms_profile_observation.rs"]
mod skcms_profile_observation;
use std::io::{self, BufRead};
fn main() {
    if let Some(family) = std::env::args().nth(1) {
        let requests = match family.as_str() {
            "matrix" => inputs::matrix(),
            "matrix-exceptional" => inputs::matrix_exceptional(),
            "cicp" => inputs::cicp(),
            "exceptional" => inputs::exceptional(),
            "tf-domain" => inputs::tf_domain(),
            "prefixes" => inputs::prefixes(),
            "init-prefixes" => inputs::init_prefixes(),
            "callbacks" => inputs::callbacks(),
            "numeric" => inputs::numeric(),
            "allocation" => inputs::allocation(),
            "luminances" => inputs::luminances(&parser_inputs::original_profiles()),
            "profile-equality" => inputs::profile_equality(&parser_inputs::original_profiles()),
            "synthetic-fields" => inputs::fields_for_profiles(parser_inputs::synthetic_profiles()),
            "malformed-fields" => {
                inputs::fields_for_parser_requests(parser_inputs::malformed_and_truncated_requests())
            }
            "profiles" => {
                for (name, bytes) in inputs::generated_profiles() {
                    println!("{name}\t{}", inputs::hex(&bytes));
                }
                return;
            }
            "guards" | "detail" | "rows" => {
                fn visit(path: &std::path::Path, files: &mut Vec<std::path::PathBuf>) {
                    for entry in std::fs::read_dir(path).unwrap() {
                        let p = entry.unwrap().path();
                        if p.is_dir() {
                            visit(&p, files);
                        } else if p.extension().is_some_and(|x| x == "icc" || x == "icm") {
                            files.push(p);
                        }
                    }
                }
                let mut files = Vec::new();
                visit(
                    std::path::Path::new("upstream/libjxl/third_party/skcms/profiles"),
                    &mut files,
                );
                files.sort();
                assert_eq!(files.len(), 88);
                let original: Vec<_> = files
                    .into_iter()
                    .map(|p| std::fs::read(p).unwrap())
                    .collect();
                match family.as_str() {
                    "guards" => inputs::guards(&original),
                    "detail" => inputs::detail(&original),
                    _ => inputs::rows(&original),
                }
            }
            _ => panic!("unknown input family: {family}"),
        };
        inputs::capture(&family, &requests);
        for request in requests {
            println!("{request}");
        }
        return;
    }
    for line in io::stdin().lock().lines() {
        println!("{}", cases::observe(&line.unwrap()));
    }
}

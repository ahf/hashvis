// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate sha3;
extern crate clap;

extern crate hashvis;

use sha3::{Sha3_256, Digest};
use clap::{Arg, App};

use hashvis::ImageGenerator;

fn main() {
    let matches = App::new("hashvis")
        .version("0.1.0")
        .author("Alexander Færøy <ahf@0x90.dk>")
        .about("Visualize hash values")
        .arg(Arg::with_name("filename")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("Output file to store the PNG image in."))
        .arg(Arg::with_name("string")
             .required(true)
             .takes_value(true)
             .index(2)
             .help("Input string to hash with SHA3-256."))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let string = matches.value_of("string").unwrap();
    let size = 512;

    println!("Generating image with SHA3(\"{}\") in {} ...", string, filename);

    let mut h = Sha3_256::default();
    h.input(string.as_bytes());
    let r = h.result();

    let generator = ImageGenerator::new(r.as_slice());
    generator.generate(String::from(filename), size);
}

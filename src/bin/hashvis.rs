// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate clap;
extern crate sha3;

extern crate hashvis;

use clap::{App, Arg};
use sha3::{Digest, Sha3_256};

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
             .help("Output file to store the image in. The extension of the filename decides the image output format."))
        .arg(Arg::with_name("string")
             .required(true)
             .takes_value(true)
             .index(2)
             .help("Input string to hash with SHA3-256."))
        .arg(Arg::with_name("size")
             .required(true)
             .takes_value(true)
             .index(3)
             .help("Size of the output image"))
        .arg(Arg::with_name("factor")
             .required(false)
             .takes_value(true)
             .index(4)
             .help("Factor specifies how much to zoom out on the image."))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let string = matches.value_of("string").unwrap();
    let size = matches.value_of("size").unwrap();
    let factor = matches.value_of("factor").unwrap_or("1.0");

    println!(
        "Generating image with SHA3(\"{}\") in {} ...",
        string, filename
    );

    let mut h = Sha3_256::default();
    h.input(string.as_bytes());
    let r = h.result();

    let generator = ImageGenerator::new(r.as_slice());
    generator.generate(String::from(filename), size.parse().unwrap_or(256), factor.parse().unwrap_or(1.0));
}

// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate image;
extern crate rand;

use image::{Rgb, RgbImage};

/// Expression types and helpers.
mod expression;
use self::expression::{Evaluate, Generator};

/// Various math helpers.
mod math;

fn to_u8(x: f64) -> u8 {
    (x * 127.5 + 127.5) as u8
}

pub struct ImageGenerator {
    r: Box<dyn Evaluate>,
    g: Box<dyn Evaluate>,
    b: Box<dyn Evaluate>,
    random_data_used: usize,
}

impl ImageGenerator {
    pub fn new(seed: &[u8]) -> ImageGenerator {
        let mut generator = Generator::new(seed);

        ImageGenerator {
            r: generator.generate(),
            g: generator.generate(),
            b: generator.generate(),
            random_data_used: generator.random_data_used(),
        }
    }

    pub fn generate(&self, filename: String, size: u32, factor: f64) {
        let unit_size: f64 = (size as f64) / 2.0;

        assert!(size % 2 == 0);

        println!("R: f(x, y) = {}", self.r);
        println!("G: f(x, y) = {}", self.g);
        println!("B: f(x, y) = {}", self.b);

        println!("Random data used = {}", self.random_data_used);

        let image = RgbImage::from_fn(size, size, |i_x, i_y| {
            let x = (((i_x as f64) - unit_size) / unit_size) * factor;
            let y = (((i_y as f64) - unit_size) / unit_size) * factor;

            let r = self.r.evaluate(x, y);
            let g = self.g.evaluate(x, y);
            let b = self.b.evaluate(x, y);

            Rgb([to_u8(r), to_u8(g), to_u8(b)])
        });

        image.save(filename).unwrap();
    }
}

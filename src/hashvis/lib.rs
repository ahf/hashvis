// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate rand;
extern crate image;

use std::fs::File;

use image::{Rgb, RgbImage, ImageRgb8, PNG};

/// Expression types and helpers.
mod expression;
use self::expression::{Evaluate, Generator};

/// Various math helpers.
mod math;

pub struct ImageGenerator {
    r: Box<Evaluate>,
    g: Box<Evaluate>,
    b: Box<Evaluate>,
}

impl ImageGenerator {
    pub fn new(seed: &[u8]) -> ImageGenerator {
        let mut generator = Generator::new(seed);

        ImageGenerator {
            r: generator.generate(),
            g: generator.generate(),
            b: generator.generate(),
        }
    }

    pub fn generate(&self, filename: String, size: u32) {
        let unit_size: f64 = (size as f64) / 2.0;

        assert!(size % 2 == 0);

        let image = RgbImage::from_fn(size, size, |i_x, i_y| {
            let x = ((i_x as f64) - unit_size) / unit_size;
            let y = ((i_y as f64) - unit_size) / unit_size;

            let r = (self.r.evaluate(x, y) * 127.5 + 127.5) as u8;
            let g = (self.g.evaluate(x, y) * 127.5 + 127.5) as u8;
            let b = (self.b.evaluate(x, y) * 127.5 + 127.5) as u8;

            Rgb([r, g, b])
        });

        let file = &mut File::create(filename).unwrap();
        ImageRgb8(image).save(file, PNG).unwrap();
    }
}

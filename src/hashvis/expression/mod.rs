// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use rand::{Rng, SeedableRng};
use rand::chacha::ChaChaRng;

mod x;
use self::x::X;

mod y;
use self::y::Y;

mod product;
use self::product::Product;

mod sin_pi;
use self::sin_pi::SinPi;

mod cos_pi;
use self::cos_pi::CosPi;

mod tan_pi;
use self::tan_pi::TanPi;

/// Trait for items that supports evaluating expressions at a given
/// point in a Cartesian coordinate system.
pub trait Evaluate: fmt::Display {
    /// Returns the z value at the given x and y coordinate.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate.
    /// * `y` - The y coordinate.
    ///
    fn evaluate(&self, x: f64, y: f64) -> f64;
}

pub struct Generator {
    rng: Box<Rng>
}

impl Generator {
    pub fn new(seed: &[u8]) -> Generator {
        // FIXME(ahf): This could probably done in a nicer way. The
        // ChaChaRng::from_seed() takes a [u32; 32], but uses it
        // internally as an [u8; 32]. We therefore have to do the safe
        // conversion from [u8; 32] to [u32; 32] ourselves. This appears
        // to be fixed in the Rand Github repository.
        let mut v: Vec<u32> = Vec::with_capacity(32);

        for value in seed.iter() {
            v.push(*value as u32);
        }

        Generator {
            rng: Box::new(ChaChaRng::from_seed(v.as_slice()))
        }
    }

    pub fn next_f64(&mut self) -> f64 {
        self.rng.next_f64()
    }

    pub fn generate(&mut self) -> Box<Evaluate> {
        self.generate_expression(0.99)
    }

    pub fn generate_expression(&mut self, probability: f64) -> Box<Evaluate> {
        assert!(probability >= 0.00 && probability < 1.00);

        let mut set = Vec::new();

        if self.next_f64() < probability {
            set.push(SinPi::generate(self, probability));
            set.push(CosPi::generate(self, probability));
            set.push(TanPi::generate(self, probability));
            set.push(Product::generate(self, probability));
        } else {
            set.push(X::generate());
            set.push(Y::generate());
        }

        self.rng.shuffle(&mut set);
        set.remove(0)
    }
}

// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;
use std::mem;

use rand::chacha::ChaChaRng;
use rand::{Rng, SeedableRng};

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

mod elliptic_curve;
use self::elliptic_curve::EllipticCurve;

pub enum ExpressionType {
    SinPi,
    CosPi,
    TanPi,
    Product,
    EllipticCurve,
    X,
    Y,
}

impl ExpressionType {
    pub fn generate(&self, generator: &mut Generator, probability: f64) -> Box<dyn Evaluate> {
        match *self {
            ExpressionType::SinPi => SinPi::generate(generator, probability),
            ExpressionType::CosPi => CosPi::generate(generator, probability),
            ExpressionType::TanPi => TanPi::generate(generator, probability),
            ExpressionType::Product => Product::generate(generator, probability),
            ExpressionType::EllipticCurve => EllipticCurve::generate(generator),
            ExpressionType::X => X::generate(),
            ExpressionType::Y => Y::generate(),
        }
    }
}

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
    rng: Box<dyn Rng>,
    random_data_used: usize,
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
            rng: Box::new(ChaChaRng::from_seed(v.as_slice())),
            random_data_used: 0,
        }
    }

    pub fn random_data_used(self) -> usize {
        self.random_data_used
    }

    pub fn next_f64(&mut self) -> f64 {
        self.random_data_used += mem::size_of::<f64>();
        self.rng.next_f64()
    }

    pub fn generate(&mut self) -> Box<dyn Evaluate> {
        self.generate_expression(0.99)
    }

    pub fn generate_expression(&mut self, probability: f64) -> Box<dyn Evaluate> {
        assert!(probability >= 0.00 && probability < 1.00);

        let mut set = Vec::new();

        if self.next_f64() < probability {
            set.push(ExpressionType::SinPi);
            set.push(ExpressionType::CosPi);
            set.push(ExpressionType::TanPi);
            set.push(ExpressionType::Product);
            set.push(ExpressionType::EllipticCurve);
        } else {
            set.push(ExpressionType::X);
            set.push(ExpressionType::Y);
        }

        self.rng.shuffle(&mut set);
        let expression_type = set.remove(0);
        expression_type.generate(self, probability)
    }
}

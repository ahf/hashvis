// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use expression::{Evaluate, Generator};

pub struct EllipticCurve {
    a: f64,
    b: f64,
}

impl EllipticCurve {
    pub fn new(a: f64, b: f64) -> EllipticCurve {
        EllipticCurve { a, b }
    }

    pub fn generate(generator: &mut Generator) -> Box<dyn Evaluate> {
        let a = generator.next_f64();
        let b = generator.next_f64();

        Box::new(EllipticCurve::new(a, b))
    }
}

impl Evaluate for EllipticCurve {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        assert!(x >= -1.0 && y <= 1.0);
        x.powf(3.0) + self.a * x + self.b - y.powf(2.0)
    }
}

impl fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EllipticCurve(x^3 + {} * x + {} - y^2)", self.a, self.b)
    }
}

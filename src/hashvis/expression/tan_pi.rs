// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use expression::{Evaluate, Generator};
use math::{tan, PI};

pub struct TanPi {
    expression: Box<Evaluate>,
}

impl TanPi {
    pub fn new(expression: Box<Evaluate>) -> TanPi {
        TanPi { expression }
    }

    pub fn generate(generator: &mut Generator, probability: f64) -> Box<Evaluate> {
        let expression = generator.generate_expression(probability * probability);

        Box::new(TanPi::new(expression))
    }
}

impl Evaluate for TanPi {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        assert!(x >= -1.0 && y <= 1.0);
        tan(PI * self.expression.evaluate(x, y))
    }
}

impl fmt::Display for TanPi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tan(pi * {})", self.expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::{X, Y};

    #[test]
    fn format() {
        assert_eq!("tan(pi * x)", format!("{}", TanPi::new(X::generate())));
        assert_eq!("tan(pi * y)", format!("{}", TanPi::new(Y::generate())));
    }
}

// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use expression::{Evaluate, Generator};
use math::{cos, PI};

pub struct CosPi {
    expression: Box<Evaluate>,
}

impl CosPi {
    pub fn new(expression: Box<Evaluate>) -> CosPi {
        CosPi { expression }
    }

    pub fn generate(generator: &mut Generator, probability: f64) -> Box<Evaluate> {
        let expression = generator.generate_expression(probability * probability);

        Box::new(CosPi::new(expression))
    }
}

impl Evaluate for CosPi {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        assert!(x >= -1.0 && y <= 1.0);
        cos(PI * self.expression.evaluate(x, y))
    }
}

impl fmt::Display for CosPi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cos(pi * {})", self.expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::{X, Y};

    #[test]
    fn format() {
        assert_eq!("cos(pi * x)", format!("{}", CosPi::new(X::generate())));
        assert_eq!("cos(pi * y)", format!("{}", CosPi::new(Y::generate())));
    }
}

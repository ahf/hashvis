// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use expression::{Evaluate, Generator};
use math::{PI, sin};

pub struct SinPi {
    expression: Box<Evaluate>
}

impl SinPi {
    pub fn new(expression: Box<Evaluate>) -> SinPi {
        SinPi {
            expression,
        }
    }

    pub fn generate(generator: &mut Generator, probability: f64) -> Box<Evaluate> {
        let expression = generator.generate_expression(probability * probability);

        Box::new(SinPi::new(expression))
    }
}

impl Evaluate for SinPi {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        assert!(x >= -1.0 && y <= 1.0);
        sin(PI * self.expression.evaluate(x, y))
    }
}

impl fmt::Display for SinPi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sin(pi * {})", self.expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::{X, Y};

    #[test]
    fn format() {
        assert_eq!("sin(pi * x)", format!("{}", SinPi::new(X::generate())));
        assert_eq!("sin(pi * y)", format!("{}", SinPi::new(Y::generate())));
    }
}

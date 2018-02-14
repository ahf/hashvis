// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use expression::{Evaluate, Generator};

pub struct Product {
    expression_lhs: Box<Evaluate>,
    expression_rhs: Box<Evaluate>,
}

impl Product {
    pub fn new(expression_lhs: Box<Evaluate>, expression_rhs: Box<Evaluate>) -> Product {
        Product {
            expression_lhs,
            expression_rhs,
        }
    }

    pub fn generate(generator: &mut Generator, probability: f64) -> Box<Evaluate> {
        let lhs = generator.generate_expression(probability * probability);
        let rhs = generator.generate_expression(probability * probability);

        Box::new(Product::new(lhs, rhs))
    }
}

impl Evaluate for Product {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        assert!(x >= -1.0 && y <= 1.0);
        self.expression_lhs.evaluate(x, y) * self.expression_rhs.evaluate(x, y)
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} * {}", self.expression_lhs, self.expression_rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::{X, Y};

    #[test]
    fn format() {
        assert_eq!("x * y",
                   format!("{}", Product::new(Box::new(X::new()),
                                              Box::new(Y::new()))));
    }
}

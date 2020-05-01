// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use expression::Evaluate;

/// The Y coordinate in the Cartesian coordinate system.
pub struct Y;

impl Y {
    pub fn new() -> Y {
        Y {}
    }

    pub fn generate() -> Box<dyn Evaluate> {
        Box::new(Y::new())
    }
}

impl Evaluate for Y {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        assert!(x >= -1.0 && y <= 1.0);
        y
    }
}

impl fmt::Display for Y {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "y")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        assert_eq!("y", format!("{}", Y::new()));
    }

    #[test]
    fn evaluate() {
        let y = Y::new();
        assert_eq!(y.evaluate(-1.0, 1.0), 1.0);
        assert_eq!(y.evaluate(1.0, -1.0), -1.0);
    }
}

// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fmt;

use expression::Evaluate;

/// The X coordinate in the Cartesian coordinate system.
pub struct X;

impl X {
    pub fn new() -> X {
        X { }
    }

    pub fn generate() -> Box<Evaluate> {
        Box::new(X::new())
    }
}

impl Evaluate for X {
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        assert!(x >= -1.0 && y <= 1.0);
        x
    }
}

impl fmt::Display for X {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        assert_eq!("x", format!("{}", X::new()));
    }
}

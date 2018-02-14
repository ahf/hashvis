// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::f64;

/// Alias for the std::f64::consts::PI.
pub const PI: f64 = f64::consts::PI;

/// Returns the sine of the value in, radians.
pub fn sin(value: f64) -> f64 {
    value.sin()
}

/// Returns the cosine of the value, in radians.
pub fn cos(value: f64) -> f64 {
    value.cos()
}

/// Returns the tangent of the value, in radians.
pub fn tan(value: f64) -> f64 {
    value.tan()
}

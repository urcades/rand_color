//! Generate random CIELAB color values with configurable component bounds.
//!
//! The crate samples each component independently from the configured numeric
//! ranges. It does not attempt palette design, gamut mapping, contrast
//! checking, or perceptual-uniform sampling.
//!
//! # Examples
//!
//! ```rust
//! use rand_lab::random_lab;
//!
//! let color = random_lab();
//!
//! assert!((0.0..=100.0).contains(&color.lightness));
//! assert!(color.to_lab_string().starts_with("lab("));
//! ```

#![warn(missing_docs)]

#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents a randomly generated CIELAB `lab()` color value.
pub struct LabColor {
    /// CIELAB lightness component in `0.0..=100.0`.
    pub lightness: f32,
    /// CIELAB `a` opponent-axis component in `-128.0..=127.0`.
    pub a: f32,
    /// CIELAB `b` opponent-axis component in `-128.0..=127.0`.
    pub b: f32,
    /// Alpha channel in `0.0..=1.0`.
    pub alpha: f32,
}

impl LabColor {
    /// Formats the color as a `lab(l, a, b, alpha)` string.
    pub fn to_lab_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for LabColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "lab({:.2}, {:.2}, {:.2}, {:.2})",
            self.lightness, self.a, self.b, self.alpha
        )
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
/// User-provided CIELAB component bounds for random generation.
pub struct LabRange {
    /// Inclusive lightness generation range.
    pub lightness: (f32, f32),
    /// Inclusive `a` component generation range.
    pub a: (f32, f32),
    /// Inclusive `b` component generation range.
    pub b: (f32, f32),
    /// Inclusive alpha generation range.
    pub alpha: (f32, f32),
}

impl Default for LabRange {
    fn default() -> Self {
        Self {
            lightness: (0.0, 100.0),
            a: (-128.0, 127.0),
            b: (-128.0, 127.0),
            alpha: (0.0, 1.0),
        }
    }
}

impl LabRange {
    /// Builds a new set of bounds.
    ///
    /// # Errors
    ///
    /// Returns an error when any range is invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_lightness: f32,
        max_lightness: f32,
        min_a: f32,
        max_a: f32,
        min_b: f32,
        max_b: f32,
        min_alpha: f32,
        max_alpha: f32,
    ) -> Result<Self, LabError> {
        let range = Self {
            lightness: (min_lightness, max_lightness),
            a: (min_a, max_a),
            b: (min_b, max_b),
            alpha: (min_alpha, max_alpha),
        };
        validate_range(&range)?;
        Ok(range)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Error returned when CIELAB bounds are invalid.
pub enum LabError {
    /// The minimum lightness bound is greater than the maximum lightness bound.
    InvalidLightnessRange,
    /// The minimum `a` bound is greater than the maximum `a` bound.
    InvalidARange,
    /// The minimum `b` bound is greater than the maximum `b` bound.
    InvalidBRange,
    /// The minimum alpha bound is greater than the maximum alpha bound.
    InvalidAlphaRange,
    /// A component bound is outside the supported range.
    ComponentOutOfBounds,
    /// A component bound is infinite or NaN.
    NonFiniteValue,
}

impl fmt::Display for LabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidLightnessRange => "lightness min value must be <= max",
            Self::InvalidARange => "a min value must be <= max",
            Self::InvalidBRange => "b min value must be <= max",
            Self::InvalidAlphaRange => "alpha min value must be <= max",
            Self::ComponentOutOfBounds => "lightness 0..=100, a/b -128..=127, alpha 0..=1",
            Self::NonFiniteValue => "all Lab components must be finite",
        };
        f.write_str(message)
    }
}

impl Error for LabError {}

/// Generates a random CIELAB color using default bounds.
pub fn random_lab() -> LabColor {
    let mut rng = rand::thread_rng();
    random_lab_with_rng(&mut rng)
}

/// Generates a random CIELAB color using default bounds and a caller-provided RNG.
///
/// This is useful for deterministic tests.
pub fn random_lab_with_rng<R: Rng + ?Sized>(rng: &mut R) -> LabColor {
    random_lab_in_with_rng(LabRange::default(), rng).expect("default lab range should be valid")
}

/// Generates a random CIELAB color using custom bounds.
///
/// # Errors
///
/// Returns a [`LabError`] when provided bounds are invalid.
pub fn random_lab_in(range: LabRange) -> Result<LabColor, LabError> {
    let mut rng = rand::thread_rng();
    random_lab_in_with_rng(range, &mut rng)
}

/// Generates a random CIELAB color using custom bounds and a caller-provided RNG.
///
/// # Errors
///
/// Returns a [`LabError`] when provided bounds are invalid.
pub fn random_lab_in_with_rng<R: Rng + ?Sized>(
    range: LabRange,
    rng: &mut R,
) -> Result<LabColor, LabError> {
    validate_range(&range)?;
    Ok(LabColor {
        lightness: rng.gen_range(range.lightness.0..=range.lightness.1),
        a: rng.gen_range(range.a.0..=range.a.1),
        b: rng.gen_range(range.b.0..=range.b.1),
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

fn validate_range(range: &LabRange) -> Result<(), LabError> {
    let numbers = [
        range.lightness.0,
        range.lightness.1,
        range.a.0,
        range.a.1,
        range.b.0,
        range.b.1,
        range.alpha.0,
        range.alpha.1,
    ];
    if numbers.iter().any(|value| !value.is_finite()) {
        return Err(LabError::NonFiniteValue);
    }
    if range.lightness.0 > range.lightness.1 {
        return Err(LabError::InvalidLightnessRange);
    }
    if range.a.0 > range.a.1 {
        return Err(LabError::InvalidARange);
    }
    if range.b.0 > range.b.1 {
        return Err(LabError::InvalidBRange);
    }
    if range.alpha.0 > range.alpha.1 {
        return Err(LabError::InvalidAlphaRange);
    }
    if range.lightness.0 < 0.0
        || range.lightness.1 > 100.0
        || range.a.0 < -128.0
        || range.a.1 > 127.0
        || range.b.0 < -128.0
        || range.b.1 > 127.0
        || range.alpha.0 < 0.0
        || range.alpha.1 > 1.0
    {
        return Err(LabError::ComponentOutOfBounds);
    }
    Ok(())
}

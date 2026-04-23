//! Generate random CIELCH color values with configurable component bounds.
//!
//! The crate samples each component independently from the configured numeric
//! ranges. It does not attempt palette design, gamut mapping, contrast
//! checking, or perceptual-uniform sampling.
//!
//! # Examples
//!
//! ```rust
//! use rand_lch::random_lch;
//!
//! let color = random_lch();
//!
//! assert!((0.0..=360.0).contains(&color.hue));
//! assert!(color.to_lch_string().starts_with("lch("));
//! ```

#![warn(missing_docs)]

#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents a randomly generated CIELCH `lch()` color value.
pub struct LchColor {
    /// CIELCH lightness component in `0.0..=100.0`.
    pub lightness: f32,
    /// CIELCH chroma component in `0.0..=150.0`.
    pub chroma: f32,
    /// CIELCH hue angle in degrees, normally in `0.0..=360.0`.
    pub hue: f32,
    /// Alpha channel in `0.0..=1.0`.
    pub alpha: f32,
}

impl LchColor {
    /// Formats the color as an `lch(l, c, h, alpha)` string.
    pub fn to_lch_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for LchColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "lch({:.2}, {:.2}, {:.1}, {:.2})",
            self.lightness, self.chroma, self.hue, self.alpha
        )
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
/// User-provided CIELCH component bounds for random generation.
pub struct LchRange {
    /// Inclusive lightness generation range.
    pub lightness: (f32, f32),
    /// Inclusive chroma generation range.
    pub chroma: (f32, f32),
    /// Inclusive hue generation range in degrees.
    pub hue: (f32, f32),
    /// Inclusive alpha generation range.
    pub alpha: (f32, f32),
}

impl Default for LchRange {
    fn default() -> Self {
        Self {
            lightness: (0.0, 100.0),
            chroma: (0.0, 150.0),
            hue: (0.0, 360.0),
            alpha: (0.0, 1.0),
        }
    }
}

impl LchRange {
    /// Builds a new set of bounds.
    ///
    /// # Errors
    ///
    /// Returns an error when any range is invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_lightness: f32,
        max_lightness: f32,
        min_chroma: f32,
        max_chroma: f32,
        min_hue: f32,
        max_hue: f32,
        min_alpha: f32,
        max_alpha: f32,
    ) -> Result<Self, LchError> {
        let range = Self {
            lightness: (min_lightness, max_lightness),
            chroma: (min_chroma, max_chroma),
            hue: (min_hue, max_hue),
            alpha: (min_alpha, max_alpha),
        };
        validate_range(&range)?;
        Ok(range)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Error returned when CIELCH bounds are invalid.
pub enum LchError {
    /// The minimum lightness bound is greater than the maximum lightness bound.
    InvalidLightnessRange,
    /// The minimum chroma bound is greater than the maximum chroma bound.
    InvalidChromaRange,
    /// The minimum hue bound is greater than the maximum hue bound.
    InvalidHueRange,
    /// The minimum alpha bound is greater than the maximum alpha bound.
    InvalidAlphaRange,
    /// A component bound is outside the supported range.
    ComponentOutOfBounds,
    /// A component bound is infinite or NaN.
    NonFiniteValue,
}

impl fmt::Display for LchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidLightnessRange => "lightness min value must be <= max",
            Self::InvalidChromaRange => "chroma min value must be <= max",
            Self::InvalidHueRange => "hue min value must be <= max",
            Self::InvalidAlphaRange => "alpha min value must be <= max",
            Self::ComponentOutOfBounds => {
                "lightness 0..=100, chroma 0..=150, hue 0..=360, alpha 0..=1"
            }
            Self::NonFiniteValue => "all Lch components must be finite",
        };
        f.write_str(message)
    }
}

impl Error for LchError {}

/// Generates a random CIELCH color using default bounds.
pub fn random_lch() -> LchColor {
    let mut rng = rand::thread_rng();
    random_lch_with_rng(&mut rng)
}

/// Generates a random CIELCH color using default bounds and a caller-provided RNG.
///
/// This is useful for deterministic tests.
pub fn random_lch_with_rng<R: Rng + ?Sized>(rng: &mut R) -> LchColor {
    random_lch_in_with_rng(LchRange::default(), rng).expect("default lch range should be valid")
}

/// Generates a random CIELCH color using custom bounds.
///
/// # Errors
///
/// Returns a [`LchError`] when provided bounds are invalid.
pub fn random_lch_in(range: LchRange) -> Result<LchColor, LchError> {
    let mut rng = rand::thread_rng();
    random_lch_in_with_rng(range, &mut rng)
}

/// Generates a random CIELCH color using custom bounds and a caller-provided RNG.
///
/// # Errors
///
/// Returns a [`LchError`] when provided bounds are invalid.
pub fn random_lch_in_with_rng<R: Rng + ?Sized>(
    range: LchRange,
    rng: &mut R,
) -> Result<LchColor, LchError> {
    validate_range(&range)?;
    Ok(LchColor {
        lightness: rng.gen_range(range.lightness.0..=range.lightness.1),
        chroma: rng.gen_range(range.chroma.0..=range.chroma.1),
        hue: rng.gen_range(range.hue.0..=range.hue.1),
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

fn validate_range(range: &LchRange) -> Result<(), LchError> {
    let numbers = [
        range.lightness.0,
        range.lightness.1,
        range.chroma.0,
        range.chroma.1,
        range.hue.0,
        range.hue.1,
        range.alpha.0,
        range.alpha.1,
    ];
    if numbers.iter().any(|value| !value.is_finite()) {
        return Err(LchError::NonFiniteValue);
    }
    if range.lightness.0 > range.lightness.1 {
        return Err(LchError::InvalidLightnessRange);
    }
    if range.chroma.0 > range.chroma.1 {
        return Err(LchError::InvalidChromaRange);
    }
    if range.hue.0 > range.hue.1 {
        return Err(LchError::InvalidHueRange);
    }
    if range.alpha.0 > range.alpha.1 {
        return Err(LchError::InvalidAlphaRange);
    }
    if range.lightness.0 < 0.0
        || range.lightness.1 > 100.0
        || range.chroma.0 < 0.0
        || range.chroma.1 > 150.0
        || range.hue.0 < 0.0
        || range.hue.1 > 360.0
        || range.alpha.0 < 0.0
        || range.alpha.1 > 1.0
    {
        return Err(LchError::ComponentOutOfBounds);
    }
    Ok(())
}

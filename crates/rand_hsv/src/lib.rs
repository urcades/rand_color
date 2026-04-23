//! Generate random HSV/HSVA color values with configurable channel bounds.
//!
//! The crate samples each component independently from the configured numeric
//! ranges. It does not attempt palette design, gamut mapping, contrast
//! checking, or perceptual-uniform sampling.
//!
//! The formatted `hsva(...)` output is a stable crate compatibility format; it
//! is not intended to imply broad browser CSS support.
//!
//! # Examples
//!
//! ```rust
//! use rand_hsv::random_hsv;
//!
//! let color = random_hsv();
//!
//! assert!((0.0..=360.0).contains(&color.hue));
//! assert!(color.to_hsva_string().starts_with("hsva("));
//! ```

#![warn(missing_docs)]

#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Represents a randomly generated `hsva()` color value.
pub struct HsvColor {
    /// Hue angle in degrees, normally in `0.0..=360.0`.
    pub hue: f32,
    /// Saturation percentage in `0.0..=100.0`.
    pub saturation: f32,
    /// Value percentage in `0.0..=100.0`.
    pub value: f32,
    /// Alpha channel in `0.0..=1.0`.
    pub alpha: f32,
}

impl HsvColor {
    /// Formats the color as an `hsva(h, s%, v%, a)` string.
    ///
    /// This is a crate compatibility format, not a browser CSS guarantee.
    pub fn to_hsva_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for HsvColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hsva({:.1}, {:.1}%, {:.1}%, {:.2})",
            self.hue, self.saturation, self.value, self.alpha
        )
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
/// User-provided HSV channel bounds for random generation.
pub struct HsvRange {
    /// Inclusive hue generation range in degrees.
    pub hue: (f32, f32),
    /// Inclusive saturation generation range in percent.
    pub saturation: (f32, f32),
    /// Inclusive value generation range in percent.
    pub value: (f32, f32),
    /// Inclusive alpha generation range.
    pub alpha: (f32, f32),
}

impl Default for HsvRange {
    fn default() -> Self {
        Self {
            hue: (0.0, 360.0),
            saturation: (0.0, 100.0),
            value: (0.0, 100.0),
            alpha: (0.0, 1.0),
        }
    }
}

impl HsvRange {
    /// Builds a new set of bounds.
    ///
    /// # Errors
    ///
    /// Returns an error when any range is invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_hue: f32,
        max_hue: f32,
        min_saturation: f32,
        max_saturation: f32,
        min_value: f32,
        max_value: f32,
        min_alpha: f32,
        max_alpha: f32,
    ) -> Result<Self, HsvError> {
        let range = Self {
            hue: (min_hue, max_hue),
            saturation: (min_saturation, max_saturation),
            value: (min_value, max_value),
            alpha: (min_alpha, max_alpha),
        };
        validate_range(&range)?;
        Ok(range)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Error returned when HSV bounds are invalid.
pub enum HsvError {
    /// The minimum hue bound is greater than the maximum hue bound.
    InvalidHueRange,
    /// The minimum saturation bound is greater than the maximum saturation bound.
    InvalidSaturationRange,
    /// The minimum value bound is greater than the maximum value bound.
    InvalidValueRange,
    /// The minimum alpha bound is greater than the maximum alpha bound.
    InvalidAlphaRange,
    /// A component bound is outside the supported range.
    ComponentOutOfBounds,
    /// A component bound is infinite or NaN.
    NonFiniteValue,
}

impl fmt::Display for HsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidHueRange => "hue min value must be <= max",
            Self::InvalidSaturationRange => "saturation min value must be <= max",
            Self::InvalidValueRange => "value min value must be <= max",
            Self::InvalidAlphaRange => "alpha min value must be <= max",
            Self::ComponentOutOfBounds => {
                "hue must be 0..=360, saturation/value 0..=100, alpha 0..=1"
            }
            Self::NonFiniteValue => "all HSV components must be finite",
        };
        f.write_str(message)
    }
}

impl Error for HsvError {}

/// Generates a random HSV color using default bounds.
pub fn random_hsv() -> HsvColor {
    let mut rng = rand::thread_rng();
    random_hsv_with_rng(&mut rng)
}

/// Generates a random HSV color using default bounds and a caller-provided RNG.
///
/// This is useful for deterministic tests.
pub fn random_hsv_with_rng<R: Rng + ?Sized>(rng: &mut R) -> HsvColor {
    random_hsv_in_with_rng(HsvRange::default(), rng).expect("default hsv range should be valid")
}

/// Generates a random HSV color using custom bounds.
///
/// # Errors
///
/// Returns a [`HsvError`] when provided bounds are invalid.
pub fn random_hsv_in(range: HsvRange) -> Result<HsvColor, HsvError> {
    let mut rng = rand::thread_rng();
    random_hsv_in_with_rng(range, &mut rng)
}

/// Generates a random HSV color using custom bounds and a caller-provided RNG.
///
/// # Errors
///
/// Returns a [`HsvError`] when provided bounds are invalid.
pub fn random_hsv_in_with_rng<R: Rng + ?Sized>(
    range: HsvRange,
    rng: &mut R,
) -> Result<HsvColor, HsvError> {
    validate_range(&range)?;
    Ok(HsvColor {
        hue: rng.gen_range(range.hue.0..=range.hue.1),
        saturation: rng.gen_range(range.saturation.0..=range.saturation.1),
        value: rng.gen_range(range.value.0..=range.value.1),
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

fn validate_range(range: &HsvRange) -> Result<(), HsvError> {
    let numbers = [
        range.hue.0,
        range.hue.1,
        range.saturation.0,
        range.saturation.1,
        range.value.0,
        range.value.1,
        range.alpha.0,
        range.alpha.1,
    ];
    if numbers.iter().any(|value| !value.is_finite()) {
        return Err(HsvError::NonFiniteValue);
    }
    if range.hue.0 > range.hue.1 {
        return Err(HsvError::InvalidHueRange);
    }
    if range.saturation.0 > range.saturation.1 {
        return Err(HsvError::InvalidSaturationRange);
    }
    if range.value.0 > range.value.1 {
        return Err(HsvError::InvalidValueRange);
    }
    if range.alpha.0 > range.alpha.1 {
        return Err(HsvError::InvalidAlphaRange);
    }
    if range.hue.0 < 0.0
        || range.hue.1 > 360.0
        || range.saturation.0 < 0.0
        || range.saturation.1 > 100.0
        || range.value.0 < 0.0
        || range.value.1 > 100.0
        || range.alpha.0 < 0.0
        || range.alpha.1 > 1.0
    {
        return Err(HsvError::ComponentOutOfBounds);
    }
    Ok(())
}

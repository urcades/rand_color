//! Generate random HSL/HSLA color values with configurable channel bounds.
//!
//! The crate samples each component independently from the configured numeric
//! ranges. It does not attempt palette design, gamut mapping, contrast
//! checking, or perceptual-uniform sampling.
//!
//! # Examples
//!
//! ```rust
//! use rand_hsl::random_hsl;
//!
//! let color = random_hsl();
//!
//! assert!((0.0..=360.0).contains(&color.hue));
//! assert!(color.to_hsla_string().starts_with("hsla("));
//! ```

#![warn(missing_docs)]

#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;

/// Represents a randomly generated `hsla()` color value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HslColor {
    /// Hue angle in degrees, normally in `0.0..=360.0`.
    pub hue: f32,
    /// Saturation percentage in `0.0..=100.0`.
    pub saturation: f32,
    /// Lightness percentage in `0.0..=100.0`.
    pub lightness: f32,
    /// Alpha channel in `0.0..=1.0`.
    pub alpha: f32,
}

impl HslColor {
    /// Formats the color as an `hsla(h, s%, l%, a)` string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rand_hsl::HslColor;
    ///
    /// let color = HslColor {
    ///     hue: 210.0,
    ///     saturation: 50.0,
    ///     lightness: 40.0,
    ///     alpha: 0.333,
    /// };
    ///
    /// assert_eq!(color.to_hsla_string(), "hsla(210.0, 50.0%, 40.0%, 0.33)");
    /// ```
    pub fn to_hsla_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hsla({:.1}, {:.1}%, {:.1}%, {:.2})",
            self.hue, self.saturation, self.lightness, self.alpha
        )
    }
}

/// User-provided HSL channel bounds for random generation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct HslRange {
    /// Inclusive hue generation range in degrees.
    pub hue: (f32, f32),
    /// Inclusive saturation generation range in percent.
    pub saturation: (f32, f32),
    /// Inclusive lightness generation range in percent.
    pub lightness: (f32, f32),
    /// Inclusive alpha generation range.
    pub alpha: (f32, f32),
}

impl Default for HslRange {
    fn default() -> Self {
        Self {
            hue: (0.0, 360.0),
            saturation: (0.0, 100.0),
            lightness: (0.0, 100.0),
            alpha: (0.0, 1.0),
        }
    }
}

impl HslRange {
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
        min_lightness: f32,
        max_lightness: f32,
        min_alpha: f32,
        max_alpha: f32,
    ) -> Result<Self, HslError> {
        let range = Self {
            hue: (min_hue, max_hue),
            saturation: (min_saturation, max_saturation),
            lightness: (min_lightness, max_lightness),
            alpha: (min_alpha, max_alpha),
        };
        validate_range(&range)?;
        Ok(range)
    }
}

/// Error returned when HSL bounds are invalid.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HslError {
    /// The minimum hue bound is greater than the maximum hue bound.
    InvalidHueRange,
    /// The minimum saturation bound is greater than the maximum saturation bound.
    InvalidSaturationRange,
    /// The minimum lightness bound is greater than the maximum lightness bound.
    InvalidLightnessRange,
    /// The minimum alpha bound is greater than the maximum alpha bound.
    InvalidAlphaRange,
    /// A component bound is outside the supported range.
    ComponentOutOfBounds,
    /// A component bound is infinite or NaN.
    NonFiniteValue,
}

impl fmt::Display for HslError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidHueRange => "hue min value must be less than or equal to max",
            Self::InvalidSaturationRange => {
                "saturation min value must be less than or equal to max"
            }
            Self::InvalidLightnessRange => "lightness min value must be less than or equal to max",
            Self::InvalidAlphaRange => "alpha min value must be less than or equal to max",
            Self::ComponentOutOfBounds => {
                "hue must be 0.0..=360.0, saturation/lightness 0.0..=100.0, alpha 0.0..=1.0"
            }
            Self::NonFiniteValue => "all HSL components must be finite",
        };
        f.write_str(message)
    }
}

impl Error for HslError {}

/// Generates a random HSL color using default bounds.
///
/// # Examples
///
/// ```rust
/// use rand_hsl::random_hsl;
///
/// let color = random_hsl();
/// assert!((0.0..=360.0).contains(&color.hue));
/// ```
pub fn random_hsl() -> HslColor {
    let mut rng = rand::thread_rng();
    random_hsl_with_rng(&mut rng)
}

/// Generates a random HSL color with a caller-provided RNG.
///
/// # Examples
///
/// ```rust
/// use rand::rngs::StdRng;
/// use rand::SeedableRng;
/// use rand_hsl::random_hsl_with_rng;
///
/// let mut rng_a = StdRng::seed_from_u64(42);
/// let mut rng_b = StdRng::seed_from_u64(42);
///
/// assert_eq!(random_hsl_with_rng(&mut rng_a), random_hsl_with_rng(&mut rng_b));
/// ```
pub fn random_hsl_with_rng<R: Rng + ?Sized>(rng: &mut R) -> HslColor {
    random_hsl_in_with_rng(HslRange::default(), rng)
        .expect("default hsl range should always be valid")
}

/// Generates a random HSL color using custom bounds.
///
/// # Errors
///
/// Returns a [`HslError`] when provided bounds are invalid.
pub fn random_hsl_in(range: HslRange) -> Result<HslColor, HslError> {
    let mut rng = rand::thread_rng();
    random_hsl_in_with_rng(range, &mut rng)
}

/// Generates a random HSL color using custom bounds and a caller-provided RNG.
///
/// # Errors
///
/// Returns a [`HslError`] when provided bounds are invalid.
pub fn random_hsl_in_with_rng<R: Rng + ?Sized>(
    range: HslRange,
    rng: &mut R,
) -> Result<HslColor, HslError> {
    validate_range(&range)?;

    Ok(HslColor {
        hue: rng.gen_range(range.hue.0..=range.hue.1),
        saturation: rng.gen_range(range.saturation.0..=range.saturation.1),
        lightness: rng.gen_range(range.lightness.0..=range.lightness.1),
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

fn validate_range(range: &HslRange) -> Result<(), HslError> {
    let numbers = [
        range.hue.0,
        range.hue.1,
        range.saturation.0,
        range.saturation.1,
        range.lightness.0,
        range.lightness.1,
        range.alpha.0,
        range.alpha.1,
    ];

    if numbers.iter().any(|value| !value.is_finite()) {
        return Err(HslError::NonFiniteValue);
    }

    if range.hue.0 > range.hue.1 {
        return Err(HslError::InvalidHueRange);
    }
    if range.saturation.0 > range.saturation.1 {
        return Err(HslError::InvalidSaturationRange);
    }
    if range.lightness.0 > range.lightness.1 {
        return Err(HslError::InvalidLightnessRange);
    }
    if range.alpha.0 > range.alpha.1 {
        return Err(HslError::InvalidAlphaRange);
    }

    if range.hue.0 < 0.0
        || range.hue.1 > 360.0
        || range.saturation.0 < 0.0
        || range.saturation.1 > 100.0
        || range.lightness.0 < 0.0
        || range.lightness.1 > 100.0
        || range.alpha.0 < 0.0
        || range.alpha.1 > 1.0
    {
        return Err(HslError::ComponentOutOfBounds);
    }

    Ok(())
}

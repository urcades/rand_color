#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;
use std::ops::RangeInclusive;

/// Represents a randomly generated `rgba()` color value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f32,
}

impl RandomColor {
    /// Formats the color as an `rgba(r, g, b, a)` string.
    ///
    /// Alpha is displayed with exactly two decimal places.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rand_rgb::RandomColor;
    ///
    /// let color = RandomColor {
    ///     red: 179,
    ///     green: 134,
    ///     blue: 103,
    ///     alpha: 0.3333,
    /// };
    ///
    /// assert_eq!(color.to_rgba_string(), "rgba(179, 134, 103, 0.33)");
    /// ```
    pub fn to_rgba_string(&self) -> String {
        format!(
            "rgba({}, {}, {}, {:.2})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

/// User-provided channel bounds for random generation.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorRange {
    pub red: RangeInclusive<u8>,
    pub green: RangeInclusive<u8>,
    pub blue: RangeInclusive<u8>,
    pub alpha: (f32, f32),
}

impl Default for ColorRange {
    fn default() -> Self {
        Self {
            red: 0..=255,
            green: 0..=255,
            blue: 0..=255,
            alpha: (0.0, 1.0),
        }
    }
}

impl ColorRange {
    /// Builds a new set of bounds.
    ///
    /// # Errors
    ///
    /// Returns an error when any range is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rand_rgb::ColorRange;
    ///
    /// let range = ColorRange::new(100, 200, 100, 200, 50, 150, 0.2, 0.8).unwrap();
    /// assert_eq!(range.alpha, (0.2, 0.8));
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_red: u8,
        max_red: u8,
        min_green: u8,
        max_green: u8,
        min_blue: u8,
        max_blue: u8,
        min_alpha: f32,
        max_alpha: f32,
    ) -> Result<Self, ColorError> {
        let range = Self {
            red: min_red..=max_red,
            green: min_green..=max_green,
            blue: min_blue..=max_blue,
            alpha: (min_alpha, max_alpha),
        };
        validate_range(&range)?;
        Ok(range)
    }
}

/// Error returned when color bounds are invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorError {
    InvalidChannelRange,
    InvalidAlphaRange,
    AlphaOutOfBounds,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidChannelRange => "channel min value must be less than or equal to max",
            Self::InvalidAlphaRange => "alpha min value must be less than or equal to max",
            Self::AlphaOutOfBounds => "alpha must be finite and within 0.0..=1.0",
        };
        f.write_str(message)
    }
}

impl Error for ColorError {}

/// Generates a random color using default bounds.
///
/// Defaults:
/// - `red`: `0..=255`
/// - `green`: `0..=255`
/// - `blue`: `0..=255`
/// - `alpha`: `0.0..=1.0`
///
/// # Examples
///
/// ```rust
/// use rand_rgb::random_color;
///
/// let color = random_color();
/// assert!((0.0..=1.0).contains(&color.alpha));
/// ```
pub fn random_color() -> RandomColor {
    random_color_in(ColorRange::default()).expect("default color range should always be valid")
}

/// Generates a random color using custom bounds.
///
/// # Errors
///
/// Returns a [`ColorError`] when provided bounds are invalid.
pub fn random_color_in(range: ColorRange) -> Result<RandomColor, ColorError> {
    validate_range(&range)?;

    let mut rng = rand::thread_rng();
    Ok(RandomColor {
        red: rng.gen_range(range.red),
        green: rng.gen_range(range.green),
        blue: rng.gen_range(range.blue),
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

impl RandomColor {
    /// Generates a random color struct from explicit bounds.
    #[deprecated(note = "Use random_color_in(ColorRange::new(...)? ) instead")]
    #[allow(clippy::too_many_arguments)]
    pub fn rand_color_struct(
        min_red: u8,
        max_red: u8,
        min_green: u8,
        max_green: u8,
        min_blue: u8,
        max_blue: u8,
        min_alpha: f32,
        max_alpha: f32,
    ) -> Self {
        let range = ColorRange::new(
            min_red, max_red, min_green, max_green, min_blue, max_blue, min_alpha, max_alpha,
        )
        .expect("invalid color range");

        random_color_in(range).expect("color generation failed")
    }

    /// Generates a random `rgba(...)` string from explicit bounds.
    #[deprecated(note = "Use random_color_in(ColorRange::new(...)? )?.to_rgba_string() instead")]
    #[allow(clippy::too_many_arguments)]
    pub fn rand_color_string(
        min_red: u8,
        max_red: u8,
        min_green: u8,
        max_green: u8,
        min_blue: u8,
        max_blue: u8,
        min_alpha: f32,
        max_alpha: f32,
    ) -> String {
        let range = ColorRange::new(
            min_red, max_red, min_green, max_green, min_blue, max_blue, min_alpha, max_alpha,
        )
        .expect("invalid color range");

        let color = random_color_in(range).expect("color generation failed");
        color.to_rgba_string()
    }
}

fn validate_range(range: &ColorRange) -> Result<(), ColorError> {
    if range.red.start() > range.red.end()
        || range.green.start() > range.green.end()
        || range.blue.start() > range.blue.end()
    {
        return Err(ColorError::InvalidChannelRange);
    }

    let (min_alpha, max_alpha) = range.alpha;
    if !min_alpha.is_finite() || !max_alpha.is_finite() {
        return Err(ColorError::AlphaOutOfBounds);
    }
    if min_alpha > max_alpha {
        return Err(ColorError::InvalidAlphaRange);
    }
    if min_alpha < 0.0 || max_alpha > 1.0 {
        return Err(ColorError::AlphaOutOfBounds);
    }

    Ok(())
}

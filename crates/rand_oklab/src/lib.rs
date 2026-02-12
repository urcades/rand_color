#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OklabColor {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
}

impl OklabColor {
    pub fn to_oklab_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for OklabColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "oklab({:.3}, {:.3}, {:.3}, {:.2})",
            self.lightness, self.a, self.b, self.alpha
        )
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct OklabRange {
    pub lightness: (f32, f32),
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub alpha: (f32, f32),
}

impl Default for OklabRange {
    fn default() -> Self {
        Self {
            lightness: (0.0, 1.0),
            a: (-0.4, 0.4),
            b: (-0.4, 0.4),
            alpha: (0.0, 1.0),
        }
    }
}

impl OklabRange {
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
    ) -> Result<Self, OklabError> {
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
pub enum OklabError {
    InvalidLightnessRange,
    InvalidARange,
    InvalidBRange,
    InvalidAlphaRange,
    ComponentOutOfBounds,
    NonFiniteValue,
}

impl fmt::Display for OklabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidLightnessRange => "lightness min value must be <= max",
            Self::InvalidARange => "a min value must be <= max",
            Self::InvalidBRange => "b min value must be <= max",
            Self::InvalidAlphaRange => "alpha min value must be <= max",
            Self::ComponentOutOfBounds => "lightness 0..=1, a/b -0.4..=0.4, alpha 0..=1",
            Self::NonFiniteValue => "all Oklab components must be finite",
        };
        f.write_str(message)
    }
}

impl Error for OklabError {}

pub fn random_oklab() -> OklabColor {
    let mut rng = rand::thread_rng();
    random_oklab_with_rng(&mut rng)
}

pub fn random_oklab_with_rng<R: Rng + ?Sized>(rng: &mut R) -> OklabColor {
    random_oklab_in_with_rng(OklabRange::default(), rng)
        .expect("default oklab range should be valid")
}

pub fn random_oklab_in(range: OklabRange) -> Result<OklabColor, OklabError> {
    let mut rng = rand::thread_rng();
    random_oklab_in_with_rng(range, &mut rng)
}

pub fn random_oklab_in_with_rng<R: Rng + ?Sized>(
    range: OklabRange,
    rng: &mut R,
) -> Result<OklabColor, OklabError> {
    validate_range(&range)?;
    Ok(OklabColor {
        lightness: rng.gen_range(range.lightness.0..=range.lightness.1),
        a: rng.gen_range(range.a.0..=range.a.1),
        b: rng.gen_range(range.b.0..=range.b.1),
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

fn validate_range(range: &OklabRange) -> Result<(), OklabError> {
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
        return Err(OklabError::NonFiniteValue);
    }
    if range.lightness.0 > range.lightness.1 {
        return Err(OklabError::InvalidLightnessRange);
    }
    if range.a.0 > range.a.1 {
        return Err(OklabError::InvalidARange);
    }
    if range.b.0 > range.b.1 {
        return Err(OklabError::InvalidBRange);
    }
    if range.alpha.0 > range.alpha.1 {
        return Err(OklabError::InvalidAlphaRange);
    }
    if range.lightness.0 < 0.0
        || range.lightness.1 > 1.0
        || range.a.0 < -0.4
        || range.a.1 > 0.4
        || range.b.0 < -0.4
        || range.b.1 > 0.4
        || range.alpha.0 < 0.0
        || range.alpha.1 > 1.0
    {
        return Err(OklabError::ComponentOutOfBounds);
    }
    Ok(())
}

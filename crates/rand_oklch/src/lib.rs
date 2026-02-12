#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OklchColor {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}

impl OklchColor {
    pub fn to_oklch_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for OklchColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "oklch({:.3}, {:.3}, {:.1}, {:.2})",
            self.lightness, self.chroma, self.hue, self.alpha
        )
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct OklchRange {
    pub lightness: (f32, f32),
    pub chroma: (f32, f32),
    pub hue: (f32, f32),
    pub alpha: (f32, f32),
}

impl Default for OklchRange {
    fn default() -> Self {
        Self {
            lightness: (0.0, 1.0),
            chroma: (0.0, 0.4),
            hue: (0.0, 360.0),
            alpha: (0.0, 1.0),
        }
    }
}

impl OklchRange {
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
    ) -> Result<Self, OklchError> {
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
pub enum OklchError {
    InvalidLightnessRange,
    InvalidChromaRange,
    InvalidHueRange,
    InvalidAlphaRange,
    ComponentOutOfBounds,
    NonFiniteValue,
}

impl fmt::Display for OklchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidLightnessRange => "lightness min value must be <= max",
            Self::InvalidChromaRange => "chroma min value must be <= max",
            Self::InvalidHueRange => "hue min value must be <= max",
            Self::InvalidAlphaRange => "alpha min value must be <= max",
            Self::ComponentOutOfBounds => {
                "lightness 0..=1, chroma 0..=0.4, hue 0..=360, alpha 0..=1"
            }
            Self::NonFiniteValue => "all Oklch components must be finite",
        };
        f.write_str(message)
    }
}

impl Error for OklchError {}

pub fn random_oklch() -> OklchColor {
    let mut rng = rand::thread_rng();
    random_oklch_with_rng(&mut rng)
}

pub fn random_oklch_with_rng<R: Rng + ?Sized>(rng: &mut R) -> OklchColor {
    random_oklch_in_with_rng(OklchRange::default(), rng)
        .expect("default oklch range should be valid")
}

pub fn random_oklch_in(range: OklchRange) -> Result<OklchColor, OklchError> {
    let mut rng = rand::thread_rng();
    random_oklch_in_with_rng(range, &mut rng)
}

pub fn random_oklch_in_with_rng<R: Rng + ?Sized>(
    range: OklchRange,
    rng: &mut R,
) -> Result<OklchColor, OklchError> {
    validate_range(&range)?;
    Ok(OklchColor {
        lightness: rng.gen_range(range.lightness.0..=range.lightness.1),
        chroma: rng.gen_range(range.chroma.0..=range.chroma.1),
        hue: rng.gen_range(range.hue.0..=range.hue.1),
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

fn validate_range(range: &OklchRange) -> Result<(), OklchError> {
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
        return Err(OklchError::NonFiniteValue);
    }
    if range.lightness.0 > range.lightness.1 {
        return Err(OklchError::InvalidLightnessRange);
    }
    if range.chroma.0 > range.chroma.1 {
        return Err(OklchError::InvalidChromaRange);
    }
    if range.hue.0 > range.hue.1 {
        return Err(OklchError::InvalidHueRange);
    }
    if range.alpha.0 > range.alpha.1 {
        return Err(OklchError::InvalidAlphaRange);
    }
    if range.lightness.0 < 0.0
        || range.lightness.1 > 1.0
        || range.chroma.0 < 0.0
        || range.chroma.1 > 0.4
        || range.hue.0 < 0.0
        || range.hue.1 > 360.0
        || range.alpha.0 < 0.0
        || range.alpha.1 > 1.0
    {
        return Err(OklchError::ComponentOutOfBounds);
    }
    Ok(())
}

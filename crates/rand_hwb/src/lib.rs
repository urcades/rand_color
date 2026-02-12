#[cfg(test)]
mod tests;

use rand::Rng;
use std::error::Error;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HwbColor {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
    pub alpha: f32,
}

impl HwbColor {
    pub fn to_hwba_string(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for HwbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hwba({:.1}, {:.1}%, {:.1}%, {:.2})",
            self.hue, self.whiteness, self.blackness, self.alpha
        )
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct HwbRange {
    pub hue: (f32, f32),
    pub whiteness: (f32, f32),
    pub blackness: (f32, f32),
    pub alpha: (f32, f32),
}

impl Default for HwbRange {
    fn default() -> Self {
        Self {
            hue: (0.0, 360.0),
            whiteness: (0.0, 100.0),
            blackness: (0.0, 100.0),
            alpha: (0.0, 1.0),
        }
    }
}

impl HwbRange {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_hue: f32,
        max_hue: f32,
        min_whiteness: f32,
        max_whiteness: f32,
        min_blackness: f32,
        max_blackness: f32,
        min_alpha: f32,
        max_alpha: f32,
    ) -> Result<Self, HwbError> {
        let range = Self {
            hue: (min_hue, max_hue),
            whiteness: (min_whiteness, max_whiteness),
            blackness: (min_blackness, max_blackness),
            alpha: (min_alpha, max_alpha),
        };
        validate_range(&range)?;
        Ok(range)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HwbError {
    InvalidHueRange,
    InvalidWhitenessRange,
    InvalidBlacknessRange,
    InvalidAlphaRange,
    InvalidWhitenessBlacknessCombination,
    ComponentOutOfBounds,
    NonFiniteValue,
}

impl fmt::Display for HwbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidHueRange => "hue min value must be <= max",
            Self::InvalidWhitenessRange => "whiteness min value must be <= max",
            Self::InvalidBlacknessRange => "blackness min value must be <= max",
            Self::InvalidAlphaRange => "alpha min value must be <= max",
            Self::InvalidWhitenessBlacknessCombination => "whiteness + blackness must be <= 100",
            Self::ComponentOutOfBounds => {
                "hue must be 0..=360, whiteness/blackness 0..=100, alpha 0..=1"
            }
            Self::NonFiniteValue => "all HWB components must be finite",
        };
        f.write_str(message)
    }
}

impl Error for HwbError {}

pub fn random_hwb() -> HwbColor {
    let mut rng = rand::thread_rng();
    random_hwb_with_rng(&mut rng)
}

pub fn random_hwb_with_rng<R: Rng + ?Sized>(rng: &mut R) -> HwbColor {
    random_hwb_in_with_rng(HwbRange::default(), rng).expect("default hwb range should be valid")
}

pub fn random_hwb_in(range: HwbRange) -> Result<HwbColor, HwbError> {
    let mut rng = rand::thread_rng();
    random_hwb_in_with_rng(range, &mut rng)
}

pub fn random_hwb_in_with_rng<R: Rng + ?Sized>(
    range: HwbRange,
    rng: &mut R,
) -> Result<HwbColor, HwbError> {
    validate_range(&range)?;
    let mut whiteness = rng.gen_range(range.whiteness.0..=range.whiteness.1);
    let mut blackness = rng.gen_range(range.blackness.0..=range.blackness.1);
    let total = whiteness + blackness;
    if total > 100.0 {
        let scale = 100.0 / total;
        whiteness *= scale;
        blackness *= scale;
    }
    Ok(HwbColor {
        hue: rng.gen_range(range.hue.0..=range.hue.1),
        whiteness,
        blackness,
        alpha: rng.gen_range(range.alpha.0..=range.alpha.1),
    })
}

fn validate_range(range: &HwbRange) -> Result<(), HwbError> {
    let numbers = [
        range.hue.0,
        range.hue.1,
        range.whiteness.0,
        range.whiteness.1,
        range.blackness.0,
        range.blackness.1,
        range.alpha.0,
        range.alpha.1,
    ];
    if numbers.iter().any(|value| !value.is_finite()) {
        return Err(HwbError::NonFiniteValue);
    }
    if range.hue.0 > range.hue.1 {
        return Err(HwbError::InvalidHueRange);
    }
    if range.whiteness.0 > range.whiteness.1 {
        return Err(HwbError::InvalidWhitenessRange);
    }
    if range.blackness.0 > range.blackness.1 {
        return Err(HwbError::InvalidBlacknessRange);
    }
    if range.alpha.0 > range.alpha.1 {
        return Err(HwbError::InvalidAlphaRange);
    }
    if range.hue.0 < 0.0
        || range.hue.1 > 360.0
        || range.whiteness.0 < 0.0
        || range.whiteness.1 > 100.0
        || range.blackness.0 < 0.0
        || range.blackness.1 > 100.0
        || range.alpha.0 < 0.0
        || range.alpha.1 > 1.0
    {
        return Err(HwbError::ComponentOutOfBounds);
    }
    if range.whiteness.0 + range.blackness.0 > 100.0 {
        return Err(HwbError::InvalidWhitenessBlacknessCombination);
    }
    Ok(())
}

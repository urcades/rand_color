//! Unified random color generation across supported color spaces.
//!
//! This umbrella crate re-exports the focused color-space crates from stable
//! modules. Use a focused crate such as `rand_rgb` when you only need one color
//! space, or use `rand_color` when you want one dependency for several spaces.
//!
//! Random values are sampled from numeric component ranges. They are not
//! gamut-checked, contrast-checked, or perceptually uniform.
//!
//! # Examples
//!
//! ```rust
//! let rgb = rand_color::rgb::random_color();
//! let hsl = rand_color::hsl::random_hsl();
//! let hsv = rand_color::hsv::random_hsv();
//!
//! assert!(rgb.to_rgba_string().starts_with("rgba("));
//! assert!(hsl.to_hsla_string().starts_with("hsla("));
//! assert!(hsv.to_hsva_string().starts_with("hsva("));
//! ```

#![warn(missing_docs)]

#[cfg(test)]
mod tests;

/// RGB/RGBA random color generation from `rand_rgb`.
pub mod rgb {
    pub use rand_rgb::*;
}

/// HSL/HSLA random color generation from `rand_hsl`.
pub mod hsl {
    pub use rand_hsl::*;
}

/// HSV/HSVA random color generation from `rand_hsv`.
pub mod hsv {
    pub use rand_hsv::*;
}

/// HWB/HWBA random color generation from `rand_hwb`.
pub mod hwb {
    pub use rand_hwb::*;
}

/// Oklab random color generation from `rand_oklab`.
pub mod oklab {
    pub use rand_oklab::*;
}

/// Oklch random color generation from `rand_oklch`.
pub mod oklch {
    pub use rand_oklch::*;
}

/// CIELAB random color generation from `rand_lab`.
pub mod lab {
    pub use rand_lab::*;
}

/// CIELCH random color generation from `rand_lch`.
pub mod lch {
    pub use rand_lch::*;
}

/// RGB <-> HSL conversion helpers and traits from `rand_color_convert`.
pub mod convert {
    pub use rand_color_convert::*;
}

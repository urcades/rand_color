//! Unified random color generation across supported color spaces.
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

#[cfg(test)]
mod tests;

pub mod rgb {
    pub use rand_rgb::*;
}

pub mod hsl {
    pub use rand_hsl::*;
}

pub mod hsv {
    pub use rand_hsv::*;
}

pub mod hwb {
    pub use rand_hwb::*;
}

pub mod oklab {
    pub use rand_oklab::*;
}

pub mod oklch {
    pub use rand_oklch::*;
}

pub mod lab {
    pub use rand_lab::*;
}

pub mod lch {
    pub use rand_lch::*;
}

pub mod convert {
    pub use rand_color_convert::*;
}

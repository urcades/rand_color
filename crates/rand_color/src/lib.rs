//! Unified random color generation across supported color spaces.
//!
//! # Examples
//!
//! ```rust
//! let rgb = rand_color::rgb::random_color();
//! let hsl = rand_color::hsl::random_hsl();
//!
//! assert!(rgb.to_rgba_string().starts_with("rgba("));
//! assert!(hsl.to_hsla_string().starts_with("hsla("));
//! ```

#[cfg(test)]
mod tests;

pub mod rgb {
    pub use rand_rgb::*;
}

pub mod hsl {
    pub use rand_hsl::*;
}

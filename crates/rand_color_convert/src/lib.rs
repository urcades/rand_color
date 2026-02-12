#[cfg(test)]
mod tests;

use rand_hsl::HslColor;
use rand_rgb::RandomColor;

pub trait ToRgb {
    fn to_rgb(&self) -> RandomColor;
}

pub trait ToHsl {
    fn to_hsl(&self) -> HslColor;
}

impl ToHsl for RandomColor {
    fn to_hsl(&self) -> HslColor {
        rgb_to_hsl(*self)
    }
}

impl ToRgb for HslColor {
    fn to_rgb(&self) -> RandomColor {
        hsl_to_rgb(*self)
    }
}

pub fn rgb_to_hsl(color: RandomColor) -> HslColor {
    let r = color.red as f32 / 255.0;
    let g = color.green as f32 / 255.0;
    let b = color.blue as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let mut hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    if hue < 0.0 {
        hue += 360.0;
    }

    let lightness = (max + min) / 2.0;
    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };

    HslColor {
        hue,
        saturation: saturation * 100.0,
        lightness: lightness * 100.0,
        alpha: color.alpha,
    }
}

pub fn hsl_to_rgb(color: HslColor) -> RandomColor {
    let hue = color.hue.rem_euclid(360.0);
    let saturation = (color.saturation / 100.0).clamp(0.0, 1.0);
    let lightness = (color.lightness / 100.0).clamp(0.0, 1.0);

    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = c * (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs());
    let m = lightness - c / 2.0;

    let (r1, g1, b1) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    RandomColor {
        red: ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        green: ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        blue: ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        alpha: color.alpha,
    }
}

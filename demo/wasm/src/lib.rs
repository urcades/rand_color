use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_color::convert::{hsl_to_rgb, rgb_to_hsl};
use rand_color::{hsl, hsv, hwb, lab, lch, oklab, oklch, rgb};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct DemoColor {
    space: &'static str,
    crate_output: String,
    css_preview: String,
    components: String,
}

#[derive(Serialize)]
struct GeneratedSet {
    space: String,
    seed: u32,
    count: usize,
    snippet: String,
    colors: Vec<DemoColor>,
}

#[derive(Serialize)]
struct ConversionResult {
    input: String,
    hsl: String,
    round_trip: String,
    delta: String,
    input_css: String,
    round_trip_css: String,
    snippet: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct AvatarCell {
    x: u8,
    y: u8,
    fill: &'static str,
}

#[derive(Serialize)]
struct AvatarResult {
    key: String,
    seed: String,
    background: String,
    foreground: String,
    accent: String,
    cells: Vec<AvatarCell>,
    snippet: String,
}

#[wasm_bindgen]
pub fn generate_colors(space: &str, count: usize, seed: u32) -> Result<JsValue, JsValue> {
    let count = count.clamp(1, 96);
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let colors = (0..count)
        .map(|_| generate_one(space, &mut rng))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| JsValue::from_str(&error))?;

    let generated = GeneratedSet {
        space: space.to_string(),
        seed,
        count,
        snippet: snippet_for(space, seed),
        colors,
    };

    serde_wasm_bindgen::to_value(&generated).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub fn generate_all_spaces(seed: u32) -> Result<JsValue, JsValue> {
    let spaces = ["rgb", "hsl", "hsv", "hwb", "oklab", "oklch", "lab", "lch"];
    let mut colors = Vec::with_capacity(spaces.len());

    for (index, space) in spaces.iter().enumerate() {
        let mut rng = StdRng::seed_from_u64(seed as u64 + index as u64);
        colors.push(generate_one(space, &mut rng).map_err(|error| JsValue::from_str(&error))?);
    }

    serde_wasm_bindgen::to_value(&colors).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub fn convert_rgb_to_hsl(red: u8, green: u8, blue: u8, alpha: f32) -> Result<JsValue, JsValue> {
    let alpha = alpha.clamp(0.0, 1.0);
    let input = rgb::RandomColor {
        red,
        green,
        blue,
        alpha,
    };
    let hsl = rgb_to_hsl(input);
    let round_trip = hsl_to_rgb(hsl);
    let delta = format!(
        "red {:+}, green {:+}, blue {:+}",
        round_trip.red as i16 - input.red as i16,
        round_trip.green as i16 - input.green as i16,
        round_trip.blue as i16 - input.blue as i16
    );

    let result = ConversionResult {
        input: input.to_rgba_string(),
        hsl: hsl.to_hsla_string(),
        round_trip: round_trip.to_rgba_string(),
        delta,
        input_css: input.to_rgba_string(),
        round_trip_css: round_trip.to_rgba_string(),
        snippet: format!(
            "use rand_color::convert::{{hsl_to_rgb, rgb_to_hsl}};\nuse rand_color::rgb::RandomColor;\n\nlet rgb = RandomColor {{ red: {red}, green: {green}, blue: {blue}, alpha: {alpha:.2} }};\nlet hsl = rgb_to_hsl(rgb);\nlet round_trip = hsl_to_rgb(hsl);"
        ),
    };

    serde_wasm_bindgen::to_value(&result).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[wasm_bindgen]
pub fn generate_avatar(key: &str) -> Result<JsValue, JsValue> {
    let key = normalized_avatar_key(key);
    let seed = stable_hash(&key);
    let mut rng = StdRng::seed_from_u64(seed);

    let background = avatar_color(
        hsl::HslRange::new(0.0, 360.0, 35.0, 80.0, 84.0, 96.0, 1.0, 1.0)
            .expect("avatar background range should be valid"),
        &mut rng,
    )?;
    let foreground = avatar_color(
        hsl::HslRange::new(0.0, 360.0, 45.0, 95.0, 24.0, 48.0, 1.0, 1.0)
            .expect("avatar foreground range should be valid"),
        &mut rng,
    )?;
    let accent = avatar_color(
        hsl::HslRange::new(0.0, 360.0, 55.0, 95.0, 48.0, 68.0, 1.0, 1.0)
            .expect("avatar accent range should be valid"),
        &mut rng,
    )?;

    let avatar = AvatarResult {
        key: key.clone(),
        seed: format!("{seed:016x}"),
        background: background.to_hsla_string(),
        foreground: foreground.to_hsla_string(),
        accent: accent.to_hsla_string(),
        cells: avatar_cells(&mut rng),
        snippet: avatar_snippet(&key),
    };

    serde_wasm_bindgen::to_value(&avatar).map_err(|error| JsValue::from_str(&error.to_string()))
}

fn generate_one(space: &str, rng: &mut StdRng) -> Result<DemoColor, String> {
    match space {
        "rgb" => {
            let color = rgb::random_color_with_rng(rng);
            Ok(DemoColor {
                space: "RGB",
                crate_output: color.to_rgba_string(),
                css_preview: color.to_rgba_string(),
                components: format!(
                    "red {}, green {}, blue {}, alpha {:.2}",
                    color.red, color.green, color.blue, color.alpha
                ),
            })
        }
        "hsl" => {
            let color = hsl::random_hsl_with_rng(rng);
            Ok(DemoColor {
                space: "HSL",
                crate_output: color.to_hsla_string(),
                css_preview: color.to_hsla_string(),
                components: format!(
                    "hue {:.1}, saturation {:.1}%, lightness {:.1}%, alpha {:.2}",
                    color.hue, color.saturation, color.lightness, color.alpha
                ),
            })
        }
        "hsv" => {
            let color = hsv::random_hsv_with_rng(rng);
            Ok(DemoColor {
                space: "HSV",
                crate_output: color.to_hsva_string(),
                css_preview: hsv_to_rgba_css(color.hue, color.saturation, color.value, color.alpha),
                components: format!(
                    "hue {:.1}, saturation {:.1}%, value {:.1}%, alpha {:.2}",
                    color.hue, color.saturation, color.value, color.alpha
                ),
            })
        }
        "hwb" => {
            let color = hwb::random_hwb_with_rng(rng);
            Ok(DemoColor {
                space: "HWB",
                crate_output: color.to_hwba_string(),
                css_preview: format!(
                    "hwb({:.1} {:.1}% {:.1}% / {:.2})",
                    color.hue, color.whiteness, color.blackness, color.alpha
                ),
                components: format!(
                    "hue {:.1}, whiteness {:.1}%, blackness {:.1}%, alpha {:.2}",
                    color.hue, color.whiteness, color.blackness, color.alpha
                ),
            })
        }
        "oklab" => {
            let color = oklab::random_oklab_with_rng(rng);
            Ok(DemoColor {
                space: "Oklab",
                crate_output: color.to_oklab_string(),
                css_preview: format!(
                    "oklab({:.3} {:.3} {:.3} / {:.2})",
                    color.lightness, color.a, color.b, color.alpha
                ),
                components: format!(
                    "lightness {:.3}, a {:.3}, b {:.3}, alpha {:.2}",
                    color.lightness, color.a, color.b, color.alpha
                ),
            })
        }
        "oklch" => {
            let color = oklch::random_oklch_with_rng(rng);
            Ok(DemoColor {
                space: "Oklch",
                crate_output: color.to_oklch_string(),
                css_preview: format!(
                    "oklch({:.3} {:.3} {:.1} / {:.2})",
                    color.lightness, color.chroma, color.hue, color.alpha
                ),
                components: format!(
                    "lightness {:.3}, chroma {:.3}, hue {:.1}, alpha {:.2}",
                    color.lightness, color.chroma, color.hue, color.alpha
                ),
            })
        }
        "lab" => {
            let color = lab::random_lab_with_rng(rng);
            Ok(DemoColor {
                space: "Lab",
                crate_output: color.to_lab_string(),
                css_preview: format!(
                    "lab({:.2}% {:.2} {:.2} / {:.2})",
                    color.lightness, color.a, color.b, color.alpha
                ),
                components: format!(
                    "lightness {:.2}, a {:.2}, b {:.2}, alpha {:.2}",
                    color.lightness, color.a, color.b, color.alpha
                ),
            })
        }
        "lch" => {
            let color = lch::random_lch_with_rng(rng);
            Ok(DemoColor {
                space: "Lch",
                crate_output: color.to_lch_string(),
                css_preview: format!(
                    "lch({:.2}% {:.2} {:.1} / {:.2})",
                    color.lightness, color.chroma, color.hue, color.alpha
                ),
                components: format!(
                    "lightness {:.2}, chroma {:.2}, hue {:.1}, alpha {:.2}",
                    color.lightness, color.chroma, color.hue, color.alpha
                ),
            })
        }
        _ => Err(format!("unknown color space: {space}")),
    }
}

fn hsv_to_rgba_css(hue: f32, saturation: f32, value: f32, alpha: f32) -> String {
    let hue = hue.rem_euclid(360.0);
    let saturation = (saturation / 100.0).clamp(0.0, 1.0);
    let value = (value / 100.0).clamp(0.0, 1.0);
    let chroma = value * saturation;
    let x = chroma * (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs());
    let m = value - chroma;
    let (r1, g1, b1) = if hue < 60.0 {
        (chroma, x, 0.0)
    } else if hue < 120.0 {
        (x, chroma, 0.0)
    } else if hue < 180.0 {
        (0.0, chroma, x)
    } else if hue < 240.0 {
        (0.0, x, chroma)
    } else if hue < 300.0 {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };

    format!(
        "rgba({}, {}, {}, {:.2})",
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
        alpha
    )
}

fn snippet_for(space: &str, seed: u32) -> String {
    match space {
        "rgb" => snippet("rand_color::rgb::random_color_with_rng", seed),
        "hsl" => snippet("rand_color::hsl::random_hsl_with_rng", seed),
        "hsv" => snippet("rand_color::hsv::random_hsv_with_rng", seed),
        "hwb" => snippet("rand_color::hwb::random_hwb_with_rng", seed),
        "oklab" => snippet("rand_color::oklab::random_oklab_with_rng", seed),
        "oklch" => snippet("rand_color::oklch::random_oklch_with_rng", seed),
        "lab" => snippet("rand_color::lab::random_lab_with_rng", seed),
        "lch" => snippet("rand_color::lch::random_lch_with_rng", seed),
        _ => String::new(),
    }
}

fn snippet(function: &str, seed: u32) -> String {
    format!(
        "use rand::rngs::StdRng;\nuse rand::SeedableRng;\n\nlet mut rng = StdRng::seed_from_u64({seed});\nlet color = {function}(&mut rng);"
    )
}

fn avatar_color<R: Rng + ?Sized>(
    range: hsl::HslRange,
    rng: &mut R,
) -> Result<hsl::HslColor, JsValue> {
    hsl::random_hsl_in_with_rng(range, rng).map_err(|error| JsValue::from_str(&error.to_string()))
}

fn avatar_cells<R: Rng + ?Sized>(rng: &mut R) -> Vec<AvatarCell> {
    let mut cells = Vec::new();

    for y in 0..5 {
        for x in 0..3 {
            if rng.gen_bool(0.58) {
                let fill = if rng.gen_bool(0.18) {
                    "accent"
                } else {
                    "foreground"
                };
                cells.push(AvatarCell { x, y, fill });

                let mirror_x = 4 - x;
                if mirror_x != x {
                    cells.push(AvatarCell {
                        x: mirror_x,
                        y,
                        fill,
                    });
                }
            }
        }
    }

    if cells.is_empty() {
        cells.push(AvatarCell {
            x: 2,
            y: 2,
            fill: "foreground",
        });
    }

    cells
}

fn normalized_avatar_key(key: &str) -> String {
    let key = key.trim();
    if key.is_empty() {
        "rand_color".to_string()
    } else {
        key.to_string()
    }
}

fn stable_hash(input: &str) -> u64 {
    const FNV_OFFSET: u64 = 14_695_981_039_346_656_037;
    const FNV_PRIME: u64 = 1_099_511_628_211;

    input.as_bytes().iter().fold(FNV_OFFSET, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(FNV_PRIME)
    })
}

fn avatar_snippet(key: &str) -> String {
    format!(
        "use rand::rngs::StdRng;\nuse rand::SeedableRng;\nuse rand_color::hsl::{{random_hsl_in_with_rng, HslRange}};\n\nlet seed = stable_hash({});\nlet mut rng = StdRng::seed_from_u64(seed);\nlet background = random_hsl_in_with_rng(\n    HslRange::new(0.0, 360.0, 35.0, 80.0, 84.0, 96.0, 1.0, 1.0).unwrap(),\n    &mut rng,\n).unwrap();",
        rust_string_literal(key)
    )
}

fn rust_string_literal(value: &str) -> String {
    let mut output = String::from("\"");

    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character => output.push(character),
        }
    }

    output.push('"');
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_hash_is_known_for_demo_key() {
        assert_eq!(
            stable_hash("7f35a4db-9d18-4696-bf5a-fc4e835ef9bd"),
            0x44df_fd35_42cc_fca4
        );
    }

    #[test]
    fn avatar_cells_are_mirrored_across_center_column() {
        let mut rng = StdRng::seed_from_u64(42);
        let cells = avatar_cells(&mut rng);

        for cell in &cells {
            let mirror_x = 4 - cell.x;
            assert!(
                cells.iter().any(|other| other.x == mirror_x
                    && other.y == cell.y
                    && other.fill == cell.fill),
                "missing mirror for {cell:?}"
            );
        }
    }

    #[test]
    fn rust_string_literal_escapes_special_characters() {
        assert_eq!(
            rust_string_literal("avatar \"one\"\nnext"),
            "\"avatar \\\"one\\\"\\nnext\""
        );
    }
}

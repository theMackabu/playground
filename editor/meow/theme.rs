use crate::constants::HIGHLIGHT_NAMES;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("TOML deserialization error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("malformed hex code: {0}")]
    InvalidHexCode(String),
    #[error("malformed hex byte: {0}")]
    InvalidHexByte(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Theme {
    pub styles: FxHashMap<String, Style>,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Debug, Clone)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);

    pub const LIGHT_RED: Self = Self::new(255, 85, 85);
    pub const LIGHT_GREEN: Self = Self::new(164, 225, 133);
    pub const LIGHT_YELLOW: Self = Self::new(231, 205, 125);
    pub const LIGHT_BLUE: Self = Self::new(103, 179, 255);
    pub const LIGHT_MAGENTA: Self = Self::new(205, 162, 244);
    pub const LIGHT_CYAN: Self = Self::new(48, 232, 233);
    pub const LIGHT_GRAY: Self = Self::new(142, 178, 217);

    pub const RED: Self = Self::new(255, 139, 126);
    pub const GREEN: Self = Self::new(45, 232, 170);
    pub const YELLOW: Self = Self::new(231, 205, 125);
    pub const BLUE: Self = Self::new(78, 162, 193);
    pub const MAGENTA: Self = Self::new(205, 162, 244);
    pub const CYAN: Self = Self::new(48, 232, 233);
    pub const GRAY: Self = Self::new(142, 178, 217);

    pub const fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }

    pub fn from_hex(src: impl AsRef<str>) -> Result<Self> {
        let src = src.as_ref();
        let hex = src.trim_start_matches('#').trim();

        if hex.len() < 6 || hex.len() > 8 || !hex.is_ascii() {
            let err = src.to_owned();

            return Err(Error::InvalidHexCode(err));
        }

        let r = u8::from_str_radix(&hex[0..=1], 16)?;
        let g = u8::from_str_radix(&hex[2..=3], 16)?;
        let b = u8::from_str_radix(&hex[4..=5], 16)?;

        Ok(Self { r, g, b })
    }
}

impl TryFrom<String> for Color {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> { Self::from_hex(value) }
}

impl Theme {
    pub fn get_style(&self, name: &str) -> Option<&Style> { self.styles.get(name) }

    pub fn get_theme(data: &str) -> Result<Self> {
        #[derive(Debug, Clone, Deserialize)]
        #[serde(untagged)]
        enum RawStyle {
            Simple(String),
            Complex { fg: Option<String>, bg: Option<String> },
        }

        #[derive(Debug, Clone, Deserialize)]
        struct RawTheme {
            #[serde(default = "default_palette")]
            palette: FxHashMap<String, Color>,
            #[serde(flatten)]
            styles: FxHashMap<String, RawStyle>,
        }

        impl RawTheme {
            fn dereference_color(&self, key: &str) -> Result<Color> {
                match self.palette.get(key) {
                    Some(c) => Ok(*c),
                    None => Color::from_hex(key),
                }
            }
        }

        let mut raw = toml::from_str::<RawTheme>(data)?;

        raw.palette = default_palette_colors().chain(raw.palette).collect();

        let to_style = |s: &RawStyle| -> Result<_> {
            match s {
                RawStyle::Simple(s) => Ok(Style {
                    fg: raw.dereference_color(s)?.into(),
                    ..Default::default()
                }),
                RawStyle::Complex { fg, bg } => {
                    let fg = match fg {
                        Some(s) => raw.dereference_color(s)?.into(),
                        None => None,
                    };

                    let bg = match bg {
                        Some(s) => raw.dereference_color(s)?.into(),
                        None => None,
                    };

                    Ok(Style { fg, bg })
                }
            }
        };

        let mut styles = FxHashMap::default();

        for name in HIGHLIGHT_NAMES {
            if let Some(s) = raw.styles.get(*name) {
                styles.insert(name.to_string(), to_style(s)?);
            }
        }

        let fg = match raw.styles.get("ui.text") {
            Some(r) => to_style(r)?.fg.unwrap_or(Color::WHITE),
            None => Color::WHITE,
        };

        let bg = match raw.styles.get("ui.background") {
            Some(r) => to_style(r)?.bg.unwrap_or(Color::BLACK),
            None => Color::BLACK,
        };

        for name in HIGHLIGHT_NAMES {
            if styles.contains_key(*name) {
                continue;
            }

            let mut p = name.to_string();

            while let Some((ancestor, _)) = p.rsplit_once('.') {
                if let Some(s) = styles.get(ancestor) {
                    styles.insert(name.to_string(), s.clone());

                    break;
                }

                p = ancestor.to_string();
            }
        }

        Ok(Self { styles, fg, bg })
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::new(0, 0, 0).into(),
            bg: None,
        }
    }
}

impl From<Color> for Style {
    fn from(value: Color) -> Self {
        Self {
            fg: value.into(),
            ..Default::default()
        }
    }
}

fn default_palette_colors() -> impl Iterator<Item = (String, Color)> {
    [
        ("default", Color::WHITE),
        ("black", Color::BLACK),
        ("white", Color::WHITE),
        ("red", Color::RED),
        ("green", Color::GREEN),
        ("yellow", Color::YELLOW),
        ("blue", Color::BLUE),
        ("magenta", Color::MAGENTA),
        ("cyan", Color::CYAN),
        ("gray", Color::GRAY),
        ("light-red", Color::LIGHT_RED),
        ("light-green", Color::LIGHT_GREEN),
        ("light-yellow", Color::LIGHT_YELLOW),
        ("light-blue", Color::LIGHT_BLUE),
        ("light-magenta", Color::LIGHT_MAGENTA),
        ("light-cyan", Color::LIGHT_CYAN),
        ("light-gray", Color::LIGHT_GRAY),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
}

fn default_palette() -> FxHashMap<String, Color> { default_palette_colors().collect() }

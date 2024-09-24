#[macro_export]
macro_rules! define_colors {
	($($name:ident => {r: $r:expr, g: $g:expr, b: $b:expr}),* $(,)?) => {
		$(const $name: crossterm::style::Color = crossterm::style::Color::Rgb { r: $r, g: $g, b: $b };)*
		pub struct Colors;
		impl Colors {
			$(pub const $name: crossterm::style::Color = $name;)*
		}
    };
}

#[macro_export]
macro_rules! define_themes {
	($($name:ident => $path:expr),* $(,)?) => {
		$(pub const $name: &str = include_str!($path);)*

		pub fn list() -> Vec<String> {
			vec![$(stringify!($name).to_ascii_lowercase().replace('_', "-"),)*]
		}

		pub fn from_token(token: &str) -> Option<&'static str> {
			match token.to_ascii_uppercase().replace('-', "_").as_str() {
				$(stringify!($name) => Some($name),)*
				_ => None,
			}
		}
	};
}

#[macro_export]
macro_rules! crcolor {
    ($color:ident) => {
        (crossterm::style::Color::$color, None)
    };
}

#[macro_export]
macro_rules! color {
    ($color:ident) => {
        (Colors::$color, None)
    };
}

#[macro_export]
macro_rules! italic {
    ($color:ident) => {
        (Colors::$color, Some(Attribute::Italic))
    };
}

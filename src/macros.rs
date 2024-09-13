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

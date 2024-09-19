use crossterm::style::{Color, Stylize};
use home::home_dir;
use serde::Deserialize;
use std::{fs, io::Write};

#[derive(Deserialize, Default)]
pub struct Config {
    pub disable_mouse_interaction: Option<bool>,
    pub tab_width: Option<usize>,
    pub theme: Option<String>,
    pub relative_line_numbers: Option<bool>,
}

const CONFIG_TEMPLATE: &str = r#"# This is an example configuration file for meow editor.
# Uncomment and modify the values as needed.

# Disable mouse interaction (default: false)
# disable_mouse_interaction = false

# Set tab width (default: 4)
# tab_width = 4

# Set theme (default: none)
# theme = "monokai"

# Use relative line numbers (default: false)
# relative_line_numbers = false
"#;

pub fn load() -> Config {
    let home = home_dir().expect("Home directory exists");
    let config_dir = home.join(".config").join("meow");
    let config_path = config_dir.join("config.toml");

    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("{}", format!("Failed to create .config/meow directory: {e}").with(Color::Red));
            return Config::default();
        }
    }

    if !config_path.exists() {
        if let Ok(mut file) = fs::File::create(&config_path) {
            if let Err(e) = file.write_all(CONFIG_TEMPLATE.as_bytes()) {
                eprintln!("{}", format!("Failed to write config template: {e}").with(Color::Red));
                return Config::default();
            }
            println!("{}", format!("Created example configuration file at: {config_path:?}").with(crate::PINK));
            println!("{}\n", "You can edit this file to customize your meow editor settings.".with(crate::PINK));
        } else {
            eprintln!("{}", format!("Failed to create configuration file at: {config_path:?}").with(Color::Red));
            return Config::default();
        }
        Config::default()
    } else {
        fs::read_to_string(&config_path).ok().and_then(|config_str| toml::from_str(&config_str).ok()).unwrap_or_default()
    }
}

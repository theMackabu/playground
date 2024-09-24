use crossterm::style::{Color, Stylize};
use home::home_dir;
use serde::Deserialize;
use std::{borrow::Cow, fs, io::Write};

#[derive(Deserialize, Default)]
pub struct Config {
    pub disable_mouse_interaction: Option<bool>,
    pub tab_width: Option<usize>,
    pub theme: Option<String>,
    pub relative_line_numbers: Option<bool>,
    pub keybinds: Option<Keybinds>,
}

#[derive(Deserialize, Default)]
pub struct Keybinds {
    pub quit: Option<String>,
    pub save: Option<String>,
    pub discard_changes: Option<String>,
    pub undo: Option<String>,
    pub redo: Option<String>,
    pub copy: Option<String>,
    pub paste: Option<String>,
    pub cut: Option<String>,
    pub system_copy: Option<String>,
    pub system_paste: Option<String>,
    pub system_cut: Option<String>,
    pub move_up: Option<String>,
    pub move_down: Option<String>,
    pub move_left: Option<String>,
    pub move_right: Option<String>,
    pub move_word_left: Option<String>,
    pub move_word_right: Option<String>,
    pub move_to_start_of_line: Option<String>,
    pub move_to_end_of_line: Option<String>,
    pub page_up: Option<String>,
    pub page_down: Option<String>,
    pub half_page_up: Option<String>,
    pub half_page_down: Option<String>,
}

impl Keybinds {
    pub fn get_keybind(&self, key: &str) -> Cow<'_, str> {
        match key {
            "quit" => self.quit.as_deref(),
            "save" => self.save.as_deref(),
            "discard_changes" => self.discard_changes.as_deref(),
            "undo" => self.undo.as_deref(),
            "redo" => self.redo.as_deref(),
            "copy" => self.copy.as_deref(),
            "paste" => self.paste.as_deref(),
            "cut" => self.cut.as_deref(),
            "system_copy" => self.system_copy.as_deref(),
            "system_paste" => self.system_paste.as_deref(),
            "system_cut" => self.system_cut.as_deref(),
            "move_up" => self.move_up.as_deref(),
            "move_down" => self.move_down.as_deref(),
            "move_left" => self.move_left.as_deref(),
            "move_right" => self.move_right.as_deref(),
            "move_word_left" => self.move_word_left.as_deref(),
            "move_word_right" => self.move_word_right.as_deref(),
            "move_to_start_of_line" => self.move_to_start_of_line.as_deref(),
            "move_to_end_of_line" => self.move_to_end_of_line.as_deref(),
            "page_up" => self.page_up.as_deref(),
            "page_down" => self.page_down.as_deref(),
            "half_page_up" => self.half_page_up.as_deref(),
            "half_page_down" => self.half_page_down.as_deref(),
            _ => None,
        }
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Borrowed(Self::default_for(key)))
    }

    fn default_for(key: &str) -> &'static str {
        match key {
            "quit" => "Ctrl+Q",
            "save" => "Ctrl+S",
            "discard_changes" => "Ctrl+A",
            "undo" => "Ctrl+Z",
            "redo" => "Ctrl+Y",
            "copy" => "Ctrl+C",
            "paste" => "Ctrl+V",
            "cut" => "Ctrl+X",
            "system_copy" => "Alt+C",
            "system_paste" => "Alt+V",
            "system_cut" => "Alt+X",
            "move_up" => "Up",
            "move_down" => "Down",
            "move_left" => "Left",
            "move_right" => "Right",
            "move_word_left" => "Ctrl+Left",
            "move_word_right" => "Ctrl+Right",
            "move_to_start_of_line" => "Home",
            "move_to_end_of_line" => "End",
            "page_up" => "PageUp",
            "page_down" => "PageDown",
            "half_page_up" => "Ctrl+U",
            "half_page_down" => "Ctrl+D",
            _ => "",
        }
    }
}

#[cfg(feature = "debugger")]
impl std::fmt::Debug for Keybinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keybinds")
            .field("quit", &self.get_keybind("quit"))
            .field("save", &self.get_keybind("save"))
            .field("discard_changes", &self.get_keybind("discard_changes"))
            .field("undo", &self.get_keybind("undo"))
            .field("redo", &self.get_keybind("redo"))
            .field("copy", &self.get_keybind("copy"))
            .field("paste", &self.get_keybind("paste"))
            .field("cut", &self.get_keybind("cut"))
            .field("system_copy", &self.get_keybind("system_copy"))
            .field("system_paste", &self.get_keybind("system_paste"))
            .field("system_cut", &self.get_keybind("system_cut"))
            .field("move_up", &self.get_keybind("move_up"))
            .field("move_down", &self.get_keybind("move_down"))
            .field("move_left", &self.get_keybind("move_left"))
            .field("move_right", &self.get_keybind("move_right"))
            .field("move_word_left", &self.get_keybind("move_word_left"))
            .field("move_word_right", &self.get_keybind("move_word_right"))
            .field("move_to_start_of_line", &self.get_keybind("move_to_start_of_line"))
            .field("move_to_end_of_line", &self.get_keybind("move_to_end_of_line"))
            .field("page_up", &self.get_keybind("page_up"))
            .field("page_down", &self.get_keybind("page_down"))
            .field("half_page_up", &self.get_keybind("half_page_up"))
            .field("half_page_down", &self.get_keybind("half_page_down"))
            .finish()
    }
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

# Keybinds configuration
# Format: "Modifier+Key" (e.g., "Ctrl+S", "Alt+X")
# Available modifiers: Ctrl, Alt, Shift
# For special keys, use their names (e.g., "Enter", "Backspace", "Tab")
# [keybinds]
# quit = "Ctrl+Q"
# save = "Ctrl+S"
# discard_changes = "Ctrl+A"
# undo = "Ctrl+Z"
# redo = "Ctrl+Y"
# copy = "Ctrl+C"
# paste = "Ctrl+V"
# cut = "Ctrl+X"
# system_copy = "Alt+C"
# system_paste = "Alt+V"
# system_cut = "Alt+X"
# move_up = "Up"
# move_down = "Down"
# move_left = "Left"
# move_right = "Right"
# move_word_left = "Ctrl+Left"
# move_word_right = "Ctrl+Right"
# move_to_start_of_line = "Home"
# move_to_end_of_line = "End"
# page_up = "PageUp"
# page_down = "PageDown"
# half_page_up = "Ctrl+U"
# half_page_down = "Ctrl+D"
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

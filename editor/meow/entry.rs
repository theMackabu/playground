mod clipboard;
mod config;
mod constants;
mod editor;
mod languages;
mod macros;
mod terminal;
mod theme;
mod ui;
mod unicode;
mod utils;
mod widgets;
mod widgets_impl;

use clap::Parser;
use clipboard::Clipboard;
use editor::*;
use terminal::*;
use theme::Theme;
use ui::*;
use widgets::*;
use widgets_impl::*;

#[cfg(feature = "debugger")]
mod debugger {
    use tracing_live::Log;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    pub fn init() { tracing_subscriber::registry().with(Log::builder().with_host("127.0.0.1:8362").build()).init(); }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "debugger")]
        tracing::debug!($($arg)*);
    }
}

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    style::{Color, Stylize},
    terminal::size,
};

use std::{
    fs,
    path::{Path, PathBuf},
    sync::RwLock,
};

const PINK: Color = Color::Rgb { r: 225, g: 120, b: 216 };
const BRIGHT_PINK: Color = Color::Rgb { r: 237, g: 171, b: 232 };

fn setup_panic() {
    ::panic::setup_panic! {
        name: "Meow Editor",
        short_name: "meow",
        version: env!("CARGO_PKG_VERSION"),
        repository: "https://github.com/theMackabu/playground/tree/master/editor",
        messages: {
            colors: (Color::Magenta, Color::BrightMagenta, Color::BrightMagenta),
            head: "Well, this is embarrassing. %(name) v%(version) had a problem and crashed. \nTo help us diagnose the problem you can send us a crash report\n",
            body: "We have generated a report file at \"%(file_path)\". \nSubmit an issue or email with the subject of \"%(name) v%(version) crash report\" and include the report as an attachment at %(repository).\n",
            footer: "We take privacy seriously, and do not perform any automated error collection. \nIn order to improve the software, we rely on people to submit reports. Thank you!"
         }
    };
}

pub fn update_and_render_to_buffer(editor: &mut TextEditor<TermLineLayoutSettings>, width: usize, height: usize, filepath: &Path, relative_line_numbers: bool, event: UiEvent) -> TerminalBuffer {
    let lines = LineNumbers::new(editor.get_first_visible_line(), editor.len_lines(), editor.get_current_line() + 1, relative_line_numbers);

    let total_lines = editor.len_lines();
    let first_visible_line = editor.get_first_visible_line();
    let buffer_size = editor.text.len_bytes();

    let file_size = match std::fs::metadata(filepath) {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => 0,
    };

    let status_bar = StatusBar::new()
        .filepath(filepath)
        .save_status(editor.has_changed_since_save())
        .file_size(file_size, buffer_size)
        .file_type(utils::file_type(filepath))
        .line_ending_type(editor.get_line_ending_type().to_string())
        .file_encoding(editor.get_file_encoding())
        .position(editor.get_row_and_column())
        .scroll_percentage(first_visible_line, total_lines, height)
        .build();

    let events = Layout::new(width as u32, height as u32)
        .add_item(&CommandLine::get(), Align::Bottom, Restriction::Shrink)
        .add_item(&status_bar, Align::Bottom, Restriction::Shrink)
        .add_item(&lines, Align::Left, Restriction::Shrink)
        .add_item(editor, Align::Left, Restriction::Grow)
        .interact(&event);

    for event in events.into_iter().rev() {
        match event {
            UiReaction::ScrollBy(amount) => editor.scroll_vertically(amount),
            UiReaction::EmitScroll(x, y) => editor.set_scroll(x, y, 6, 6),
            UiReaction::SetRelativeCursorPos(x, y, select) => editor.set_relative_cursor_pos(x, y, select),
        }
    }

    let lines = LineNumbers::new(editor.get_first_visible_line(), editor.len_lines(), editor.get_current_line() + 1, relative_line_numbers);

    let buffer = Layout::new(width as u32, height as u32)
        .add_item(&CommandLine::get(), Align::Bottom, Restriction::Shrink)
        .add_item(&status_bar, Align::Bottom, Restriction::Shrink)
        .add_item(&lines, Align::Left, Restriction::Shrink)
        .add_item(editor, Align::Left, Restriction::Grow)
        .draw();

    buffer
}

fn parse_keybind(keybind: &str) -> Option<(KeyModifiers, KeyCode)> {
    let parts: Vec<&str> = keybind.split('+').map(str::trim).collect();
    let mut modifiers = KeyModifiers::empty();
    let key_code = parts.last()?;

    for part in &parts[..parts.len() - 1] {
        match part.to_lowercase().as_str() {
            "ctrl" => modifiers.insert(KeyModifiers::CONTROL),
            "alt" => modifiers.insert(KeyModifiers::ALT),
            "shift" => modifiers.insert(KeyModifiers::SHIFT),
            _ => return None,
        }
    }

    let key_code = match key_code.to_lowercase().as_str() {
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "tab" => KeyCode::Tab,
        "backtab" => KeyCode::BackTab,
        "backspace" => KeyCode::Backspace,
        "enter" => KeyCode::Enter,
        "esc" => KeyCode::Esc,
        "insert" => KeyCode::Insert,
        "delete" => KeyCode::Delete,
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        "f3" => KeyCode::F(3),
        "f4" => KeyCode::F(4),
        "f5" => KeyCode::F(5),
        "f6" => KeyCode::F(6),
        "f7" => KeyCode::F(7),
        "f8" => KeyCode::F(8),
        "f9" => KeyCode::F(9),
        "f10" => KeyCode::F(10),
        "f11" => KeyCode::F(11),
        "f12" => KeyCode::F(12),
        s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap().to_ascii_lowercase()),
        _ => return None,
    };

    Some((modifiers, key_code))
}

fn handle_key_event(
    event: KeyEvent, editor: &mut TextEditor<TermLineLayoutSettings>, keybinds: &config::Keybinds, clip: &mut String, system_clip: &mut Option<Clipboard>, file_path: &Path, width: usize,
    height: usize, relative_line_numbers: bool, current_buffer: &mut Vec<Char>,
) -> UiEvent {
    let KeyEvent { code, modifiers, .. } = event;

    debug!("Received key event: {:?} {:?}", code, modifiers);

    for (action, keybind) in keybinds.iter() {
        if let Some((kb_mod, kb_code)) = parse_keybind(&keybind) {
            if code == kb_code && modifiers == kb_mod {
                debug!("Matched keybind for action: {}", action);
                return perform_action(action, editor, clip, system_clip, file_path, width, height, relative_line_numbers, current_buffer);
            }
        }
    }

    match code {
        KeyCode::Char(c) => editor.insert_character_at_cursor(c),
        KeyCode::Enter => editor.insert_newline_at_cursor(),
        KeyCode::Tab => editor.insert_tab_at_cursor(),
        KeyCode::Backspace => editor.remove_character_or_selection_at_cursor(true),
        KeyCode::Delete => editor.remove_character_or_selection_at_cursor(false),
        _ => {}
    }

    UiEvent::Nothing
}

fn perform_action(
    action: &str, editor: &mut TextEditor<TermLineLayoutSettings>, clip: &mut String, system_clip: &mut Option<Clipboard>, file_path: &Path, width: usize, height: usize, relative_line_numbers: bool,
    current_buffer: &mut Vec<Char>,
) -> UiEvent {
    match action {
        "quit" => {
            if editor.has_changed_since_save() {
                match prompt_save(editor, width, height, file_path, relative_line_numbers) {
                    SavePromptResult::Save => {
                        let string = editor.to_string();
                        if std::fs::create_dir_all(file_path.parent().unwrap()).is_ok() && std::fs::write(file_path, string).is_ok() {
                            editor.set_saved();
                        }
                        UiEvent::Quit
                    }
                    SavePromptResult::DontSave => UiEvent::Quit,
                    SavePromptResult::Cancel(next_buffer) => {
                        CommandLine::set("");
                        *current_buffer = next_buffer;
                        let cursor_position = editor.get_cursor_pos();
                        render(width, Some(cursor_position), current_buffer, &[]);
                        UiEvent::Nothing
                    }
                }
            } else {
                UiEvent::Quit
            }
        }
        "save" => {
            let string = editor.to_string();
            if std::fs::create_dir_all(file_path.parent().unwrap()).is_ok() && std::fs::write(file_path, string).is_ok() {
                editor.set_saved();
            }
            UiEvent::Nothing
        }
        "discard_changes" => {
            editor.discard_changes();
            UiEvent::Nothing
        }
        "undo" => {
            editor.undo();
            UiEvent::Nothing
        }
        "redo" => {
            editor.redo();
            UiEvent::Nothing
        }
        "copy" => {
            if let Some(x) = editor.get_selection() {
                *clip = x;
            }
            UiEvent::Nothing
        }
        "paste" => {
            if !clip.is_empty() {
                editor.insert_string_at_cursor(clip);
            }
            UiEvent::Nothing
        }
        "cut" => {
            if let Some(x) = editor.cut_selection() {
                *clip = x;
            }
            UiEvent::Nothing
        }
        "system_copy" => {
            if let Some(x) = editor.get_selection() {
                system_clip.as_mut().map(|y| y.set_text(x));
            }
            UiEvent::Nothing
        }
        "system_paste" => {
            if let Some(x) = system_clip.as_mut() {
                if let Ok(y) = x.get_text() {
                    if !y.is_empty() {
                        editor.insert_string_at_cursor(&y);
                    }
                }
            }
            UiEvent::Nothing
        }
        "system_cut" => {
            if let Some(x) = editor.cut_selection() {
                system_clip.as_mut().map(|y| y.set_text(x));
            }
            UiEvent::Nothing
        }
        "move_up" => {
            editor.move_cursor_vertical(-1, false, false);
            UiEvent::Nothing
        }
        "move_down" => {
            editor.move_cursor_vertical(1, false, false);
            UiEvent::Nothing
        }
        "move_left" => {
            editor.move_cursor_horizontal(-1, false, true);
            UiEvent::Nothing
        }
        "move_right" => {
            editor.move_cursor_horizontal(1, false, true);
            UiEvent::Nothing
        }
        "move_word_left" => {
            editor.move_cursor_horizontal_words(-1, false, true);
            UiEvent::Nothing
        }
        "move_word_right" => {
            editor.move_cursor_horizontal_words(1, false, true);
            UiEvent::Nothing
        }
        "move_to_start_of_line" => {
            editor.move_cursor_to_start_of_line(false, true);
            UiEvent::Nothing
        }
        "move_to_end_of_line" => {
            editor.move_cursor_to_end_of_line(false, true);
            UiEvent::Nothing
        }
        "page_up" => UiEvent::ScrollPage(Scroll::Up(Size::Full)),
        "page_down" => UiEvent::ScrollPage(Scroll::Down(Size::Full)),
        "half_page_up" => UiEvent::ScrollPage(Scroll::Up(Size::Half)),
        "half_page_down" => UiEvent::ScrollPage(Scroll::Down(Size::Half)),
        _ => UiEvent::Nothing,
    }
}

fn terminal_main(file_content: String, newly_loaded: bool, args: Parsed) {
    let Parsed {
        file_path,
        relative_line_numbers,
        tab_width,
        disable_mouse_interaction,
        keybinds,
        ..
    } = args;

    setup_terminal(disable_mouse_interaction);

    let (mut width, mut height) = size().unwrap();
    let mut editor = TextEditor::new(&file_content, TermLineLayoutSettings::new(tab_width), tab_width, newly_loaded, &file_path);
    let mut clip = String::new();
    let mut system_clip = Clipboard::new().ok();

    let (mut current_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &file_path, relative_line_numbers, UiEvent::Nothing);

    render(width as usize, cursor_position, &current_buffer, &[]);

    loop {
        if poll(std::time::Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Mouse(MouseEvent { row, column, kind, .. })
                    if !disable_mouse_interaction && (kind == MouseEventKind::Down(MouseButton::Left) || kind == MouseEventKind::Drag(MouseButton::Left)) =>
                {
                    let (next_buffer, cursor_position) = update_and_render_to_buffer(
                        &mut editor,
                        width as usize,
                        height as usize,
                        &file_path,
                        relative_line_numbers,
                        UiEvent::Clicked(column as usize, row as usize, kind == MouseEventKind::Drag(MouseButton::Left)),
                    );

                    render(width as usize, cursor_position, &next_buffer, &current_buffer);

                    current_buffer = next_buffer;
                }
                Event::Key(key_event) => {
                    let ui_event = handle_key_event(
                        key_event,
                        &mut editor,
                        &keybinds,
                        &mut clip,
                        &mut system_clip,
                        &file_path,
                        width as usize,
                        height as usize,
                        relative_line_numbers,
                        &mut current_buffer,
                    );

                    if ui_event == UiEvent::Quit {
                        break;
                    }

                    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &file_path, relative_line_numbers, ui_event);
                    render(width as usize, cursor_position, &next_buffer, &current_buffer);
                    current_buffer = next_buffer;
                }

                Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollDown, .. }) => {
                    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &file_path, relative_line_numbers, UiEvent::ScrollBy(1));

                    render(width as usize, cursor_position, &next_buffer, &current_buffer);
                    current_buffer = next_buffer;
                }

                Event::Resize(..) => {
                    width = size().unwrap().0;
                    height = size().unwrap().1;

                    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &file_path, relative_line_numbers, UiEvent::Nothing);
                    render(width as usize, cursor_position, &next_buffer, &[]);

                    current_buffer = next_buffer;
                }
                _ => (),
            }
        }
    }

    cleanup_terminal();
}

enum SavePromptResult {
    Save,
    DontSave,
    Cancel(Vec<Char>),
}

fn prompt_save(mut editor: &mut TextEditor<TermLineLayoutSettings>, width: usize, height: usize, save_path: &Path, relative_line_numbers: bool) -> SavePromptResult {
    CommandLine::set(" save changes to file before closing? (y,n,esc)");

    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width, height as usize, &save_path, relative_line_numbers, UiEvent::Nothing);
    render(width as usize, cursor_position, &next_buffer, &[]);

    loop {
        if let Event::Key(KeyEvent { code, .. }) = crossterm::event::read().unwrap() {
            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') => return SavePromptResult::Save,
                KeyCode::Char('n') | KeyCode::Char('N') => return SavePromptResult::DontSave,
                KeyCode::Esc => return SavePromptResult::Cancel(next_buffer),
                _ => {}
            }
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg()]
    /// File to edit
    #[arg(required_unless_present = "list_themes")]
    file_path: Option<PathBuf>,

    /// Whether to allow mouse navigation
    #[arg(long, short)]
    disable_mouse_interaction: Option<bool>,

    /// Tab width
    #[arg(long, short)]
    tab_width: Option<usize>,

    /// Theme name
    #[arg(long, short = 's')]
    theme: Option<String>,

    /// Whether to use relative line numbers
    #[arg(long, short)]
    relative_line_numbers: Option<bool>,

    /// List available themes
    #[arg(long)]
    list_themes: bool,
}

struct Parsed {
    file_path: PathBuf,
    disable_mouse_interaction: bool,
    tab_width: usize,
    theme: Option<String>,
    relative_line_numbers: bool,
    keybinds: config::Keybinds,
}

static THEME: RwLock<Option<Theme>> = RwLock::new(None);
static HIGHLIGHT_COLORS: RwLock<Vec<Color>> = RwLock::new(Vec::new());

static BG_COLOR: RwLock<Color> = RwLock::new(Color::Rgb { r: 33, g: 33, b: 33 });
static FG_COLOR: RwLock<Color> = RwLock::new(Color::Rgb { r: 255, g: 255, b: 255 });

fn main() {
    setup_panic();

    #[cfg(feature = "debugger")]
    debugger::init();

    let config = config::load();
    let args = Args::parse();

    if args.list_themes {
        println!("{}", "Available themes:".with(PINK));
        for theme in constants::list() {
            println!("  {}", theme.with(BRIGHT_PINK));
        }
        return;
    }

    #[cfg(feature = "debugger")]
    {
        if let Some(ref keybinds) = config.keybinds {
            debug!("Loaded keybinds: {:?}", keybinds);
        } else {
            debug!("No custom keybinds loaded, using defaults");
        }
    }

    let args = Parsed {
        file_path: args.file_path.unwrap(),
        theme: args.theme.or(config.theme),
        keybinds: config.keybinds.unwrap_or_default(),
        tab_width: args.tab_width.or(config.tab_width).unwrap_or(4),
        relative_line_numbers: args.relative_line_numbers.or(config.relative_line_numbers).unwrap_or(false),
        disable_mouse_interaction: args.disable_mouse_interaction.or(config.disable_mouse_interaction).unwrap_or(false),
    };

    let (file_content, is_newly_loaded) = match fs::read(&args.file_path.to_owned()) {
        Ok(d) => (String::from_utf8_lossy(&d).into_owned(), false),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (String::new(), true),
        Err(e) => return eprintln!("{}", format!("Failed to open file: {e}").with(Color::Red)),
    };

    if let Some(name) = args.theme.to_owned() {
        if let Some(theme) = constants::from_token(&name) {
            *THEME.write().expect("Able to write to THEME") = match Theme::get_theme(theme) {
                Ok(data) => {
                    let mut bg_hook = BG_COLOR.write().expect("Able to write to BG_COLOR");
                    let mut fg_hook = FG_COLOR.write().expect("Able to write to FG_COLOR");
                    let mut hl_hook = HIGHLIGHT_COLORS.write().expect("Failed to acquire write lock on HIGHLIGHT_COLORS");

                    *bg_hook = Color::Rgb {
                        r: data.bg.r,
                        g: data.bg.g,
                        b: data.bg.b,
                    };

                    *fg_hook = Color::Rgb {
                        r: data.fg.r,
                        g: data.fg.g,
                        b: data.fg.b,
                    };

                    *hl_hook = constants::HIGHLIGHT_NAMES
                        .iter()
                        .map(|&style| {
                            let color = data.get_style(style).and_then(|s| s.fg).unwrap_or(data.fg);
                            Color::Rgb { r: color.r, g: color.g, b: color.b }
                        })
                        .collect();

                    Some(data)
                }
                Err(_) => None,
            };
        }
    }

    terminal_main(file_content, is_newly_loaded, args)
}

mod clipboard;
mod editor;
mod terminal;
mod ui;
mod unicode;
mod utils;
mod widgets;
mod widgets_impl;

use clap::Parser;
use clipboard::Clipboard;
use editor::*;
use terminal::*;
use ui::*;
use widgets::*;
use widgets_impl::*;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    terminal::size,
};

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub fn update_and_render_to_buffer(editor: &mut TextEditor<TermLineLayoutSettings>, width: usize, height: usize, filepath: &Path, relative_line_numbers: bool, event: UiEvent) -> TerminalBuffer {
    let (pos_x, pos_y) = {
        let (x, y) = editor.get_row_and_column();
        (x + 1, y + 1)
    };

    let total_lines = editor.len_lines();
    let first_visible_line = editor.get_first_visible_line();
    let last_visible_line = first_visible_line + height - 1;

    let percent_scrolled = if first_visible_line == 0 {
        "top".to_string()
    } else if last_visible_line >= total_lines - 1 {
        "end".to_string()
    } else {
        format!("{}%", ((first_visible_line as f64 / total_lines as f64) * 100.0).round())
    };

    let buffer_size = editor.text.len_bytes();
    let file_size = match std::fs::metadata(filepath) {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => 0,
    };

    let file_size_str = if file_size < 1024 {
        format!("{}b", file_size)
    } else if file_size < 1024 * 1024 {
        format!("{:.1}kb", file_size as f64 / 1024.0)
    } else {
        format!("{:.1}mb", file_size as f64 / (1024.0 * 1024.0))
    };

    let size_info = if buffer_size == file_size {
        format!("({file_size_str})")
    } else {
        let diff = buffer_size - file_size;
        let diff_str = if diff < 1024 {
            format!("+{}b", diff)
        } else if diff < 1024 * 1024 {
            format!("+{:.1}kb", diff as f64 / 1024.0)
        } else {
            format!("+{:.1}mb", diff as f64 / (1024.0 * 1024.0))
        };
        format!("({}{})", file_size_str, diff_str)
    };

    let lines = LineNumbers::new(editor.get_first_visible_line(), editor.len_lines(), editor.get_current_line() + 1, relative_line_numbers);

    let template = format!(
        " {}{} {size_info} | ft:{} | {} | {}",
        filepath.file_name().unwrap().to_string_lossy(),
        if editor.has_changed_since_save() { "*" } else { "" },
        utils::file_type(filepath),
        editor.get_line_ending_type(),
        editor.get_file_encoding(),
    );

    let position_status = format!("{pos_x}:{pos_y} | {percent_scrolled} ");
    let padding_width = width.saturating_sub(template.len() + position_status.len());
    let padding = " ".repeat(padding_width);
    let status_bar_text = format!("{}{}{}", template, padding, position_status);
    let status_bar = TextLine::new(&status_bar_text);

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

fn terminal_main(file_content: String, newly_loaded: bool, save_path: PathBuf, relative_line_numbers: bool, tab_width: usize, disable_mouse_interaction: bool) {
    setup_terminal(disable_mouse_interaction);

    let (mut width, mut height) = size().unwrap();
    let mut editor = TextEditor::new(&file_content, TermLineLayoutSettings::new(tab_width), tab_width, newly_loaded, &save_path);
    let mut clip = String::new();
    let mut system_clip = Clipboard::new().ok();

    let (mut current_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &save_path, relative_line_numbers, UiEvent::Nothing);

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
                        &save_path,
                        relative_line_numbers,
                        UiEvent::Clicked(column as usize, row as usize, kind == MouseEventKind::Drag(MouseButton::Left)),
                    );

                    render(width as usize, cursor_position, &next_buffer, &current_buffer);

                    current_buffer = next_buffer;
                }
                Event::Key(KeyEvent { code, modifiers, .. }) => {
                    if code == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
                        if editor.has_changed_since_save() {
                            match prompt_save(&mut editor, width as usize, height as usize, &save_path, relative_line_numbers) {
                                SavePromptResult::Save => {
                                    break;
                                }
                                SavePromptResult::DontSave => break,
                                SavePromptResult::Cancel(next_buffer) => {
                                    CommandLine::set("");
                                    render(width as usize, cursor_position, &next_buffer, &current_buffer);
                                    current_buffer = next_buffer;
                                }
                            }
                        } else {
                            break;
                        }
                    }

                    let mut ui_event = UiEvent::Nothing;

                    if code == KeyCode::Char('s') && modifiers == KeyModifiers::CONTROL {
                        let string = editor.to_string();

                        if std::fs::create_dir_all(save_path.as_path().parent().unwrap()).is_ok() && std::fs::write(save_path.as_path(), string).is_ok() {
                            editor.set_saved();
                        }
                    } else if code == KeyCode::Char('a') && modifiers == KeyModifiers::CONTROL {
                        editor.discard_changes();
                    } else if code == KeyCode::Char('z') && modifiers == KeyModifiers::CONTROL {
                        editor.undo();
                    } else if code == KeyCode::Char('y') && modifiers == KeyModifiers::CONTROL {
                        editor.redo();
                    } else if code == KeyCode::Char('c') && modifiers == KeyModifiers::CONTROL {
                        if let Some(x) = editor.get_selection() {
                            clip = x;
                        }
                    } else if code == KeyCode::Char('v') && modifiers == KeyModifiers::CONTROL {
                        if !clip.is_empty() {
                            editor.insert_string_at_cursor(&clip);
                        }
                    } else if code == KeyCode::Char('x') && modifiers == KeyModifiers::CONTROL {
                        if let Some(x) = editor.cut_selection() {
                            clip = x;
                        }
                    } else if code == KeyCode::Char('c') && modifiers == KeyModifiers::ALT {
                        if let Some(x) = editor.get_selection() {
                            system_clip.as_mut().map(|y| y.set_text(x));
                        }
                    } else if code == KeyCode::Char('v') && modifiers == KeyModifiers::ALT {
                        if let Some(x) = system_clip.as_mut() {
                            if let Ok(y) = x.get_text() {
                                if !y.is_empty() {
                                    editor.insert_string_at_cursor(&y);
                                }
                            }
                        }
                    } else if code == KeyCode::Char('x') && modifiers == KeyModifiers::ALT {
                        if let Some(x) = editor.cut_selection() {
                            system_clip.as_mut().map(|y| y.set_text(x));
                        }
                    } else if code == KeyCode::Up {
                        editor.move_cursor_vertical(-1, modifiers == KeyModifiers::SHIFT, false);
                    } else if code == KeyCode::Down {
                        editor.move_cursor_vertical(1, modifiers == KeyModifiers::SHIFT, false);
                    } else if code == KeyCode::Left && modifiers.contains(KeyModifiers::CONTROL) {
                        editor.move_cursor_horizontal_words(-1, modifiers.contains(KeyModifiers::SHIFT), true);
                    } else if code == KeyCode::Right && modifiers.contains(KeyModifiers::CONTROL) {
                        editor.move_cursor_horizontal_words(1, modifiers.contains(KeyModifiers::SHIFT), true);
                    } else if code == KeyCode::Left {
                        editor.move_cursor_horizontal(-1, modifiers == KeyModifiers::SHIFT, true);
                    } else if code == KeyCode::Right {
                        editor.move_cursor_horizontal(1, modifiers == KeyModifiers::SHIFT, true);
                    } else if code == KeyCode::Home {
                        editor.move_cursor_to_start_of_line(modifiers == KeyModifiers::SHIFT, true);
                    } else if code == KeyCode::End {
                        editor.move_cursor_to_end_of_line(modifiers == KeyModifiers::SHIFT, true);
                    } else if code == KeyCode::Char('b') && modifiers == KeyModifiers::CONTROL {
                        ui_event = UiEvent::ScrollPage(Scroll::Up(Size::Full));
                    } else if code == KeyCode::Char('f') && modifiers == KeyModifiers::CONTROL {
                        ui_event = UiEvent::ScrollPage(Scroll::Down(Size::Full));
                    } else if code == KeyCode::Char('u') && modifiers == KeyModifiers::CONTROL {
                        ui_event = UiEvent::ScrollPage(Scroll::Up(Size::Half));
                    } else if code == KeyCode::Char('d') && modifiers == KeyModifiers::CONTROL {
                        ui_event = UiEvent::ScrollPage(Scroll::Down(Size::Half));
                    } else if code == KeyCode::PageUp {
                        ui_event = UiEvent::ScrollPage(Scroll::Up(Size::Full));
                    } else if code == KeyCode::PageDown {
                        ui_event = UiEvent::ScrollPage(Scroll::Down(Size::Full));
                    } else if let KeyCode::Char(c) = code {
                        editor.insert_character_at_cursor(c);
                    } else if code == KeyCode::Enter {
                        editor.insert_newline_at_cursor();
                    } else if code == KeyCode::Tab {
                        editor.insert_tab_at_cursor();
                    } else if code == KeyCode::Backspace {
                        editor.remove_character_or_selection_at_cursor(true);
                    } else if code == KeyCode::Delete {
                        editor.remove_character_or_selection_at_cursor(false);
                    }

                    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &save_path, relative_line_numbers, ui_event);
                    render(width as usize, cursor_position, &next_buffer, &current_buffer);

                    current_buffer = next_buffer;
                }

                Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollUp, .. }) => {
                    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &save_path, relative_line_numbers, UiEvent::ScrollBy(-1));

                    render(width as usize, cursor_position, &next_buffer, &current_buffer);
                    current_buffer = next_buffer;
                }
                Event::Mouse(MouseEvent { kind: MouseEventKind::ScrollDown, .. }) => {
                    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &save_path, relative_line_numbers, UiEvent::ScrollBy(1));

                    render(width as usize, cursor_position, &next_buffer, &current_buffer);
                    current_buffer = next_buffer;
                }

                Event::Resize(..) => {
                    width = size().unwrap().0;
                    height = size().unwrap().1;

                    let (next_buffer, cursor_position) = update_and_render_to_buffer(&mut editor, width as usize, height as usize, &save_path, relative_line_numbers, UiEvent::Nothing);
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
    file_path: PathBuf,

    /// Whether to allow mouse navigation
    #[arg(long, short, default_value_t = false)]
    disable_mouse_interaction: bool,

    /// Tab width
    #[arg(long, short, default_value_t = 4)]
    tab_width: usize,

    /// Whether to use relative line numbers
    #[arg(long, short)]
    relative_line_numbers: bool,
}

fn main() {
    let args = Args::parse();

    let (file_content, newly_loaded) = match read_to_string(&args.file_path) {
        Ok(x) => (x, false),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (String::new(), true),
        Err(e) => {
            println!("Failed to open file: {:?}", e);
            return;
        }
    };

    terminal_main(file_content, newly_loaded, args.file_path, args.relative_line_numbers, args.tab_width, args.disable_mouse_interaction);
}

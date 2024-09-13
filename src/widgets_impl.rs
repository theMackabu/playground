use crate::editor::*;
use crate::terminal::*;
use crate::ui::*;
use crate::unicode::*;
use crate::widgets::*;
use crossterm::style::Color;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Size {
    Half,
    Full,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Scroll {
    Up(Size),
    Down(Size),
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum UiEvent {
    Clicked(usize, usize, bool),
    ScrollPage(Scroll),
    ScrollBy(isize),
    Nothing,
}

#[derive(Copy, Clone)]
pub enum UiReaction {
    EmitScroll(usize, usize),
    SetRelativeCursorPos(usize, usize, bool),
    ScrollBy(isize),
}

impl<'a> Drawable<TerminalBuffer> for CommandLine<'a> {
    fn draw(&self, width: u32, height: u32) -> TerminalBuffer {
        (
            self.text
                .chars()
                .chain(std::iter::repeat(' '))
                .scan(0, |acc, x| {
                    *acc += string_width(std::iter::once(x), TERM_TAB_WIDTH);
                    if *acc <= width as usize * height as usize {
                        Some(x)
                    } else {
                        None
                    }
                })
                .map(|c| Char::new(c, None, Highlight::None))
                .collect(),
            None,
        )
    }
}

impl<'a> Interactive<UiEvent, Vec<UiReaction>> for CommandLine<'a> {
    fn interact(&self, _: &UiEvent, _: u32, _: u32, _: u32, _: u32) -> Vec<UiReaction> { Vec::new() }
}

impl<'a> Widget<TerminalBuffer, UiEvent, Vec<UiReaction>> for CommandLine<'a> {
    fn minimum_size(&self, _: u32, _: u32) -> (u32, u32) { (string_width(self.text.chars(), TERM_TAB_WIDTH) as u32, 1) }
    fn maximum_size(&self, width: u32, height: u32) -> (u32, u32) { (width, height) }
}

impl<'a> Drawable<TerminalBuffer> for TextLine<'a> {
    fn draw(&self, width: u32, height: u32) -> TerminalBuffer {
        (
            self.string
                .chars()
                .chain(std::iter::repeat(' '))
                .scan(0, |acc, x| {
                    *acc += string_width(std::iter::once(x), TERM_TAB_WIDTH);
                    if *acc <= width as usize * height as usize {
                        Some(x)
                    } else {
                        None
                    }
                })
                .map(|c| Char::new(c, None, Highlight::Status))
                .collect(),
            None,
        )
    }
}

impl<'a> Interactive<UiEvent, Vec<UiReaction>> for TextLine<'a> {
    fn interact(&self, _: &UiEvent, _: u32, _: u32, _: u32, _: u32) -> Vec<UiReaction> { Vec::new() }
}

impl<'a> Widget<TerminalBuffer, UiEvent, Vec<UiReaction>> for TextLine<'a> {
    fn minimum_size(&self, _: u32, _: u32) -> (u32, u32) { (string_width(self.string.chars(), TERM_TAB_WIDTH) as u32, 1) }
    fn maximum_size(&self, width: u32, height: u32) -> (u32, u32) { (width, height) }
}

impl Drawable<TerminalBuffer> for LineNumbers {
    fn draw(&self, width: u32, height: u32) -> TerminalBuffer {
        let mut buffer = Vec::with_capacity(width as usize * height as usize);

        let padding = self.width_number(height as usize);
        let space_padding = (width as usize).saturating_sub(padding + 1).max(1);

        let start = self.start + 1;
        let end = (self.start + 1 + height as usize).min(self.total + 1);

        for line in start..end {
            let mut column = 0;

            while column < (width as usize).min(space_padding) {
                buffer.push(Char::new(' ', None, Highlight::Gutter));
                column += 1;
            }

            let mut base = 10_usize.pow(padding.saturating_sub(1) as u32);
            let line_number = if self.relative && line != self.current { line.abs_diff(self.current) } else { line };

            while base > 0 && column < width as usize {
                if line_number / base > 0 {
                    let digit = (line_number / base) % 10;
                    let character = char::from_digit(digit as u32, 10).unwrap();

                    buffer.push(Char::new(character, None, Highlight::Gutter));
                } else {
                    buffer.push(Char::new(' ', None, Highlight::Gutter));
                }

                base /= 10;
                column += 1;
            }

            if column < width as usize {
                buffer.push(Char::new(' ', None, Highlight::Gutter));
            }
        }

        buffer.extend(
            std::iter::repeat((0..width as usize).map(|_| Char::new(' ', None, Highlight::Gutter)))
                .take((height as usize).saturating_sub(end - start))
                .flatten(),
        );

        (buffer, None)
    }
}

impl Interactive<UiEvent, Vec<UiReaction>> for LineNumbers {
    fn interact(&self, _: &UiEvent, _: u32, _: u32, _: u32, _: u32) -> Vec<UiReaction> { Vec::new() }
}

impl Widget<TerminalBuffer, UiEvent, Vec<UiReaction>> for LineNumbers {
    fn minimum_size(&self, _: u32, height: u32) -> (u32, u32) { (self.width(height as usize) as u32, height) }
    fn maximum_size(&self, _: u32, height: u32) -> (u32, u32) { (self.width(height as usize) as u32, height) }
}

impl Drawable<TerminalBuffer> for TextEditor<TermLineLayoutSettings> {
    fn draw(&self, width: u32, height: u32) -> TerminalBuffer {
        let mut buffer = Vec::with_capacity(width as usize * height as usize);

        let selection_range = self.get_selection_range().unwrap_or(0..0);
        let highlight_map = self.highlight_map.lock().unwrap();

        for line_num in self.get_lines_scrolled()..self.get_lines_scrolled() + height as usize {
            let mut column = 0;
            let mut cursor = 0;

            if let Some(line) = self.get_text().get_line(line_num) {
                let line_start = self.get_text().line_to_byte(line_num);

                while cursor < line.len_bytes() && column < self.get_columns_scrolled() + width as usize {
                    let next_cursor = move_grapheme(1, cursor, line);
                    let grapheme = line.byte_slice(cursor..next_cursor);

                    if grapheme.chars().any(is_newline) {
                        if column >= self.get_columns_scrolled() && column < self.get_columns_scrolled() + width as usize && selection_range.contains(&(cursor + line_start)) {
                            buffer.push(Char::new(' ', Some((Color::White, None)), Highlight::Selection));
                            column += 1;
                        }

                        break;
                    }

                    let grapheme_width = string_width(grapheme.chars(), self.get_tab_width());

                    if column < self.get_columns_scrolled() && column + grapheme_width > self.get_columns_scrolled() {
                        buffer.extend(std::iter::repeat(Char::new_text(' ', Some((Color::White, None)), false)).take(column + grapheme_width - self.get_columns_scrolled()));
                    } else if column + grapheme_width > self.get_columns_scrolled() + width as usize {
                        buffer.extend(std::iter::repeat(Char::new_text(' ', Some((Color::White, None)), false)).take(self.get_columns_scrolled() + width as usize - column));
                    } else if column >= self.get_columns_scrolled() && column + grapheme_width <= self.get_columns_scrolled() + width as usize && grapheme.chars().eq(std::iter::once('\t')) {
                        buffer.extend(
                            std::iter::repeat(' ')
                                .take(self.get_tab_width())
                                .map(|x| Char::new_text(x, Some((Color::White, None)), selection_range.contains(&(cursor + line_start)))),
                        );
                    } else if column >= self.get_columns_scrolled() && column + grapheme_width <= self.get_columns_scrolled() + width as usize {
                        let color = highlight_map.get(&(cursor + line_start)).copied().unwrap_or((Color::Reset, None));

                        if grapheme.chars().eq(std::iter::once('\t')) {
                            buffer.extend(
                                std::iter::repeat(' ')
                                    .take(self.get_tab_width())
                                    .map(|x| Char::new_text(x, Some(color), selection_range.contains(&(cursor + line_start)))),
                            );
                        } else {
                            buffer.extend(grapheme.chars().map(|x| Char::new_text(x, Some(color), selection_range.contains(&(cursor + line_start)))));
                        }
                    }

                    cursor = next_cursor;
                    column += grapheme_width;
                }
            }

            buffer.extend(
                std::iter::repeat(Char::new_text(' ', Some((Color::White, None)), false)).take((width as usize + self.get_columns_scrolled()).saturating_sub(column.max(self.get_columns_scrolled()))),
            );
        }

        let cursor_pos = self.get_relative_cursor_pos();
        (buffer, cursor_pos)
    }
}

impl Interactive<UiEvent, Vec<UiReaction>> for TextEditor<TermLineLayoutSettings> {
    fn interact(&self, event: &UiEvent, x: u32, y: u32, width: u32, height: u32) -> Vec<UiReaction> {
        let extra = match event {
            UiEvent::Clicked(cx, cy, select) => {
                let (click_x, click_y) = (*cx as isize - x as isize, *cy as isize - y as isize);

                if click_x >= 0 && click_x < width as isize && click_y >= 0 && click_y < height as isize {
                    Some(UiReaction::SetRelativeCursorPos(click_x as usize, click_y as usize, *select))
                } else {
                    None
                }
            }

            UiEvent::ScrollPage(change) => match change {
                Scroll::Up(size) => match size {
                    Size::Full => Some(UiReaction::ScrollBy(-(height as isize))),
                    Size::Half => Some(UiReaction::ScrollBy(-(height.checked_div(2).unwrap_or(0) as isize))),
                },
                Scroll::Down(size) => match size {
                    Size::Full => Some(UiReaction::ScrollBy(height as isize)),
                    Size::Half => Some(UiReaction::ScrollBy(height.checked_div(2).unwrap_or(0) as isize)),
                },
            },

            UiEvent::ScrollBy(amount) => Some(UiReaction::ScrollBy(*amount)),

            _ => None,
        };

        [UiReaction::EmitScroll(width as usize, height as usize)].into_iter().chain(extra).collect()
    }
}

impl Widget<TerminalBuffer, UiEvent, Vec<UiReaction>> for TextEditor<TermLineLayoutSettings> {
    fn minimum_size(&self, width: u32, height: u32) -> (u32, u32) { (width, height) }
    fn maximum_size(&self, width: u32, height: u32) -> (u32, u32) { (width, height) }
}

impl OutputResult for Vec<UiReaction> {
    fn empty() -> Self { Vec::new() }
    fn combine(self, other: Self) -> Self { self.into_iter().chain(other).collect() }
}

impl DrawResult for TerminalBuffer {
    fn empty(width: u32, height: u32) -> Self { (std::iter::repeat(Char::new_text(' ', None, false)).take(width as usize * height as usize).collect(), None) }

    fn combine_vertical(self, other: Self, _: u32, split: u32, _: u32) -> Self { (self.0.iter().chain(other.0.iter()).copied().collect(), self.1.or(other.1.map(|(x, y)| (x, y + split as usize)))) }

    fn combine_horizontal(self, other: Self, width: u32, split: u32, _: u32) -> Self {
        let mut left_chars = self.0.iter();
        let mut right_chars = other.0.iter();

        let mut column = 0;
        let mut buffer = Vec::with_capacity(self.0.len() + other.0.len());

        while let Some(character) = if column < split as usize { left_chars.next() } else { right_chars.next() } {
            column += string_width(std::iter::once(character.c), TERM_TAB_WIDTH);

            if column >= width as usize {
                column -= width as usize;
            }

            buffer.push(*character);
        }

        (buffer, self.1.or(other.1.map(|(x, y)| (x + split as usize, y))))
    }
}

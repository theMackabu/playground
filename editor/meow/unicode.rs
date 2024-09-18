use ropey::RopeSlice;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};
use unicode_width::UnicodeWidthChar;

pub const TERM_TAB_WIDTH: usize = 0;

pub fn move_grapheme(amount: isize, mut byte_cursor: usize, text: RopeSlice) -> usize {
    let mut cursor = GraphemeCursor::new(byte_cursor, text.len_bytes(), true);
    let (mut chunk, mut chunk_idx, _, _) = text.chunk_at_byte(byte_cursor);

    for _ in 0..amount.abs() {
        loop {
            match if amount > 0 {
                cursor.next_boundary(chunk, chunk_idx)
            } else {
                cursor.prev_boundary(chunk, chunk_idx)
            } {
                Ok(None) => {
                    return if amount > 0 { text.len_bytes() } else { 0 };
                }

                Ok(Some(n)) => {
                    byte_cursor = n;
                    break;
                }

                Err(GraphemeIncomplete::NextChunk) => {
                    let (next_chunk, next_chunk_idx, _, _) = text.chunk_at_byte(chunk_idx + chunk.len());
                    chunk = next_chunk;
                    chunk_idx = next_chunk_idx;
                }

                Err(GraphemeIncomplete::PrevChunk) => {
                    let (prev_chunk, prev_chunk_idx, _, _) = text.chunk_at_byte(chunk_idx - 1);
                    chunk = prev_chunk;
                    chunk_idx = prev_chunk_idx;
                }

                Err(GraphemeIncomplete::PreContext(n)) => {
                    let ctx = text.chunk_at_byte(n - 1).0;
                    cursor.provide_context(ctx, n - ctx.len());
                }

                _ => unreachable!(),
            }
        }
    }

    byte_cursor
}

pub fn string_width<I: IntoIterator<Item = char>>(iterator: I, tab_width: usize) -> usize { iterator.into_iter().map(|x| if x == '\t' { tab_width } else { x.width_cjk().unwrap_or(0) }).sum() }

pub fn is_newline(c: char) -> bool { ['\n', '\r', '\u{000B}', '\u{000C}', '\u{0085}', '\u{2028}', '\u{2029}'].contains(&c) }

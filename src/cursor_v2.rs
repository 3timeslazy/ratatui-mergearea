
use crate::widget::Viewport;
use crate::word::find_word_inclusive_end_forward;
#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp;

/// Specify how to move the cursor.
///
/// This type is marked as `#[non_exhaustive]` since more variations may be supported in the future.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CursorMove {
    /// Move cursor forward by one character. When the cursor is at the end of line, it moves to the head of next line.
    /// ```
    /// use tui_textarea::{TextArea, CursorMove};
    ///
    /// let mut textarea = TextArea::from(["abc"]);
    ///
    /// textarea.move_cursor(CursorMove::Forward);
    /// assert_eq!(textarea.cursor(), (0, 1));
    /// textarea.move_cursor(CursorMove::Forward);
    /// assert_eq!(textarea.cursor(), (0, 2));
    /// ```
    Forward,
    /// Move cursor backward by one character. When the cursor is at the head of line, it moves to the end of previous
    /// line.
    /// ```
    /// use tui_textarea::{TextArea, CursorMove};
    ///
    /// let mut textarea = TextArea::from(["abc"]);
    ///
    /// textarea.move_cursor(CursorMove::Forward);
    /// textarea.move_cursor(CursorMove::Forward);
    /// textarea.move_cursor(CursorMove::Back);
    /// assert_eq!(textarea.cursor(), (0, 1));
    /// ```
    Back,
    /// Move cursor up by one line.
    /// ```
    /// use tui_textarea::{TextArea, CursorMove};
    ///
    /// let mut textarea = TextArea::from(["a", "b", "c"]);
    ///
    /// textarea.move_cursor(CursorMove::Down);
    /// textarea.move_cursor(CursorMove::Down);
    /// textarea.move_cursor(CursorMove::Up);
    /// assert_eq!(textarea.cursor(), (1, 0));
    /// ```
    Up,
    /// Move cursor down by one line.
    /// ```
    /// use tui_textarea::{TextArea, CursorMove};
    ///
    /// let mut textarea = TextArea::from(["a", "b", "c"]);
    ///
    /// textarea.move_cursor(CursorMove::Down);
    /// assert_eq!(textarea.cursor(), (1, 0));
    /// textarea.move_cursor(CursorMove::Down);
    /// assert_eq!(textarea.cursor(), (2, 0));
    /// ```
    Down,
}

impl CursorMove {
    pub(crate) fn next_cursor(
        &self,
        offset: usize,
        text: &autosurgeon::Text,
        viewport: &Viewport,
    ) -> Option<usize> {
        use CursorMove::*;

        fn clen(text: &autosurgeon::Text) -> usize {
            text.as_str().chars().count()
        }

        fn find_line_start(offset: usize, chars: &[char]) -> usize {
            let mut current_line_start = offset;
            while current_line_start > 0 && chars[current_line_start - 1] != '\n' {
                current_line_start -= 1;
            }
            current_line_start
        }

        match self {
            Forward => {
                if offset >= clen(text) {
                    None
                } else {
                    Some(offset + 1)
                }
            }
            Back => offset.checked_sub(1),
            Up => {
                let chars = text.as_str().chars().collect::<Vec<_>>();

                let line_start = find_line_start(offset, &chars);

                if line_start == 0 {
                    return None;
                }

                let mut prev_line_start = line_start - 1;
                while prev_line_start > 0 && chars[prev_line_start - 1] != '\n' {
                    prev_line_start -= 1;
                }

                let prev_line_end = line_start - 1;
                let prev_line_length = prev_line_end - prev_line_start;

                let current_column = offset - line_start;
                let column = cmp::min(current_column, prev_line_length);

                Some(prev_line_start + column)
            }
            Down => {
                let chars = text.as_str().chars().collect::<Vec<_>>();

                let next_line_start = match chars[offset..].iter().position(|c| *c == '\n') {
                    None => return None,
                    Some(pos) => offset + pos + 1,
                };

                let mut next_line_end = next_line_start;
                while next_line_end < chars.len() && chars[next_line_end] != '\n' {
                    next_line_end += 1;
                }

                let current_column = {
                    let line_start = find_line_start(offset, &chars);
                    offset - line_start
                };
                let next_line_length = next_line_end - next_line_start;
                let new_column = cmp::min(current_column, next_line_length);

                Some(next_line_start + new_column)
            }
        }
    }
}

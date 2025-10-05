use crate::widget::Viewport;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Specify how to scroll the textarea.
///
/// This type is marked as `#[non_exhaustive]` since more variations may be supported in the future. Note that the cursor will
/// not move until it goes out the viewport. See also: [`TextArea::scroll`]
///
/// [`TextArea::scroll`]: https://docs.rs/tui-textarea/latest/tui_textarea/struct.TextArea.html#method.scroll
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Scrolling {
    /// Scroll the textarea by rows (vertically) and columns (horizontally). Passing positive scroll amounts to `rows` and `cols`
    /// scolls it to down and right. Negative integers means the opposite directions. `(i16, i16)` pair can be converted into
    /// `Scrolling::Delta` where 1st element means rows and 2nd means columns.
    ///
    /// ```
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// # use ratatui::widgets::Widget as _;
    /// use ratatui_mergearea::{TextArea, Scrolling};
    ///
    /// // Let's say terminal height is 8.
    ///
    /// // Create textarea with 20 lines "0", "1", "2", "3", ...
    /// let text = (0..20).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
    /// let mut textarea = TextArea::with_value(text);
    /// # // Call `render` at least once to populate terminal size
    /// # let r = Rect { x: 0, y: 0, width: 24, height: 8 };
    /// # let mut b = Buffer::empty(r.clone());
    /// # textarea.render(r, &mut b);
    ///
    /// // Scroll down by 2 lines.
    /// textarea.scroll(Scrolling::Delta{rows: 2, cols: 0});
    /// assert_eq!(textarea.cursor2(), (2, 0));
    ///
    /// // (1, 0) is converted into Scrolling::Delta{rows: 1, cols: 0}
    /// textarea.scroll((1, 0));
    /// assert_eq!(textarea.cursor2(), (3, 0));
    /// ```
    Delta { rows: i16, cols: i16 },
    /// Scroll down the textarea by one page.
    ///
    /// ```
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// # use ratatui::widgets::Widget as _;
    /// use ratatui_mergearea::{TextArea, Scrolling};
    ///
    /// // Let's say terminal height is 8.
    ///
    /// // Create textarea with 20 lines "0", "1", "2", "3", ...
    /// let text = (0..20).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
    /// let mut textarea = TextArea::with_value(text);
    /// # // Call `render` at least once to populate terminal size
    /// # let r = Rect { x: 0, y: 0, width: 24, height: 8 };
    /// # let mut b = Buffer::empty(r.clone());
    /// # textarea.render(r, &mut b);
    ///
    /// // Scroll down by one page (8 lines)
    /// textarea.scroll(Scrolling::PageDown);
    /// assert_eq!(textarea.cursor2(), (8, 0));
    /// textarea.scroll(Scrolling::PageDown);
    /// assert_eq!(textarea.cursor2(), (16, 0));
    /// textarea.scroll(Scrolling::PageDown);
    /// assert_eq!(textarea.cursor2(), (19, 0)); // Reached bottom of the textarea
    /// ```
    PageDown,
    /// Scroll up the textarea by one page.
    ///
    /// ```
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// # use ratatui::widgets::Widget as _;
    /// use ratatui_mergearea::{TextArea, Scrolling, CursorMove};
    ///
    /// // Let's say terminal height is 8.
    ///
    /// // Create textarea with 20 lines "0", "1", "2", "3", ...
    /// let text = (0..20).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
    /// let mut textarea = TextArea::with_value(text);
    /// # // Call `render` at least once to populate terminal size
    /// # let r = Rect { x: 0, y: 0, width: 24, height: 8 };
    /// # let mut b = Buffer::empty(r.clone());
    /// # textarea.render(r.clone(), &mut b);
    ///
    /// // Go to the last line at first
    /// textarea.move_cursor(CursorMove::Bottom);
    /// assert_eq!(textarea.cursor2(), (19, 0));
    /// # // Call `render` to populate terminal size
    /// # textarea.render(r.clone(), &mut b);
    ///
    /// // Scroll up by one page (8 lines)
    /// textarea.scroll(Scrolling::PageUp);
    /// assert_eq!(textarea.cursor2(), (11, 0));
    /// textarea.scroll(Scrolling::PageUp);
    /// assert_eq!(textarea.cursor2(), (7, 0)); // Reached top of the textarea
    /// ```
    PageUp,
    /// Scroll down the textarea by half of the page.
    ///
    /// ```
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// # use ratatui::widgets::Widget as _;
    /// use ratatui_mergearea::{TextArea, Scrolling};
    ///
    /// // Let's say terminal height is 8.
    ///
    /// // Create textarea with 10 lines "0", "1", "2", "3", ...
    /// let text = (0..20).map(|i| i.to_string()).collect::<Vec<_>>().join("\n") + "\n";
    /// let mut textarea = TextArea::with_value(text);
    /// # // Call `render` at least once to populate terminal size
    /// # let r = Rect { x: 0, y: 0, width: 24, height: 8 };
    /// # let mut b = Buffer::empty(r.clone());
    /// # textarea.render(r, &mut b);
    ///
    /// // Scroll down by half-page (4 lines)
    /// textarea.scroll(Scrolling::HalfPageDown);
    /// assert_eq!(textarea.cursor2(), (4, 0));
    /// textarea.scroll(Scrolling::HalfPageDown);
    /// assert_eq!(textarea.cursor2(), (8, 0));
    /// textarea.scroll(Scrolling::HalfPageDown);
    /// assert_eq!(textarea.cursor2(), (9, 0)); // Reached bottom of the textarea
    /// ```
    HalfPageDown,
    /// Scroll up the textarea by half of the page.
    ///
    /// ```
    /// # use ratatui::buffer::Buffer;
    /// # use ratatui::layout::Rect;
    /// # use ratatui::widgets::Widget as _;
    /// use ratatui_mergearea::{TextArea, Scrolling, CursorMove};
    ///
    /// // Let's say terminal height is 8.
    ///
    /// // Create textarea with 20 lines "0", "1", "2", "3", ...
    /// let text = (0..20).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
    /// let mut textarea = TextArea::with_value(text);
    /// # // Call `render` at least once to populate terminal size
    /// # let r = Rect { x: 0, y: 0, width: 24, height: 8 };
    /// # let mut b = Buffer::empty(r.clone());
    /// # textarea.render(r.clone(), &mut b);
    ///
    /// // Go to the last line at first
    /// textarea.move_cursor(CursorMove::Bottom);
    /// assert_eq!(textarea.cursor2(), (19, 0));
    /// # // Call `render` to populate terminal size
    /// # textarea.render(r.clone(), &mut b);
    ///
    /// // Scroll up by half-page (4 lines)
    /// textarea.scroll(Scrolling::HalfPageUp);
    /// assert_eq!(textarea.cursor2(), (15, 0));
    /// textarea.scroll(Scrolling::HalfPageUp);
    /// assert_eq!(textarea.cursor2(), (11, 0));
    /// ```
    HalfPageUp,
}

impl Scrolling {
    pub(crate) fn scroll(self, viewport: &mut Viewport) {
        let (rows, cols) = match self {
            Self::Delta { rows, cols } => (rows, cols),
            Self::PageDown => {
                let (_, _, _, height) = viewport.rect();
                (height as i16, 0)
            }
            Self::PageUp => {
                let (_, _, _, height) = viewport.rect();
                (-(height as i16), 0)
            }
            Self::HalfPageDown => {
                let (_, _, _, height) = viewport.rect();
                ((height as i16) / 2, 0)
            }
            Self::HalfPageUp => {
                let (_, _, _, height) = viewport.rect();
                (-(height as i16) / 2, 0)
            }
        };
        viewport.scroll(rows, cols);
    }
}

impl From<(i16, i16)> for Scrolling {
    fn from((rows, cols): (i16, i16)) -> Self {
        Self::Delta { rows, cols }
    }
}

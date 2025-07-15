use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum EditKind {
    InsertChar(char),
    DeleteChar(char),
    InsertNewline,
    DeleteNewline,
    InsertStr(String),
    DeleteStr(String),
}

impl EditKind {
    pub(crate) fn apply(&self, text: &mut autosurgeon::Text, offset: usize) {
        match self {
            EditKind::InsertChar(c) => {
                text.splice(offset, 0, c.to_string());
            }
            EditKind::DeleteChar(c) => {
                text.splice(offset, c.len_utf8() as isize, "");
            }
            EditKind::InsertNewline => {
                Self::InsertChar('\n').apply(text, offset);
            }
            EditKind::DeleteNewline => {
                Self::DeleteChar('\n').apply(text, offset);
            }
            EditKind::InsertStr(s) => {
                text.splice(offset, 0, s);
            }
            EditKind::DeleteStr(s) => {
                text.splice(offset, s.chars().count() as isize, "");
            }
        }
    }

    fn invert(&self) -> Self {
        use EditKind::*;
        match self.clone() {
            InsertChar(c) => DeleteChar(c),
            DeleteChar(c) => InsertChar(c),
            InsertNewline => DeleteNewline,
            DeleteNewline => InsertNewline,
            InsertStr(s) => DeleteStr(s),
            DeleteStr(s) => InsertStr(s),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edit {
    kind: EditKind,
    offset: usize,
}

impl Edit {
    pub fn new(kind: EditKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    pub fn redo(&self, lines: &mut autosurgeon::Text) {
        self.kind.apply(lines, self.offset);
    }

    pub fn undo(&self, lines: &mut autosurgeon::Text) {
        self.kind.invert().apply(lines, self.offset); // Undo is redo of inverted edit
    }

    pub fn cursor_before(&self) -> usize {
        self.offset
    }

    pub fn cursor_after(&self) -> usize {
        self.offset
    }
}

#[derive(Clone, Debug)]
pub struct History {
    index: usize,
    max_items: usize,
    edits: VecDeque<Edit>,
}

impl History {
    pub fn new(max_items: usize) -> Self {
        Self {
            index: 0,
            max_items,
            edits: VecDeque::new(),
        }
    }

    pub fn push(&mut self, edit: Edit) {
        if self.max_items == 0 {
            return;
        }

        if self.edits.len() == self.max_items {
            self.edits.pop_front();
            self.index = self.index.saturating_sub(1);
        }

        if self.index < self.edits.len() {
            self.edits.truncate(self.index);
        }

        self.index += 1;
        self.edits.push_back(edit);
    }

    pub fn redo(&mut self, text: &mut autosurgeon::Text) -> Option<usize> {
        if self.index == self.edits.len() {
            return None;
        }
        let edit = &self.edits[self.index];
        edit.redo(text);
        self.index += 1;
        Some(edit.cursor_after())
    }

    pub fn undo(&mut self, text: &mut autosurgeon::Text) -> Option<usize> {
        self.index = self.index.checked_sub(1)?;
        let edit = &self.edits[self.index];
        edit.undo(text);
        Some(edit.cursor_before())
    }

    pub fn max_items(&self) -> usize {
        self.max_items
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn insert_delete_chunk() {
        #[rustfmt::skip]
        let tests = [
            // Positions
            (
                // Text before edit
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                // (row, col) position before edit
                0,
                // Chunk to be inserted
                &[
                    "x", "y",
                ][..],
                // Text after edit
                &[
                    "x",
                    "yab",
                    "cd",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                1,
                &[
                    "x", "y",
                ][..],
                &[
                    "ax",
                    "yb",
                    "cd",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                2,
                &[
                    "x", "y",
                ][..],
                &[
                    "abx",
                    "y",
                    "cd",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                3,
                &[
                    "x", "y",
                ][..],
                &[
                    "ab",
                    "x",
                    "ycd",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                4,
                &[
                    "x", "y",
                ][..],
                &[
                    "ab",
                    "cx",
                    "yd",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                5,
                &[
                    "x", "y",
                ][..],
                &[
                    "ab",
                    "cdx",
                    "y",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                6,
                &[
                    "x", "y",
                ][..],
                &[
                    "ab",
                    "cd",
                    "x",
                    "yef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                7,
                &[
                    "x", "y",
                ][..],
                &[
                    "ab",
                    "cd",
                    "ex",
                    "yf",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                8,
                &[
                    "x", "y",
                ][..],
                &[
                    "ab",
                    "cd",
                    "efx",
                    "y",
                ][..],
            ),
            // More than 2 lines
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                4,
                &[
                    "x", "y", "z", "w"
                ][..],
                &[
                    "ab",
                    "cx",
                    "y",
                    "z",
                    "wd",
                    "ef",
                ][..],
            ),
            // Empty lines
            (
                &[
                    "",
                    "",
                    "",
                ][..],
                0,
                &[
                    "x", "y", "z"
                ][..],
                &[
                    "x",
                    "y",
                    "z",
                    "",
                    "",
                ][..],
            ),
            (
                &[
                    "",
                    "",
                    "",
                ][..],
                1,
                &[
                    "x", "y", "z"
                ][..],
                &[
                    "",
                    "x",
                    "y",
                    "z",
                    "",
                ][..],
            ),
            (
                &[
                    "",
                    "",
                    "",
                ][..],
                2,
                &[
                    "x", "y", "z"
                ][..],
                &[
                    "",
                    "",
                    "x",
                    "y",
                    "z",
                ][..],
            ),
            // Empty buffer
            (
                &[][..],
                0,
                &[
                    "x", "y", "z"
                ][..],
                &[
                    "x",
                    "y",
                    "z",
                ][..],
            ),
            // Insert empty lines
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                0,
                &[
                    "", "", "",
                ][..],
                &[
                    "",
                    "",
                    "ab",
                    "cd",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                3,
                &[
                    "", "", "",
                ][..],
                &[
                    "ab",
                    "",
                    "",
                    "cd",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                4,
                &[
                    "", "", "",
                ][..],
                &[
                    "ab",
                    "c",
                    "",
                    "d",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                5,
                &[
                    "", "", "",
                ][..],
                &[
                    "ab",
                    "cd",
                    "",
                    "",
                    "ef",
                ][..],
            ),
            (
                &[
                    "ab",
                    "cd",
                    "ef",
                ][..],
                8,
                &[
                    "", "", "",
                ][..],
                &[
                    "ab",
                    "cd",
                    "ef",
                    "",
                    "",
                ][..],
            ),
            // Multi-byte characters
            (
                &[
                    "🐶🐱",
                    "🐮🐰",
                    "🐧🐭",
                ][..],
                0,
                &[
                    "🐷", "🐼", "🐴",
                ][..],
                &[
                    "🐷",
                    "🐼",
                    "🐴🐶🐱",
                    "🐮🐰",
                    "🐧🐭",
                ][..],
            ),
            // (
            //     &[
            //         "🐶🐱",
            //         "🐮🐰",
            //         "🐧🐭",
            //     ][..],
            //     2,
            //     &[
            //         "🐷", "🐼", "🐴",
            //     ][..],
            //     &[
            //         "🐶🐱🐷",
            //         "🐼",
            //         "🐴",
            //         "🐮🐰",
            //         "🐧🐭",
            //     ][..],
            // ),
            // (
            //     &[
            //         "🐶🐱",
            //         "🐮🐰",
            //         "🐧🐭",
            //     ][..],
            //     3,
            //     &[
            //         "🐷", "🐼", "🐴",
            //     ][..],
            //     &[
            //         "🐶🐱",
            //         "🐷",
            //         "🐼",
            //         "🐴🐮🐰",
            //         "🐧🐭",
            //     ][..],
            // ),
            // (
            //     &[
            //         "🐶🐱",
            //         "🐮🐰",
            //         "🐧🐭",
            //     ][..],
            //     (1, 1),
            //     &[
            //         "🐷", "🐼", "🐴",
            //     ][..],
            //     &[
            //         "🐶🐱",
            //         "🐮🐷",
            //         "🐼",
            //         "🐴🐰",
            //         "🐧🐭",
            //     ][..],
            // ),
            // (
            //     &[
            //         "🐶🐱",
            //         "🐮🐰",
            //         "🐧🐭",
            //     ][..],
            //     (2, 2),
            //     &[
            //         "🐷", "🐼", "🐴",
            //     ][..],
            //     &[
            //         "🐶🐱",
            //         "🐮🐰",
            //         "🐧🐭🐷",
            //         "🐼",
            //         "🐴",
            //     ][..],
            // ),
        ];

        for test in tests {
            let (before, pos, input, expected) = test;
            let mut lines: autosurgeon::Text = before.join("\n").into();
            let chunk = input.join("\n");

            let edit = EditKind::InsertStr(chunk.clone());
            edit.apply(&mut lines, pos);
            assert_eq!(lines.as_str(), expected.join("\n"), "{test:?}");

            let edit = EditKind::DeleteStr(chunk);
            edit.apply(&mut lines, pos);
            assert_eq!(lines.as_str(), before.join("\n"), "{test:?}");
        }
    }
}

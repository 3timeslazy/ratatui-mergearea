use ratatui_mergearea::{CursorMoveV2 as CursorMove, TextArea};

const BOTTOM_RIGHT: CursorMove = CursorMove::Jump(u16::MAX, u16::MAX);

#[test]
fn empty_textarea() {
    use CursorMove::*;

    let mut t = TextArea::default();
    for m in [
        Forward, Back, Up, Down, Head, End, Top,
        Bottom,
        // WordForward,
        // WordEnd,
        // WordBack,
        // ParagraphForward,
        // ParagraphBack,
        // Jump(0, 0),
        // Jump(u16::MAX, u16::MAX),
    ] {
        t.move_cursor_v2(m);
        assert_eq!(t.cursor2(), (0, 0), "{m:?}");
    }
}

#[test]
fn forward() {
    for (text, positions) in [
        (
            ["abc", "def"],
            [
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 3),
            ],
        ),
        (
            ["あいう", "🐶🐱👪"],
            [
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 3),
            ],
        ),
    ] {
        let mut t = TextArea::with_value(text.join("\n"));

        for pos in positions {
            t.move_cursor_v2(CursorMove::Forward);
            assert_eq!(t.cursor2(), pos, "{:?}", t.text());
        }
    }
}

#[test]
fn back() {
    for (text, positions) in [
        (
            ["abc", "def"],
            [
                (1, 2),
                (1, 1),
                (1, 0),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
            ],
        ),
        (
            ["あいう", "🐶🐱👪"],
            [
                (1, 2),
                (1, 1),
                (1, 0),
                (0, 3),
                (0, 2),
                (0, 1),
                (0, 0),
                (0, 0),
            ],
        ),
    ] {
        let mut t = TextArea::with_value(text.join("\n"));
        t.move_cursor_v2(BOTTOM_RIGHT);

        for pos in positions {
            t.move_cursor_v2(CursorMove::Back);
            assert_eq!(t.cursor2(), pos);
        }
    }
}

// #[test]
// fn up() {
//     for (text, positions, init_offset) in [
//         // don't move if on the first line
//         ("", &[0][..], 0),
//         ("a", &[0][..], 0),
//         ("a", &[1][..], 1),
//         ("ab", &[1][..], 1),
//         ("ab", &[2][..], 2),
//         ("a\n", &[1][..], 1),
//         ("a\n", &[0][..], 2),
//         // move up in the the same column
//         ("a\nb", &[0][..], 2),
//         ("a\nbc", &[0][..], 2),
//         ("a\nb\nc", &[2, 0][..], 4),
//         ("ab\ncd", &[1][..], 4),
//         // move from eol to eol
//         ("a\nb", &[1][..], 3),
//         ("a\nb\n", &[2, 0][..], 4),
//         ("\n\n", &[1, 0][..], 2),
//         ("\n\n\n", &[1, 0][..], 2),
//         ("ab\ncd", &[2][..], 5),
//         // moving from longer line to shorter line
//         ("a\nbc", &[1][..], 3),
//         ("a\nbc", &[1][..], 4),
//         // moving from shorter line to longer line
//         ("ab\nc", &[0][..], 3),
//         ("ab\nc", &[1][..], 4),
//         // Unicode
//         ("й\nу\n𝑥", &[2, 0][..], 4),
//         ("🐶\n🐱", &[0][..], 2),
//     ] {
//         let at = autosurgeon::Text::from(text);
//         let mut t = TextArea::new(at.clone());
//         t.cursor_v2 = init_offset;

//         for pos in positions {
//             t.move_cursor_v2(CursorMoveV2::Up);
//             assert_eq!(t.cursor_v2, *pos);
//         }
//     }
// }

#[test]
fn up() {
    for text in [["abc", "def", "ghi"], ["あいう", "🐶🐱🐰", "👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]]
    {
        let mut t = TextArea::with_value(text.join("\n"));

        for col in 0..=3 {
            let mut row = 2;

            t.move_cursor_v2(CursorMove::Jump(2, col as u16));
            assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());

            while row > 0 {
                t.move_cursor_v2(CursorMove::Up);
                row -= 1;
                assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());
            }
        }
    }
}

#[test]
fn up_trim() {
    for text in [["", "a", "bcd", "efgh"], ["", "👪", "🐶!🐱", "あ?い!"]] {
        let mut t = TextArea::with_value(text.join("\n"));
        t.move_cursor_v2(CursorMove::Jump(3, 3));

        for expected in [(2, 3), (1, 1), (0, 0)] {
            t.move_cursor_v2(CursorMove::Up);
            assert_eq!(t.cursor2(), expected, "{:?}", t.text());
        }
    }
}

// #[test]
// fn down() {
//     for (text, positions, init_offset) in [
//         // don't move if on the first line
//         ("", &[0][..], 0),
//         ("a", &[0][..], 0),
//         ("a", &[1][..], 1),
//         ("ab", &[1][..], 1),
//         ("ab", &[2][..], 2),
//         // move down in the the same column
//         ("a\n", &[2][..], 1),
//         ("a\nb", &[2][..], 0),
//         ("a\nbc", &[2][..], 0),
//         ("a\nb\nc", &[2, 4][..], 0),
//         ("ab\ncd", &[4][..], 1),
//         // move from eol to eol
//         ("a\nb", &[3][..], 1),
//         ("a\nb\n", &[3, 4][..], 1),
//         ("\n\n", &[1, 2][..], 0),
//         ("ab\ncd", &[5][..], 2),
//         ("ab\nc\n", &[4][..], 2),
//         // moving from longer line to shorter line
//         ("ab\nc", &[4][..], 2),
//         ("ab\nc", &[4][..], 1),
//         // moving from shorter line to longer line
//         ("a\nbc", &[2][..], 0),
//         ("a\nbc", &[3][..], 1),
//         // // Unicode
//         ("й\nу\n𝑥", &[2, 4][..], 0),
//         ("🐶\n🐱", &[2][..], 0),
//     ] {
//         let mut t = TextArea::with_value(text);
//         t.cursor_v2 = init_offset;

//         for i in 0..positions.len() {
//             t.move_cursor_v2(CursorMove::Down);
//             assert_eq!(
//                 t.cursor_v2,
//                 positions[i],
//                 "{:?} {positions:?} {init_offset}",
//                 text.chars().collect::<Vec<char>>()
//             );
//         }
//     }
// }

#[test]
fn down() {
    for text in [["abc", "def", "ghi"], ["あいう", "🐶🐱🐰", "👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]]
    {
        let mut t = TextArea::with_value(text.join("\n"));

        for col in 0..=3 {
            let mut row = 0;

            t.move_cursor_v2(CursorMove::Jump(0, col as u16));
            assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());

            while row < 2 {
                t.move_cursor_v2(CursorMove::Down);
                row += 1;
                assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());
            }
        }
    }
}

#[test]
fn down_trim() {
    for text in [["abcd", "efg", "h", ""], ["あ?い!", "🐶!🐱", "👪", ""]] {
        let mut t = TextArea::with_value(text.join("\n"));
        t.move_cursor_v2(CursorMove::Jump(0, 3));

        for expected in [(1, 3), (2, 1), (3, 0)] {
            t.move_cursor_v2(CursorMove::Down);
            assert_eq!(t.cursor2(), expected, "{:?}", t.text());
        }
    }
}

#[test]
fn head() {
    for text in ["efg\nh\n", "あいう\n👪\n"] {
        let mut t = TextArea::with_value(text);
        let lines = t
            .text()
            .as_str()
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for (row, line) in lines.iter().enumerate() {
            let len = line.len();
            for col in [0, len / 2, len] {
                t.move_cursor_v2(CursorMove::Jump(row as u16, col as u16));
                t.move_cursor_v2(CursorMove::Head);
                assert_eq!(t.cursor2(), (row, 0), "{lines:?}");
            }
        }
    }
}

#[test]
fn end() {
    for text in [["efg", "h", ""], ["あいう", "👪", ""]] {
        let mut t = TextArea::with_value(text.join("\n"));
        for row in 0..t.lines().len() {
            let len = match row {
                0 => 3,
                1 => 1,
                2 => 0,
                _ => unreachable!(),
            };
            for col in [0, len / 2, len] {
                t.move_cursor_v2(CursorMove::Jump(row as u16, col as u16));
                t.move_cursor_v2(CursorMove::End);
                assert_eq!(t.cursor2(), (row, len), "{:?}", t.text());
            }
        }
    }
}

#[test]
fn top() {
    for text in ["abc\ndef\nghi", "あいう\n🐶🐱🐰\n👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]
    {
        let mut t = TextArea::with_value(text);

        for row in 0..=2 {
            for col in 0..=3 {
                t.move_cursor_v2(CursorMove::Jump(row as u16, col as u16));
                t.move_cursor_v2(CursorMove::Top);
                assert_eq!(t.cursor2(), (0, col as usize), "{:?} {row} {col}", t.text());
            }
        }
    }
}

#[test]
fn top_trim() {
    for lines in [
        &["a", "bc"][..],
        &["あ", "🐶🐱"][..],
        &["a", "bcd", "ef"][..],
        &["", "犬"][..],
    ] {
        let mut t = {
            let str = lines
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            TextArea::with_value(str)
        };
        t.move_cursor_v2(CursorMove::Jump(u16::MAX, u16::MAX));
        t.move_cursor_v2(CursorMove::Top);
        let col = t.text().as_str().chars().position(|c| c == '\n').unwrap();
        assert_eq!(t.cursor2(), (0, col), "{:?}", t.text());
    }
}

#[test]
fn bottom() {
    for text in [["abc", "def", "ghi"], ["あいう", "🐶🐱🐰", "👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]]
    {
        let mut t = TextArea::with_value(text.join("\n"));
        for row in 0..=2 {
            for col in 0..=3 {
                t.move_cursor_v2(CursorMove::Jump(row, col));
                t.move_cursor_v2(CursorMove::Bottom);
                assert_eq!(t.cursor2(), (2, col as usize), "{:?}", t.text());
            }
        }
    }
}

#[test]
fn bottom_trim() {
    for lines in [
        &["bc", "a"][..],
        &["🐶🐱", "🐰"][..],
        &["ef", "bcd", "a"][..],
        &["犬", ""][..],
    ] {
        let mut t = TextArea::with_value(lines.join("\n"));
        t.move_cursor_v2(CursorMove::Jump(0, u16::MAX));
        t.move_cursor_v2(CursorMove::Bottom);
        let text_lines = t.text().as_str().lines().collect::<Vec<&str>>();
        let col = text_lines.last().unwrap().chars().count();
        assert_eq!(t.cursor2(), (text_lines.len() - 1, col), "{:?}", t.text());
    }
}

// #[test]
// fn word_end() {
//     for (lines, positions) in [
//         (
//             &[
//                 "aaa !!! bbb", // Consecutive punctuations are a word
//             ][..],
//             &[(0, 2), (0, 6), (0, 10)][..],
//         ),
//         (
//             &[
//                 "aaa!!!bbb", // Word boundaries without spaces
//             ][..],
//             &[(0, 2), (0, 5), (0, 8)][..],
//         ),
//         (
//             &[
//                 "aaa", "", "", "bbb", // Go across multiple empty lines (regression of #75)
//             ][..],
//             &[(0, 2), (3, 2)][..],
//         ),
//         (
//             &[
//                 "aaa", "   ", "   ", "bbb", // Go across multiple blank lines
//             ][..],
//             &[(0, 2), (3, 2)][..],
//         ),
//         (
//             &[
//                 "   aaa", "   bbb", // Ignore the spaces at the head of line
//             ][..],
//             &[(0, 5), (1, 5)][..],
//         ),
//         (
//             &[
//                 "aaa   ", "bbb   ", // Ignore the spaces at the end of line
//             ][..],
//             &[(0, 2), (1, 2), (1, 6)][..],
//         ),
//         (
//             &[
//                 "a aa", "b!!!", // Accept the head of line (regression of #75)
//             ][..],
//             &[(0, 3), (1, 0), (1, 3)][..],
//         ),
//     ] {
//         let mut t: TextArea = lines.iter().cloned().collect();
//         for pos in positions {
//             t.move_cursor(CursorMove::WordEnd);
//             assert_eq!(t.cursor(), *pos, "{:?}", t.lines());
//         }
//     }
// }

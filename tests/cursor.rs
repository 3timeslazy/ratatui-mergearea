use ratatui_mergearea::{CursorMove, MergeArea};

const BOTTOM_RIGHT: CursorMove = CursorMove::Jump(u16::MAX, u16::MAX);

#[test]
fn empty_textarea() {
    use CursorMove::*;

    let mut t = MergeArea::default();
    for m in [
        Forward,
        Back,
        Up,
        Down,
        Head,
        End,
        Top,
        Bottom,
        WordForward,
        WordEnd,
        WordBack,
        Jump(0, 0),
        Jump(u16::MAX, u16::MAX),
    ] {
        t.move_cursor(m);
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
        let mut t = MergeArea::with_value(text.join("\n"));

        for pos in positions {
            t.move_cursor(CursorMove::Forward);
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
        let mut t = MergeArea::with_value(text.join("\n"));
        t.move_cursor(BOTTOM_RIGHT);

        for pos in positions {
            t.move_cursor(CursorMove::Back);
            assert_eq!(t.cursor2(), pos);
        }
    }
}

#[test]
fn up() {
    for text in [["abc", "def", "ghi"], ["あいう", "🐶🐱🐰", "👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]]
    {
        let mut t = MergeArea::with_value(text.join("\n"));

        for col in 0..=3 {
            let mut row = 2;

            t.move_cursor(CursorMove::Jump(2, col as u16));
            assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());

            while row > 0 {
                t.move_cursor(CursorMove::Up);
                row -= 1;
                assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());
            }
        }
    }
}

#[test]
fn up_trim() {
    for text in [["", "a", "bcd", "efgh"], ["", "👪", "🐶!🐱", "あ?い!"]] {
        let mut t = MergeArea::with_value(text.join("\n"));
        t.move_cursor(CursorMove::Jump(3, 3));

        for expected in [(2, 3), (1, 1), (0, 0)] {
            t.move_cursor(CursorMove::Up);
            assert_eq!(t.cursor2(), expected, "{:?}", t.text());
        }
    }
}

#[test]
fn down() {
    for text in [["abc", "def", "ghi"], ["あいう", "🐶🐱🐰", "👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]]
    {
        let mut t = MergeArea::with_value(text.join("\n"));

        for col in 0..=3 {
            let mut row = 0;

            t.move_cursor(CursorMove::Jump(0, col as u16));
            assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());

            while row < 2 {
                t.move_cursor(CursorMove::Down);
                row += 1;
                assert_eq!(t.cursor2(), (row, col), "{:?}", t.text());
            }
        }
    }
}

#[test]
fn down_trim() {
    for text in [["abcd", "efg", "h", ""], ["あ?い!", "🐶!🐱", "👪", ""]] {
        let mut t = MergeArea::with_value(text.join("\n"));
        t.move_cursor(CursorMove::Jump(0, 3));

        for expected in [(1, 3), (2, 1), (3, 0)] {
            t.move_cursor(CursorMove::Down);
            assert_eq!(t.cursor2(), expected, "{:?}", t.text());
        }
    }
}

#[test]
fn head() {
    for text in ["efg\nh\n", "あいう\n👪\n"] {
        let mut t = MergeArea::with_value(text);
        let lines = t
            .text()
            .as_str()
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for (row, line) in lines.iter().enumerate() {
            let len = line.len();
            for col in [0, len / 2, len] {
                t.move_cursor(CursorMove::Jump(row as u16, col as u16));
                t.move_cursor(CursorMove::Head);
                assert_eq!(t.cursor2(), (row, 0), "{lines:?}");
            }
        }
    }
}

#[test]
fn end() {
    for text in [["efg", "h", ""], ["あいう", "👪", ""]] {
        let mut t = MergeArea::with_value(text.join("\n"));
        for row in 0..t.lines().len() {
            let len = match row {
                0 => 3,
                1 => 1,
                2 => 0,
                _ => unreachable!(),
            };
            for col in [0, len / 2, len] {
                t.move_cursor(CursorMove::Jump(row as u16, col as u16));
                t.move_cursor(CursorMove::End);
                assert_eq!(t.cursor2(), (row, len), "{:?}", t.text());
            }
        }
    }
}

#[test]
fn top() {
    for text in ["abc\ndef\nghi", "あいう\n🐶🐱🐰\n👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]
    {
        let mut t = MergeArea::with_value(text);

        for row in 0..=2 {
            for col in 0..=3 {
                t.move_cursor(CursorMove::Jump(row as u16, col as u16));
                t.move_cursor(CursorMove::Top);
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
            MergeArea::with_value(str)
        };
        t.move_cursor(CursorMove::Jump(u16::MAX, u16::MAX));
        t.move_cursor(CursorMove::Top);
        let col = t.text().as_str().chars().position(|c| c == '\n').unwrap();
        assert_eq!(t.cursor2(), (0, col), "{:?}", t.text());
    }
}

#[test]
fn bottom() {
    for text in [["abc", "def", "ghi"], ["あいう", "🐶🐱🐰", "👪🤟🏿👩🏻‍❤️‍💋‍👨🏾"]]
    {
        let mut t = MergeArea::with_value(text.join("\n"));
        for row in 0..=2 {
            for col in 0..=3 {
                t.move_cursor(CursorMove::Jump(row, col));
                t.move_cursor(CursorMove::Bottom);
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
        let mut t = MergeArea::with_value(lines.join("\n"));
        t.move_cursor(CursorMove::Jump(0, u16::MAX));
        t.move_cursor(CursorMove::Bottom);
        let text_lines = t.text().as_str().lines().collect::<Vec<&str>>();
        let col = text_lines.last().unwrap().chars().count();
        assert_eq!(t.cursor2(), (text_lines.len() - 1, col), "{:?}", t.text());
    }
}

#[test]
fn word_end() {
    for (lines, positions) in [
        (
            &[
                "aaa !!! bbb", // Consecutive punctuation is a word
            ][..],
            &[(0, 2), (0, 6), (0, 10)][..],
        ),
        (
            &[
                "aaa!!!bbb", // Word boundaries without spaces
            ][..],
            &[(0, 2), (0, 5), (0, 8)][..],
        ),
        (
            &[
                "aaa", "", "", "bbb", // Go across multiple empty lines (regression of #75)
            ][..],
            &[(0, 2), (3, 2)][..],
        ),
        (
            &[
                "aaa", "   ", "   ", "bbb", // Go across multiple blank lines
            ][..],
            &[(0, 2), (3, 2)][..],
        ),
        (
            &[
                "   aaa", "   bbb", // Ignore the spaces at the head of line
            ][..],
            &[(0, 5), (1, 5)][..],
        ),
        (
            &[
                "aaa   ", "bbb   ", // Ignore the spaces at the end of line
            ][..],
            &[(0, 2), (1, 2), (1, 6)][..],
        ),
        (
            &[
                "a aa", "b!!!", // Accept the head of line (regression of #75)
            ][..],
            &[(0, 3), (1, 0), (1, 3)][..],
        ),
        (&["aaa йу𝑥 🐶🐱"][..], &[(0, 2), (0, 6), (0, 9)][..]),
    ] {
        let mut t = MergeArea::with_value(lines.join("\n"));
        for pos in positions {
            t.move_cursor(CursorMove::WordEnd);
            assert_eq!(t.cursor2(), *pos, "{:?}", t.text());
        }
    }
}

#[test]
fn word_back() {
    for (lines, positions) in [
        (
            &[
                "aaa !!! bbb", // Consecutive punctuations are a word
            ][..],
            &[(0, 8), (0, 4), (0, 0)][..],
        ),
        (
            &[
                "aaa!!!bbb", // Word boundaries without spaces
            ][..],
            &[(0, 6), (0, 3), (0, 0)][..],
        ),
        (
            &[
                "aaa", "", "", "bbb", // Go across multiple empty lines (regression of #75)
            ][..],
            &[(3, 0), (0, 0)][..],
        ),
        (
            &[
                "aaa", "   ", "   ", "bbb", // Go across multiple blank lines
            ][..],
            &[(3, 0), (0, 0)][..],
        ),
        (
            &[
                "   aaa", "   bbb", // Ignore the spaces at the head of line
            ][..],
            &[(1, 3), (0, 3)][..],
        ),
        (
            &[
                "aaa   ", "bbb   ", // Ignore the spaces at the end of line
            ][..],
            &[(1, 0), (0, 0)][..],
        ),
        (&["a aa", "b!!!"][..], &[(1, 1), (1, 0), (0, 2), (0, 0)][..]),
        (&["aaa йу𝑥 🐶🐱"][..], &[(0, 8), (0, 4), (0, 0)][..]),
    ] {
        let mut t = MergeArea::with_value(lines.join("\n"));
        t.move_cursor(CursorMove::Jump(u16::MAX, u16::MAX));

        for pos in positions {
            t.move_cursor(CursorMove::WordBack);
            assert_eq!(t.cursor2(), *pos, "{:?}", t.text());
        }
    }
}

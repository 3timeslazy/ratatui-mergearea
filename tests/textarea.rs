use ratatui_mergearea::{CursorMove, TextArea};
use std::cmp;
use std::fmt::Debug;

fn assert_undo_redo<T: Debug>(
    before_pos: (usize, usize),
    before_buf: &str,
    after_buf: &str,
    t: &mut TextArea<'_>,
    context: T,
) {
    let after_pos = t.cursor2();
    let modified = before_buf != after_buf;
    assert_eq!(t.cursor2(), after_pos, "pos before undo: {context:?}");
    assert_eq!(t.undo(), modified, "undo modification: {context:?}");
    assert_eq!(t.text().as_str(), before_buf, "buf after undo: {context:?}");
    assert_eq!(t.cursor2(), before_pos, "pos after undo: {context:?}");
    assert_eq!(t.redo_v2(), modified, "redo modification: {context:?}");
    assert_eq!(t.text().as_str(), after_buf, "buf after redo: {context:?}");
    // assert_eq!(t.cursor2(), after_pos, "pos after redo: {context:?}");
}

fn assert_no_undo_redo<T: Debug>(t: &mut TextArea<'_>, context: T) {
    let pos = t.cursor2();
    let buf = t.text().clone();
    assert!(!t.undo(), "undo modification: {context:?}");
    assert_eq!(
        t.text().as_str(),
        buf.as_str(),
        "buf after undo: {context:?}"
    );
    assert_eq!(t.cursor2(), pos, "pos after undo: {context:?}");
    assert!(!t.redo(), "redo modification: {context:?}");
    assert_eq!(
        t.text().as_str(),
        buf.as_str(),
        "buf after redo: {context:?}"
    );
    assert_eq!(t.cursor2(), pos, "pos after redo: {context:?}");
}

#[test]
fn test_insert_soft_tab() {
    for test in [
        ("", 0, "    ", 4),
        ("a", 1, "a   ", 3),
        ("abcd", 4, "abcd    ", 4),
        ("a", 0, "    a", 4),
        ("ab", 1, "a   b", 3),
        ("abcdefgh", 4, "abcd    efgh", 4),
        ("あ", 1, "あ  ", 2),
        ("🐶", 1, "🐶  ", 2),
        ("あ", 0, "    あ", 4),
        // ("あい", 1, "あ  い", 2),
    ] {
        let (input, col, expected, width) = test;
        let mut t = TextArea::with_value(input);
        t.move_cursor(CursorMove::Jump(0, col));
        assert!(t.insert_tab(), "{test:?}");
        assert_eq!(t.text().as_str(), expected, "{test:?}");
        assert_eq!(t.cursor2(), (0, col as usize + width), "{test:?}");
        assert_undo_redo((0, col as _), input, expected, &mut t, test);
    }
}

#[test]
fn test_insert_hard_tab() {
    let mut t = TextArea::default();
    t.set_hard_tab_indent(true);
    assert!(t.insert_tab());
    assert_eq!(t.cursor2(), (0, 1));
    assert_undo_redo((0, 0), "", "\t", &mut t, "");

    let mut t = TextArea::default();
    t.set_hard_tab_indent(true);
    t.set_tab_length(0);
    t.insert_tab();
    assert!(!t.insert_tab());
    assert_eq!(t.text().as_str(), "");
    assert_eq!(t.cursor2(), (0, 0));
}

#[test]
fn test_insert_char() {
    let tests = [
        (0, 'x', "xab"),
        (1, 'x', "axb"),
        (2, 'x', "abx"),
        (1, 'あ', "aあb"),
        (1, '\n', "a\nb"),
    ];

    for test in tests {
        let (col, ch, want) = test;
        let mut t = TextArea::with_value("ab");
        t.move_cursor(CursorMove::Jump(0, col));
        t.insert_char(ch);
        assert_eq!(t.text().as_str(), want, "{test:?}");
        let pos = if ch == '\n' {
            (1, 0)
        } else {
            (0, col as usize + 1)
        };
        assert_eq!(t.cursor2(), pos, "{test:?}");
        assert_undo_redo((0, col as _), "ab", want, &mut t, test);
    }
}

#[test]
fn test_insert_str_one_line() {
    for i in 0..="ab".len() {
        let mut t = TextArea::with_value("ab");
        t.move_cursor(CursorMove::Jump(0, i as u16));
        assert!(t.insert_str("x"), "{i}");

        let mut want = "ab".to_string();
        want.insert(i, 'x');
        let want = want.as_str();
        assert_eq!(t.text().as_str(), want, "{i}");
        assert_eq!(t.cursor2(), (0, i + 1));
        assert_undo_redo((0, i), "ab", want, &mut t, i);
    }

    let mut t = TextArea::default();
    assert!(t.insert_str("x"));
    assert_eq!(t.cursor2(), (0, 1));
    assert_undo_redo((0, 0), "", "x", &mut t, "");
}

#[test]
fn test_insert_str_empty_line() {
    let mut t = TextArea::with_value("ab");
    assert!(!t.insert_str(""));
    assert_eq!(t.text().as_str(), "ab");
    assert_eq!(t.cursor2(), (0, 0));
    assert_no_undo_redo(&mut t, "");
}

#[test]
fn test_insert_str_multiple_lines() {
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
            // (row, offset) position before edit
            (0, 0),
            // String to be inserted
            "x\ny",
            // (row, offset) position after edit
            (1, 1),
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
            (0, 1),
            "x\ny",
            (1, 1),
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
            (0, 2),
            "x\ny",
            (1, 1),
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
            (1, 0),
            "x\ny",
            (2, 1),
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
            (1, 1),
            "x\ny",
            (2, 1),
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
            (1, 2),
            "x\ny",
            (2, 1),
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
            (2, 0),
            "x\ny",
            (3, 1),
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
            (2, 1),
            "x\ny",
            (3, 1),
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
            (2, 2),
            "x\ny",
            (3, 1),
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
            (1, 1),
            "x\ny\nz\nw",
            (4, 1),
            &[
                "ab",
                "cx",
                "y",
                "z",
                "wd",
                "ef",
            ][..],
        ),
        // Newline at end of line
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (1, 1),
            "x\ny\n",
            (3, 0),
            &[
                "ab",
                "cx",
                "y",
                "d",
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
            (0, 0),
            "x\ny\nz",
            (2, 1),
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
            (1, 0),
            "x\ny\nz",
            (3, 1),
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
               "\n\n\n"
            ][..],
            (2, 0),
            "x\ny\nz",
            (4, 1),
            &[
                "\n\nx\ny\nz\n"
            ][..],
        ),
        // Empty buffer
        (
            &[
                "",
            ][..],
            (0, 0),
            "x\ny\nz",
            (2, 1),
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
            (0, 0),
            "\n\n\n",
            (3, 0),
            &[
                "",
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
            (1, 0),
            "\n\n\n",
            (4, 0),
            &[
                "ab",
                "",
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
            (1, 1),
            "\n\n\n",
            (4, 0),
            &[
                "ab",
                "c",
                "",
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
            (1, 2),
            "\n\n\n",
            (4, 0),
            &[
                "ab",
                "cd",
                "",
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
            (2, 2),
            "\n\n\n",
            (5, 0),
            &[
                "ab",
                "cd",
                "ef",
                "",
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
            (0, 0),
            "🐷\n🐼\n🐴",
            (2, 1),
            &[
                "🐷",
                "🐼",
                "🐴🐶🐱",
                "🐮🐰",
                "🐧🐭",
            ][..],
        ),
        (
            &[
                "🐶🐱",
                "🐮🐰",
                "🐧🐭",
            ][..],
            (0, 2),
            "🐷\n🐼\n🐴",
            (2, 1),
            &[
                "🐶🐱🐷",
                "🐼",
                "🐴",
                "🐮🐰",
                "🐧🐭",
            ][..],
        ),
        (
            &[
                "🐶🐱",
                "🐮🐰",
                "🐧🐭",
            ][..],
            (1, 0),
            "🐷\n🐼\n🐴",
            (3, 1),
            &[
                "🐶🐱",
                "🐷",
                "🐼",
                "🐴🐮🐰",
                "🐧🐭",
            ][..],
        ),
        (
            &[
                "🐶🐱",
                "🐮🐰",
                "🐧🐭",
            ][..],
            (1, 1),
            "🐷\n🐼\n🐴",
            (3, 1),
            &[
                "🐶🐱",
                "🐮🐷",
                "🐼",
                "🐴🐰",
                "🐧🐭",
            ][..],
        ),
        (
            &[
                "🐶🐱",
                "🐮🐰",
                "🐧🐭",
            ][..],
            (2, 2),
            "🐷\n🐼\n🐴",
            (4, 1),
            &[
                "🐶🐱",
                "🐮🐰",
                "🐧🐭🐷",
                "🐼",
                "🐴",
            ][..],
        ),
        // Handle \r\n as newlines
        // (
        //     &[
        //         "ab",
        //         "cd",
        //         "ef",
        //     ][..],
        //     (1, 1),
        //     "x\r\ny\r\nz",
        //     (3, 1),
        //     &[
        //         "ab",
        //         "cx",
        //         "y",
        //         "zd",
        //         "ef",
        //     ][..],
        // ),
        // (
        //     &[
        //         "ab",
        //         "cd",
        //         "ef",
        //     ][..],
        //     (1, 1),
        //     "x\ny\r\nz",
        //     (3, 1),
        //     &[
        //         "ab",
        //         "cx",
        //         "y",
        //         "zd",
        //         "ef",
        //     ][..],
        // ),
    ];

    for test in tests {
        let (before, before_pos, input, after_pos, expected) = test;
        let expected = expected.join("\n");
        let before = before.join("\n");

        let mut t = TextArea::with_value(&before);
        let (row, col) = before_pos;
        t.move_cursor(CursorMove::Jump(row as _, col as _));

        assert!(t.insert_str(input), "{test:?}");
        assert_eq!(t.text().as_str(), expected, "{test:?}");
        assert_eq!(t.cursor2(), after_pos, "{test:?}");

        // assert_undo_redo(before_pos, &before, &expected, &mut t, test);
    }
}

#[test]
fn test_delete_str_nothing() {
    for i in 0..="ab".len() {
        let mut t = TextArea::with_value("ab");
        assert!(!t.delete_str(0), "{i}");
        assert_eq!(t.cursor2(), (0, 0));
    }
    let mut t = TextArea::default();
    assert!(!t.delete_str(0));
    assert_eq!(t.cursor2(), (0, 0));
}

#[test]
fn test_delete_str_within_line() {
    for i in 0.."abc".len() {
        for j in 1..="abc".len() - i {
            let mut t = TextArea::with_value("abc");
            t.move_cursor(CursorMove::Jump(0, i as _));
            assert!(t.delete_str(j), "at {i}, size={j}");

            let mut want = "abc".to_string();
            want.drain(i..i + j);
            let want = want.as_str();
            assert_eq!(t.text().as_str(), want, "at {i}, size={j}");
            assert_eq!(t.cursor2(), (0, i));

            // delete_str deletes string as if moving cursor at the end of the deleted string
            // assert_undo_redo((0, i + j), "abc", want, &mut t, (i, j));
        }
    }
}

#[test]
fn test_delete_str_multiple_lines() {
    #[rustfmt::skip]
    let tests = [
        // Length
        (
            // Text before edit
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            // (row, offset) cursor position
            (0, 0),
            // Chars to be deleted
            3,
            // Deleted text
            "ab\n",
            // Text after edit
            &[
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
            (0, 0),
            4,
            "ab\nc",
            &[
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
            (0, 0),
            5,
            "ab\ncd",
            &[
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
            (0, 0),
            6,
            "ab\ncd\n",
            &[
                "ef",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (0, 0),
            7,
            "ab\ncd\ne",
            &[
                "f",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (0, 0),
            8,
            "ab\ncd\nef",
            &[
                "",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (0, 0),
            9,
            "ab\ncd\nef",
            &[
                "",
            ][..],
        ),
        // Positions
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (0, 1),
            3,
            "b\nc",
            &[
                "ad",
                "ef",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (0, 2),
            4,
            "\ncd\n",
            &[
                "abef",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (1, 0),
            4,
            "cd\ne",
            &[
                "ab",
                "f",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (2, 0),
            3,
            "ef",
            &[
                "ab",
                "cd",
                "",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (2, 1),
            2,
            "f",
            &[
                "ab",
                "cd",
                "e",
            ][..],
        ),
        (
            &[
                "ab",
                "cd",
                "ef",
            ][..],
            (2, 2),
            1,
            "",
            &[
                "ab",
                "cd",
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
            (0, 0),
            1,
            "\n",
            &[
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
            (0, 0),
            2,
            "\n\n",
            &[
                "",
            ][..],
        ),
        (
            &[
                "",
                "",
                "",
            ][..],
            (0, 0),
            3,
            "\n\n",
            &[
                "",
            ][..],
        ),
        (
            &[
                "",
                "",
                "",
            ][..],
            (1, 0),
            1,
            "\n",
            &[
                "",
                "",
            ][..],
        ),
        // (
        //     &[
        //         "",
        //         "",
        //         "",
        //     ][..],
        //     (2, 0),
        //     1,
        //     "",
        //     &[
        //         "",
        //         "",
        //         "",
        //     ][..],
        // ),
        // Empty buffer
        (
            &[
                "",
            ][..],
            (0, 0),
            1,
            "",
            &[
                "",
            ][..],
        ),
    ];

    for test in tests {
        let (before, (row, col), chars, deleted, after) = test;
        let before = before.join("\n");
        let after = after.join("\n");

        let mut t = TextArea::with_value(&before);
        t.move_cursor(CursorMove::Jump(row as _, col as _));

        assert!(t.delete_str(chars), "{test:?}");
        assert_eq!(t.cursor2(), (row, col), "{test:?}");
        assert_eq!(t.text().as_str(), after, "{test:?}");
        assert_eq!(t.yank_text(), deleted, "{test:?}");

        // let pos = t.cursor2();
        // assert!(t.undo(), "{test:?}");
        // assert_eq!(t.text().as_str(), before, "{test:?}");
        // assert!(t.redo(), "{test:?}");
        // assert_eq!(t.text().as_str(), after, "{test:?}");
        // assert_eq!(t.cursor2(), pos, "{test:?}");
    }
}

#[test]
fn test_copy_single_line() {
    for i in 0..="abc".len() {
        for j in i.."abc".len() {
            let mut t = TextArea::with_value("abc");

            t.move_cursor(CursorMove::Jump(0, i as u16));
            t.start_selection();
            t.move_cursor(CursorMove::Jump(0, j as u16));
            t.copy();

            assert_eq!(t.yank_text(), &"abc"[i..j], "from {i} to {j}");
            assert_eq!(t.text().as_str(), "abc", "from {i} to {j}");

            assert_no_undo_redo(&mut t, (i, j));
        }
    }
}

// #[test]
// fn test_cut_single_line() {
//     for i in 0.."abc".len() {
//         for j in i + 1.."abc".len() {
//             let mut t = TextArea::with_value("abc");

//             t.move_cursor(CursorMove::Jump(0, i as u16));
//             t.start_selection_v2();
//             t.move_cursor(CursorMove::Jump(0, j as u16));
//             t.cut();

//             assert_eq!(t.yank_text(), &"abc"[i..j], "from {i} to {j}");

//             let mut after = "abc".to_string();
//             after.replace_range(i..j, "");
//             let after = after.as_str();
//             assert_eq!(t.text().as_str(), after, "from {i} to {j}");
//             assert_eq!(t.cursor2(), (0, i));
//             assert_undo_redo((0, j), "abc", after, &mut t, (i, j));

//             t.paste();
//             assert_eq!(t.text().as_str(), "abc", "from {i} to {j}");
//             assert_undo_redo((0, i), after, "abc", &mut t, (i, j));
//         }
//     }
// }

// #[test]
// fn test_copy_cut_empty() {
//     for row in 0..=2 {
//         for col in 0..=2 {
//             let check = |f: fn(&mut TextArea<'_>)| {
//                 let mut t = TextArea::from(["ab", "cd", "ef"]);
//                 t.move_cursor(CursorMove::Jump(row, col));
//                 t.start_selection();
//                 t.move_cursor(CursorMove::Jump(row, col));
//                 f(&mut t);
//                 assert!(!t.is_selecting());
//                 assert_eq!(t.cursor(), (row as _, col as _));
//                 assert_eq!(t.lines(), ["ab", "cd", "ef"]);
//                 assert_no_undo_redo(&mut t, "");
//             };

//             check(|t| {
//                 assert!(!t.cut());
//             });
//             check(|t| t.copy());
//         }
//     }
// }

// #[test]
// fn test_copy_cut_paste_multi_lines() {
//     #[rustfmt::skip]
//     let tests = [
//         (
//             // Initial text
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             // Start position of selection
//             (0, 0),
//             // End position of selection
//             (1, 0),
//             // Expected yanked text
//             "ab\n",
//             // Text buffer after cut
//             &[
//                 "cd",
//                 "ef",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 0),
//             (1, 1),
//             "ab\nc",
//             &[
//                 "d",
//                 "ef",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 0),
//             (1, 2),
//             "ab\ncd",
//             &[
//                 "",
//                 "ef",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 0),
//             (2, 0),
//             "ab\ncd\n",
//             &[
//                 "ef",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 0),
//             (2, 1),
//             "ab\ncd\ne",
//             &[
//                 "f",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 0),
//             (2, 2),
//             "ab\ncd\nef",
//             &[
//                 "",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 1),
//             (1, 1),
//             "b\nc",
//             &[
//                 "ad",
//                 "ef",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 2),
//             (1, 1),
//             "\nc",
//             &[
//                 "abd",
//                 "ef",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (1, 0),
//             (2, 1),
//             "cd\ne",
//             &[
//                 "ab",
//                 "f",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 2),
//             (1, 0),
//             "\n",
//             &[
//                 "abcd",
//                 "ef",
//             ][..]
//         ),
//         (
//             &[
//                 "ab",
//                 "cd",
//                 "ef",
//             ][..],
//             (0, 2),
//             (2, 0),
//             "\ncd\n",
//             &[
//                 "abef",
//             ][..]
//         ),
//         // Multi-byte characters
//         (
//             &[
//                 "あい",
//                 "うえ",
//                 "おか",
//             ][..],
//             (0, 0),
//             (2, 2),
//             "あい\nうえ\nおか",
//             &[
//                 "",
//             ][..]
//         ),
//         (
//             &[
//                 "あい",
//                 "うえ",
//                 "おか",
//             ][..],
//             (0, 1),
//             (2, 1),
//             "い\nうえ\nお",
//             &[
//                 "あか",
//             ][..]
//         ),
//         (
//             &[
//                 "あい",
//                 "うえ",
//                 "おか",
//             ][..],
//             (0, 2),
//             (2, 0),
//             "\nうえ\n",
//             &[
//                 "あいおか",
//             ][..]
//         ),
//         (
//             &[
//                 "あい",
//                 "うえ",
//                 "おか",
//             ][..],
//             (0, 2),
//             (1, 2),
//             "\nうえ",
//             &[
//                 "あい",
//                 "おか",
//             ][..]
//         ),
//         (
//             &[
//                 "あい",
//                 "うえ",
//                 "おか",
//             ][..],
//             (0, 2),
//             (1, 1),
//             "\nう",
//             &[
//                 "あいえ",
//                 "おか",
//             ][..]
//         ),
//         (
//             &[
//                 "あい",
//                 "うえ",
//                 "おか",
//             ][..],
//             (0, 2),
//             (1, 0),
//             "\n",
//             &[
//                 "あいうえ",
//                 "おか",
//             ][..]
//         ),
//     ];

//     for test in tests {
//         let (init_text, (srow, scol), (erow, ecol), yanked, after_cut) = test;

//         {
//             let mut t = TextArea::from(init_text.iter().map(|s| s.to_string()));
//             t.move_cursor(CursorMove::Jump(srow as _, scol as _));
//             t.start_selection();
//             t.move_cursor(CursorMove::Jump(erow as _, ecol as _));
//             t.copy();

//             assert_eq!(t.cursor(), (erow, ecol), "{test:?}");
//             assert_eq!(t.yank_text(), yanked, "{test:?}");
//             assert_eq!(t.lines(), init_text, "{test:?}");
//             assert_no_undo_redo(&mut t, test);
//         }

//         {
//             let mut t = TextArea::from(init_text.iter().map(|s| s.to_string()));
//             t.move_cursor(CursorMove::Jump(srow as _, scol as _));
//             t.start_selection();
//             t.move_cursor(CursorMove::Jump(erow as _, ecol as _));
//             t.cut();

//             assert_eq!(t.cursor(), (srow, scol), "{test:?}");
//             assert_eq!(t.yank_text(), yanked, "{test:?}");
//             assert_eq!(t.lines(), after_cut, "{test:?}");
//             assert_undo_redo((erow, ecol), init_text, after_cut, &mut t, test);

//             t.paste();
//             assert_eq!(t.lines(), init_text, "{test:?}");
//             assert_undo_redo((srow, scol), after_cut, init_text, &mut t, test);
//         }

//         // Reverse positions
//         {
//             let mut t = TextArea::from(init_text.iter().map(|s| s.to_string()));
//             t.move_cursor(CursorMove::Jump(erow as _, ecol as _));
//             t.start_selection();
//             t.move_cursor(CursorMove::Jump(srow as _, scol as _));
//             t.copy();

//             assert_eq!(t.cursor(), (srow, scol), "{test:?}");
//             assert_eq!(t.yank_text(), yanked, "{test:?}");
//             assert_eq!(t.lines(), init_text, "{test:?}");
//             assert_no_undo_redo(&mut t, test);
//         }

//         {
//             let mut t = TextArea::from(init_text.iter().map(|s| s.to_string()));
//             t.move_cursor(CursorMove::Jump(erow as _, ecol as _));
//             t.start_selection();
//             t.move_cursor(CursorMove::Jump(srow as _, scol as _));
//             t.cut();

//             assert_eq!(t.cursor(), (srow, scol), "{test:?}");
//             assert_eq!(t.yank_text(), yanked, "{test:?}");
//             assert_eq!(t.lines(), after_cut, "{test:?}");
//             assert_undo_redo((erow, ecol), init_text, after_cut, &mut t, test);

//             t.paste();
//             assert_eq!(t.lines(), init_text, "{test:?}");
//             assert_undo_redo((srow, scol), after_cut, init_text, &mut t, test);
//         }
//     }
// }

#[test]
fn test_delete_selection_on_delete_operations() {
    macro_rules! test_case {
        ($name:ident($($args:expr),*)) => {
            (
                stringify!($name),
                (|t| t.$name($($args),*)) as fn(&mut TextArea) -> bool,
            )
        };
    }

    let tests = [
        test_case!(delete_char()),
        test_case!(delete_next_char()),
        test_case!(delete_line_by_end()),
        // test_case!(delete_line_by_head()),
        // test_case!(delete_word()),
        // test_case!(delete_next_word()),
        test_case!(delete_str(3)),
    ];

    for (n, f) in tests {
        let mut t = TextArea::new(autosurgeon::Text::with_value("ab\ncd\nef"));
        t.move_cursor(CursorMove::Jump(0, 1));
        t.start_selection();
        t.move_cursor(CursorMove::Jump(2, 1));

        let modified = f(&mut t);
        assert!(modified, "{n}");
        assert_eq!(t.text().as_str(), "af", "{n}");
        assert_eq!(t.cursor(), 1);

        // assert_undo_redo((2, 1), "ab\ncd\nef", "af", &mut t, n);
    }
}

#[test]
fn test_delete_selection_on_delete_edge_cases() {
    macro_rules! test_case {
        ($name:ident($($args:expr),*), $pos:expr) => {
            (
                stringify!($name),
                (|t| t.$name($($args),*)) as fn(&mut TextArea) -> bool,
                $pos,
            )
        };
    }

    // When deleting nothing and deleting newline
    let tests = [
        test_case!(delete_char(), (0, 0)),
        test_case!(delete_char(), (1, 0)),
        test_case!(delete_next_char(), (2, 2)),
        test_case!(delete_next_char(), (1, 2)),
        test_case!(delete_line_by_end(), (0, 2)),
        test_case!(delete_line_by_end(), (2, 2)),
        // test_case!(delete_line_by_head(), (0, 0)),
        // test_case!(delete_line_by_head(), (1, 0)),
        // test_case!(delete_word(), (0, 0)),
        // test_case!(delete_word(), (1, 0)),
        // test_case!(delete_next_word(), (2, 2)),
        // test_case!(delete_next_word(), (1, 2)),
        test_case!(delete_str(0), (0, 0)),
        test_case!(delete_str(100), (2, 2)),
    ];

    for (n, f, pos) in tests {
        let mut t = TextArea::with_value("ab\ncd\nef");
        t.move_cursor(CursorMove::Jump(1, 1));
        t.start_selection();
        t.move_cursor(CursorMove::Jump(pos.0 as _, pos.1 as _));

        assert!(f(&mut t), "{n}, {pos:?}");
        assert_eq!(t.cursor2(), cmp::min(pos, (1, 1)), "{n}, {pos:?}");

        t.undo();
        assert_eq!(t.text().as_str(), "ab\ncd\nef", "{n}, {pos:?}");
    }
}

// #[test]
// fn test_delete_selection_before_insert() {
//     macro_rules! test_case {
//         ($name:ident($($args:expr),*), $want:expr) => {
//             (
//                 stringify!($name),
//                 (|t| {
//                     t.$name($($args),*);
//                 }) as fn(&mut TextArea),
//                 &$want as &str,
//             )
//         };
//     }

//     let tests = [
//         test_case!(insert_newline_v2(), "a\nf"),
//         test_case!(insert_char_v2('x'), "axf"),
//         test_case!(insert_tab_v2(), "a   f"), // Default tab is 4 spaces
//         // test_case!(insert_str("xyz"), ["axyzf"]),
//     ];

//     for (n, f, after) in tests {
//         let mut t = TextArea::with_value("ab\ncd\nef");
//         t.move_cursor(CursorMove::Jump(0, 1));
//         t.start_selection_v2();
//         t.move_cursor(CursorMove::Jump(2, 1));

//         f(&mut t);
//         assert_eq!(t.text().as_str(), after, "{n}");

//         // XXX: Deleting selection and inserting text are separate undo units for now
//         // t.undo();
//         // t.undo();
//         // assert_eq!(t.lines(), ["ab", "cd", "ef"], "{n}");
//     }
// }

// #[test]
// fn test_undo_redo_stop_selection() {
//     fn check(t: &mut TextArea, f: fn(&mut TextArea) -> bool) {
//         t.move_cursor(CursorMove::Jump(0, 0));
//         t.start_selection();
//         t.move_cursor(CursorMove::Jump(0, 1));
//         assert!(t.is_selecting());
//         assert!(f(t));
//         assert!(!t.is_selecting());
//     }

//     let mut t = TextArea::default();
//     t.insert_char('a');

//     check(&mut t, |t| t.undo());
//     assert_eq!(t.lines(), [""]);
//     check(&mut t, |t| t.redo());
//     assert_eq!(t.lines(), ["a"]);
// }

// #[test]
// fn test_set_yank_paste_text() {
//     let tests = [
//         ("", &[""][..], (0, 0)),
//         ("abc", &["abc"][..], (0, 3)),
//         ("abc\ndef", &["abc", "def"][..], (1, 3)),
//         ("\n\n", &["", "", ""][..], (2, 0)),
//     ];

//     for test in tests {
//         let (text, want, pos) = test;
//         let mut t = TextArea::default();
//         t.set_yank_text(text);
//         t.paste();
//         assert_eq!(t.lines(), want, "{test:?}");
//         assert_eq!(t.yank_text(), text, "{test:?}");
//         assert_eq!(t.cursor(), pos, "{test:?}");
//         assert_undo_redo((0, 0), &[""], want, &mut t, test);
//     }
// }

// #[test]
// fn test_set_yank_crlf() {
//     let tests = [
//         ("\r\n", &["", ""][..], "\n"),
//         ("\r\n\r\n", &["", "", ""][..], "\n\n"),
//         ("a\r\nb", &["a", "b"][..], "a\nb"),
//         ("a\r\nb\r\n", &["a", "b", ""][..], "a\nb\n"),
//     ];
//     for test in tests {
//         let (pasted, lines, yanked) = test;
//         let mut t = TextArea::default();
//         t.set_yank_text(pasted);
//         t.paste();
//         assert_eq!(t.lines(), lines, "{test:?}");
//         assert_eq!(t.yank_text(), yanked, "{test:?}");
//     }
// }

#[test]
fn test_select_all() {
    let mut t = TextArea::with_value("aaa\nbbb\nccc");
    t.select_all();
    assert!(t.is_selecting());
    assert_eq!(t.cursor2(), (2, 3));
    t.cut();
    assert_eq!(t.text().as_str(), "");
    assert_eq!(t.yank_text(), "aaa\nbbb\nccc");
    // assert_undo_redo((2, 3), "aaa\nbbb\nccc", "", &mut t, "");
}

// #[test]
// fn test_paste_while_selection() {
//     let mut t = TextArea::from(["ab", "cd"]);
//     t.move_cursor(CursorMove::Jump(0, 1));
//     t.start_selection();
//     t.move_cursor(CursorMove::Jump(1, 1));
//     t.set_yank_text("x\ny");
//     assert!(t.paste());
//     assert_eq!(t.lines(), ["ax", "yd"]);
//     assert_eq!(t.cursor(), (1, 1));
//     assert!(!t.is_selecting());

//     let mut t = TextArea::from(["ab", "cd"]);
//     t.select_all();
//     t.set_yank_text("xy\nzw");
//     assert!(t.paste());
//     assert_eq!(t.lines(), ["xy", "zw"]);
//     assert_eq!(t.cursor(), (1, 2));
//     assert!(!t.is_selecting());
// }

// #[test]
// fn test_selection_range() {
//     #[rustfmt::skip]
//     let mut t = TextArea::with_value([
//         "あいうえお",
//         "Hello",
//         "🐶🐱🐰🐮🐹",
//         ].join("\n"));

//     assert_eq!(t.selection_range_v2(), None);

//     for (from, to) in [
//         ((0, 0), (0, 0)),
//         ((2, 5), (2, 5)),
//         ((0, 2), (2, 3)),
//         ((2, 1), (0, 4)),
//         ((0, 0), (2, 5)),
//         ((2, 5), (0, 0)),
//     ] {
//         let (x, y) = from;
//         t.move_cursor(CursorMove::Jump(x as _, y as _));

//         t.start_selection_v2();

//         let (x, y) = to;
//         t.move_cursor(CursorMove::Jump(x as _, y as _));

//         // TODO: select range with 2d index
//         let have = t.selection_range_v2().unwrap();
//         let want = if from <= to { (from, to) } else { (to, from) };
//         assert_eq!(have, want, "selection from {from:?} to {to:?}");

//         t.cancel_selection_v2();
//         let range = t.selection_range_v2();
//         assert_eq!(range, None, "selection from {from:?} to {to:?}");
//     }
// }

struct DeleteTester(&'static str, fn(&mut TextArea) -> bool);
impl DeleteTester {
    fn test(&self, before: (usize, usize), after: (usize, usize, &str, &str)) {
        let Self(buf_before, op) = *self;
        let (row, col) = before;

        let mut t = TextArea::with_value(buf_before);
        t.move_cursor(CursorMove::Jump(row as _, col as _));
        let modified = op(&mut t);

        let (row, col, buf_after, yank) = after;
        assert_eq!(t.text().as_str(), buf_after);
        assert_eq!(t.cursor2(), (row, col));
        assert_eq!(modified, buf_before != buf_after);
        assert_eq!(t.yank_text(), yank);

        if modified {
            t.undo();
            assert_eq!(t.text().as_str(), buf_before);
            // t.redo();
            // assert_eq!(t.text().as_str(), buf_after);
        } else {
            assert_no_undo_redo(&mut t, "");
        }
    }
}

#[test]
fn test_delete_newline() {
    let t = DeleteTester("a\nb\nc", |t| t.delete_newline());
    t.test((0, 0), (0, 0, t.0, ""));
    t.test((1, 0), (0, 1, "ab\nc", ""));
    t.test((2, 0), (1, 1, "a\nbc", ""));
}

#[test]
fn test_delete_char() {
    let t = DeleteTester("ab\nc", |t| t.delete_char());
    t.test((0, 0), (0, 0, t.0, ""));
    t.test((0, 1), (0, 0, "b\nc", ""));
    t.test((0, 2), (0, 1, "a\nc", ""));
    t.test((1, 0), (0, 2, "abc", ""));
}

#[test]
fn test_delete_next_char() {
    let t = DeleteTester("ab\nc", |t| t.delete_next_char());
    t.test((0, 0), (0, 0, "b\nc", ""));
    t.test((0, 1), (0, 1, "a\nc", ""));
    t.test((0, 2), (0, 2, "abc", ""));
    t.test((1, 1), (1, 1, t.0, ""));
}

#[test]
fn test_delete_line_by_end() {
    let t = DeleteTester("aaa bbb\nd", |t| t.delete_line_by_end());
    t.test((0, 0), (0, 0, "\nd", "aaa bbb"));
    t.test((0, 3), (0, 3, "aaa\nd", " bbb"));
    t.test((0, 6), (0, 6, "aaa bb\nd", "b"));
    t.test((0, 7), (0, 7, "aaa bbbd", "")); // Newline is not yanked
    t.test((1, 1), (1, 1, t.0, ""));
}

// #[test]
// fn test_delete_line_by_head() {
//     let t = DeleteTester(&["aaa bbb", "d"], |t| t.delete_line_by_head());
//     t.test((0, 0), (0, 0, t.0, ""));
//     t.test((0, 3), (0, 0, &[" bbb", "d"], "aaa"));
//     t.test((0, 7), (0, 0, &["", "d"], "aaa bbb"));
//     t.test((1, 0), (0, 7, &["aaa bbbd"], "")); // Newline is not yanked
// }

// #[test]
// fn test_delete_word() {
//     let t = DeleteTester(&["word  ことば 🐶", " x"], |t| t.delete_word());
//     t.test((0, 0), (0, 0, t.0, ""));
//     t.test((0, 2), (0, 0, &["rd  ことば 🐶", " x"], "wo"));
//     t.test((0, 4), (0, 0, &["  ことば 🐶", " x"], "word"));
//     t.test((0, 5), (0, 0, &[" ことば 🐶", " x"], "word "));
//     t.test((0, 6), (0, 0, &["ことば 🐶", " x"], "word  "));
//     t.test((0, 7), (0, 6, &["word  とば 🐶", " x"], "こ"));
//     t.test((0, 9), (0, 6, &["word   🐶", " x"], "ことば"));
//     t.test((0, 10), (0, 6, &["word  🐶", " x"], "ことば "));
//     t.test((0, 11), (0, 10, &["word  ことば ", " x"], "🐶"));
//     t.test((1, 0), (0, 11, &["word  ことば 🐶 x"], ""));
//     t.test((1, 1), (1, 0, &["word  ことば 🐶", "x"], " "));
//     t.test((1, 2), (1, 1, &["word  ことば 🐶", " "], "x"));
// }

// #[test]
// fn test_delete_next_word() {
//     let t = DeleteTester(&["word  ことば 🐶", " x"], |t| t.delete_next_word());
//     t.test((0, 0), (0, 0, &["  ことば 🐶", " x"], "word"));
//     t.test((0, 2), (0, 2, &["wo  ことば 🐶", " x"], "rd"));
//     t.test((0, 4), (0, 4, &["word 🐶", " x"], "  ことば"));
//     t.test((0, 5), (0, 5, &["word  🐶", " x"], " ことば"));
//     t.test((0, 6), (0, 6, &["word   🐶", " x"], "ことば"));
//     t.test((0, 9), (0, 9, &["word  ことば", " x"], " 🐶"));
//     t.test((0, 10), (0, 10, &["word  ことば ", " x"], "🐶"));
//     t.test((0, 11), (0, 11, &["word  ことば 🐶 x"], ""));
//     t.test((1, 0), (1, 0, &["word  ことば 🐶", ""], " x"));
//     t.test((1, 2), (1, 2, t.0, ""));
// }

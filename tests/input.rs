use ratatui_mergearea::{Input, Key, MergeArea};

// Sanity test for checking textarea does not crash against all combination of inputs
#[test]
fn test_input_all_combinations_sanity() {
    use Key::*;

    fn push_all_modifiers_combination(inputs: &mut Vec<Input>, key: Key) {
        for ctrl in [true, false] {
            for alt in [true, false] {
                for shift in [true, false] {
                    inputs.push(Input {
                        key,
                        ctrl,
                        alt,
                        shift,
                    });
                }
            }
        }
    }

    let mut inputs = vec![];

    for c in ' '..='~' {
        push_all_modifiers_combination(&mut inputs, Char(c));
    }
    for i in 0..=15 {
        push_all_modifiers_combination(&mut inputs, F(i));
    }
    for k in [
        Null,
        Char('あ'),
        Char('🐶'),
        Backspace,
        Enter,
        Left,
        Right,
        Up,
        Down,
        Tab,
        Delete,
        Home,
        End,
        PageUp,
        PageDown,
        Esc,
        MouseScrollDown,
        MouseScrollUp,
        Copy,
        Cut,
        Paste,
    ] {
        push_all_modifiers_combination(&mut inputs, k);
    }

    let mut t = MergeArea::with_value(["abc", "def", "ghi", "jkl", "mno", "pqr"].join("\n"));

    for input in inputs {
        t.input(input.clone());
        t.undo();
        t.redo();
        t.input(input);
        t.undo();
        t.redo();
    }
}

#[test]
fn test_insert_multi_code_unit_emoji() {
    let mut t = MergeArea::default();
    for c in "👨‍👩‍👧‍👦".chars() {
        let input = Input {
            key: Key::Char(c),
            ctrl: false,
            alt: false,
            shift: false,
        };
        assert!(t.input(input), "{c:?}");
    }
    assert_eq!(t.text().as_str(), "👨‍👩‍👧‍👦");
}

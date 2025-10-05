use std::cmp;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CharKind {
    Space,
    Punct,
    Other,
}

impl CharKind {
    fn new(c: char) -> Self {
        if c.is_whitespace() {
            Self::Space
        } else if c.is_ascii_punctuation() {
            Self::Punct
        } else {
            Self::Other
        }
    }
}

pub fn find_word_start_forward(text: &str, cursor: usize) -> Option<usize> {
    let mut it = text.chars().enumerate().skip(cursor);
    let mut prev = CharKind::new(it.next()?.1);
    for (col, c) in it {
        let cur = CharKind::new(c);
        if cur != CharKind::Space && prev != cur {
            return Some(col);
        }
        prev = cur;
    }
    None
}

pub fn find_word_exclusive_end_forward(line: &str, start_col: usize) -> Option<usize> {
    let mut it = line.chars().enumerate().skip(start_col);
    let mut prev = CharKind::new(it.next()?.1);
    for (col, c) in it {
        let cur = CharKind::new(c);
        if prev != CharKind::Space && prev != cur {
            return Some(col);
        }
        prev = cur;
    }
    None
}

pub fn find_word_inclusive_end_forward(text: &str, cursor: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();

    let mut pos = cursor;
    while pos < chars.len() && CharKind::new(chars[pos]) == CharKind::Space {
        pos += 1;
    }

    if pos >= chars.len() {
        return Some(chars.len());
    }

    let word_kind = CharKind::new(chars[pos]);

    while pos < chars.len() && CharKind::new(chars[pos]) == word_kind {
        pos += 1;
    }

    Some(pos.saturating_sub(1))
}

pub fn find_word_start_backward(line: &str, start_col: usize) -> Option<usize> {
    let idx = line
        .char_indices()
        .nth(start_col)
        .map(|(i, _)| i)
        .unwrap_or(line.len());
    let mut it = line[..idx].chars().rev().enumerate();
    let mut cur = CharKind::new(it.next()?.1);
    for (i, c) in it {
        let next = CharKind::new(c);
        if cur != CharKind::Space && next != cur {
            return Some(start_col - i);
        }
        cur = next;
    }
    (cur != CharKind::Space).then(|| 0)
}

pub fn find_word_start_backward_v2(text: &str, cursor: usize) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();

    let mut pos = cmp::min(cursor, chars.len().saturating_sub(1));
    while pos > 0 && CharKind::new(chars[pos]) == CharKind::Space {
        pos -= 1;
    }

    if pos == 0 {
        return Some(pos);
    }

    let word_kind = CharKind::new(chars[pos]);
    while pos > 0 && CharKind::new(chars[pos]) == word_kind {
        pos -= 1;
    }

    if pos == 0 {
        Some(pos)
    } else {
        Some(pos + 1)
    }
}

pub fn spaces(size: u8) -> &'static str {
    const SPACES: &str = "                                                                                                                                                                                                                                                                ";
    &SPACES[..size as usize]
}

pub fn num_digits(i: usize) -> u8 {
    f64::log10(i as f64) as u8 + 1
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
    pub offset: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize, offset: usize) -> Self {
        Self { row, col, offset }
    }
}

pub fn find_line_start(offset: usize, chars: &[char]) -> usize {
    let mut current_line_start = offset;
    while current_line_start > 0 && chars[current_line_start - 1] != '\n' {
        current_line_start -= 1;
    }
    current_line_start
}

pub fn find_line_end(offset: usize, chars: &[char]) -> usize {
    let mut line_end = offset;
    while line_end < chars.len() && chars[line_end] != '\n' {
        line_end += 1;
    }
    line_end - 1
}


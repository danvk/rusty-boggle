use crate::util;
use crate::trie;
use std::io;
use std::fmt;

const WORD_SCORES: [u32; 18] = [ 0, 0, 0, 1, 1, 2, 3, 5, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11 ];
pub const Q: usize = (('q' as i8) - ('a' as i8)) as usize;

pub struct Boggler {
    bd: [[usize; 4]; 4],
    runs: u32,
}

const NEIGHBORS_00: [(usize, usize); 3] = [(0, 1), (1, 0), (1, 1)];
const NEIGHBORS_01: [(usize, usize); 5] = [(0, 0), (1, 0), (1, 1), (1, 2), (0, 2)];
const NEIGHBORS_02: [(usize, usize); 5] = [(0, 1), (1, 1), (1, 2), (1, 3), (0, 3)];
const NEIGHBORS_03: [(usize, usize); 3] = [(0, 2), (1, 2), (1, 3)];

const NEIGHBORS_10: [(usize, usize); 5] = [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)];
const NEIGHBORS_11: [(usize, usize); 8] = [(0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2)];
const NEIGHBORS_12: [(usize, usize); 8] = [(0, 1), (1, 1), (2, 1), (0, 2), (2, 2), (0, 3), (1, 3), (2, 3)];
const NEIGHBORS_13: [(usize, usize); 5] = [(0, 2), (1, 2), (2, 2), (0, 3), (2, 3)];

const NEIGHBORS_20: [(usize, usize); 5] = [(1, 0), (3, 0), (1, 1), (2, 1), (3, 1)];
const NEIGHBORS_21: [(usize, usize); 8] = [(1, 0), (2, 0), (3, 0), (1, 1), (3, 1), (1, 2), (2, 2), (3, 2)];
const NEIGHBORS_22: [(usize, usize); 8] = [(1, 1), (2, 1), (3, 1), (1, 2), (3, 2), (1, 3), (2, 3), (3, 3)];
const NEIGHBORS_23: [(usize, usize); 5] = [(1, 2), (2, 2), (3, 2), (1, 3), (3, 3)];

const NEIGHBORS_30: [(usize, usize); 3] = [(2, 0), (2, 1), (3, 1)];
const NEIGHBORS_31: [(usize, usize); 5] = [(2, 0), (2, 1), (2, 2), (3, 0), (3, 2)];
const NEIGHBORS_32: [(usize, usize); 5] = [(2, 1), (2, 2), (2, 3), (3, 1), (3, 3)];
const NEIGHBORS_33: [(usize, usize); 3] = [(2, 2), (3, 2), (2, 3)];

// Are these all allocated statically or at runtime? How can I tell?
fn neighbors(x: usize, y: usize) -> &'static[(usize, usize)] {
    match (x, y) {
        (0, 0) => &NEIGHBORS_00,
        (0, 1) => &NEIGHBORS_01,
        (0, 2) => &NEIGHBORS_02,
        (0, 3) => &NEIGHBORS_03,
        (1, 0) => &NEIGHBORS_10,
        (1, 1) => &NEIGHBORS_11,
        (1, 2) => &NEIGHBORS_12,
        (1, 3) => &NEIGHBORS_13,
        (2, 0) => &NEIGHBORS_20,
        (2, 1) => &NEIGHBORS_21,
        (2, 2) => &NEIGHBORS_22,
        (2, 3) => &NEIGHBORS_23,
        (3, 0) => &NEIGHBORS_30,
        (3, 1) => &NEIGHBORS_31,
        (3, 2) => &NEIGHBORS_32,
        (3, 3) => &NEIGHBORS_33,
        (_, _) => panic!(),
    }
}

impl Boggler {
    pub fn new() -> Boggler {
        Boggler {
            bd: Default::default(),
            runs: 0,
        }
    }

    // TODO: method to bogglify & load

    pub fn set_cell(&mut self, x: usize, y: usize, i: usize) {
        self.bd[x][y] = i;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> usize {
        self.bd[x][y]
    }

    pub fn width(&self) -> usize {
        4
    }
    pub fn height(&self) -> usize {
        4
    }

    pub fn parse_board(&mut self, board: &str) -> Result<(), String> {
        let w = self.width();
        let h = self.height();
        if board.len() != w * h {
            return Err(String::from("Incorrect board length"));
        }

        for (i, c) in board.char_indices() {
            if c < 'a' || c > 'z' {
                return Err(format!("Invalid character: {}", c));
            }
            self.set_cell(i % w, i / w, trie::idx(c));
        }

        Ok(())
    }

    pub fn score(&mut self, dict: &mut trie::Trie) -> u32 {
        let mut score = 0;
        self.runs += 1;
        for x in 0..4 {
            for y in 0..4 {
                let c = self.get_cell(x, y);
                if let Some(d) = dict.descend_mut(c) {
                    score += self.do_dfs(x, y, 0, 0, d, &String::from(trie::idx_to_char(c)));
                }
            }
        }
        score
    }

    fn do_dfs(&self, x: usize, y: usize, len_in: usize, used_in: u32, t: &mut trie::Trie, wd_so_far: &str) -> u32 {
        let mut score = 0u32;
        let c = self.get_cell(x, y);
        let i = 4 * x + y;
        let used = used_in ^ 1 << i;
        let len = len_in + if c == Q { 2 } else { 1 };

        if t.is_word {
            if t.mark != self.runs {
                t.mark = self.runs;
                score += WORD_SCORES[len];
                if len >= 3 {
                    println!("{} {}", len, wd_so_far);
                }
            }
        }

        // XXX how efficient is it to do all this with iterators?
        for &(cx, cy) in neighbors(x, y) {
            let idx = 4 * cx + cy;
            if used & (1 << idx) == 0 {
                let cc = self.bd[cx][cy];
                if let Some(tc) = t.descend_mut(cc) {
                    let mut prefix = String::from(wd_so_far);
                    prefix.push(trie::idx_to_char(cc));
                    score += self.do_dfs(cx, cy, len, used, tc, &prefix);
                }
            }
        }

        score
    }
}

impl fmt::Display for Boggler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let w = self.width();
        let h = self.height();
        let mut result = String::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                result.push(trie::idx_to_char(self.get_cell(x, y)));
            }
        }
        write!(f, "{}", result)
    }
}

pub fn is_boggle_word(word: &str) -> bool {
    let len = word.len();
    if len < 3 || len > 17 {
        return false
    }
    let b = word.as_bytes();
    for (i, c) in b.iter().enumerate() {
        if *c == b'q' && b.get(i+1) != Some(&b'u') {
            return false;
        }
    }
    return true;
}

/// Replace "qu" with "q" in the word
pub fn bogglify_word(word: &str) -> String {
    if !word.contains("q") {
        return String::from(word);
    }
    let entries = word.chars().collect::<Vec<char>>();
    let len = entries.len();
    let mut i = 0;
    let mut result = String::with_capacity(len - 1);
    loop {
        let c = entries[i];
        result.push(c);
        i += if c == 'q' { 2 } else { 1 };
        if i >= len {
            break;
        }
    }
    result
}

pub fn load_dictionary(filename: &str) -> io::Result<trie::Trie> {
    let mut t = trie::Trie::new();
    let lines = util::read_lines(filename)?;
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(word_line) = line {
            let word = word_line.trim();
            if !word.is_empty() && is_boggle_word(word) {
                let boggle_word = bogglify_word(word);
                t.add_word_chars(boggle_word.chars());
            }
        }
    }
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_boggle_word() {
        assert!(!is_boggle_word(""));
        assert!(!is_boggle_word("f"));
        assert!(!is_boggle_word("fo"));
        assert!(is_boggle_word("foo"));
        assert!(is_boggle_word("quick"));
        assert!(!is_boggle_word("qick"));
        assert!(!is_boggle_word("extremelylongwordmaybgerman"));
        assert!(!is_boggle_word("suq"));
        assert!(is_boggle_word("suqu"));
    }

    #[test]
    fn test_bogglify_word() {
        assert_eq!(bogglify_word("food"), "food");
        assert_eq!(bogglify_word("quickly"), "qickly");
        assert_eq!(bogglify_word("suqu"), "suq");
    }

    #[test]
    fn test_parse_and_display() {
        let mut b = Boggler::new();
        b.parse_board("abcdefghijklmnop").unwrap();
        assert_eq!("abcdefghijklmnop", b.to_string());

        assert!(b.parse_board("abcdefghijklmno").is_err());
        assert!(b.parse_board("abcdefghijklmnopq").is_err());
    }

    #[test]
    fn test_set_cell() {
        let mut b = Boggler::new();
        b.parse_board("abcdefghijklmnop").unwrap();
        assert_eq!("abcdefghijklmnop", b.to_string());
        b.set_cell(1, 0, 0);
        b.set_cell(2, 0, 0);
        assert_eq!("aaadefghijklmnop", b.to_string());
    }
}

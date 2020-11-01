#[path = "./trie.rs"]
mod trie;
use std::fmt;

pub struct Boggler {
    dict: trie::Trie,
    bd: [[usize; 4]; 4],
}

impl Boggler {
    fn new(dict: trie::Trie) -> Boggler {
        Boggler {
            dict,
            bd: Default::default(),
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, i: usize) {
        self.bd[x][y] = i;
    }

    fn get_cell(&self, x: usize, y: usize) -> usize {
        self.bd[x][y]
    }

    fn width(&self) -> usize {
        4
    }
    fn height(&self) -> usize {
        4
    }
}

impl fmt::Display for Boggler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for y in 0..self.height() {
            for x in 0..self.width() {
                let letter = ('a' as usize) + self.get_cell(x, y);
                result = write!(f, "{}", letter);
            }
        }
        result
    }
}

pub fn is_boggle_word(word: &str) -> bool {
    let len = word.len();
    if len < 3 || len > 17 {
        return false
    }
    let mut last_was_q = false;
    for c in word.chars() {
        if last_was_q && c != 'u' {
            return false;
        }
        last_was_q = c == 'q';
    }
    return true;
}

/// Replace "qu" with "q" in the word
pub fn bogglify_word(word: &str) -> String {
    if !word.contains("q") {
        return String::from(word);
    }
    let entries: Vec<char> = word.chars().collect();
    let len = entries.len();
    let mut i = 0;
    let mut result = String::with_capacity(len - 1);
    loop {
        let c = entries[i];
        result.push(c);
        i +=  if c == 'q' { 2 } else { 1 };
        if i >= len {
            break;
        }
    }
    result
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
    }

    #[test]
    fn test_bogglify_word() {
        assert_eq!(bogglify_word("food"), "food");
        assert_eq!(bogglify_word("quickly"), "qickly");
    }
}

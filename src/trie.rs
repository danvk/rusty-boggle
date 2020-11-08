use super::util;
use std::io;

pub const NUM_LETTERS: usize = 26;

pub fn idx(letter: char) -> usize {
    if letter < 'a' || letter > 'z' {
        panic!("Invalid letter {}", letter)
    }
    return (letter as usize) - ('a' as usize);
}

pub fn idx_to_char(idx: usize) -> char {
    if idx > 26 {
        panic!("Invalid letter index {}", idx);
    }
    return (('a' as usize) + idx) as u8 as char;
}

pub struct Trie {
    pub is_word: bool,
    pub mark: u32,
    // XXX would this be simpler as Box<[Option<Trie>; NUM_LETTERS]?
    children: [Option<Box<Trie>>; NUM_LETTERS],
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            is_word: false,
            mark: 0,
            children: Default::default(),  // What's going on here?
        }
    }

    pub fn destroy(&mut self) {
        for child in self.children.iter_mut().flatten() {
            child.destroy()
        }
    }

    pub fn starts_word(&self, i: usize) -> bool {
        self.children.get(i).map_or(false, |c| c.is_some())
    }

    // XXX does the iterator here need to be mutable/consumed?
    pub fn add_word_chars(&mut self, mut chars: std::str::Chars) {
        if let Some(letter) = chars.next() {
            let i = idx(letter) as usize;

            if let Some(child) = self.descend_mut(i) {
                child.add_word_chars(chars);
            } else {
                let mut new_child = Trie::new();
                new_child.add_word_chars(chars);
                self.children[i] = Some(Box::from(new_child));
            }
        } else {
            self.is_word = true;
        }
    }

    pub fn add_word(&mut self, word: &str) {
        self.add_word_chars(word.chars());
    }

    pub fn is_word(&self, word: &str) -> bool {
        self.find_word(word).map_or(false, |c| c.is_word)
    }

    pub fn descend(&self, i: usize) -> Option<&Trie> {
        match self.children.get(i) {
            None => panic!("Invalid letter: {i}"),
            // This looks like the identity, but there's some implicit unboxing.
            // XXX is there a better way to do this?
            Some(c) => match c {
                Some(d) => Some(d),
                None => None,
            }
        }
    }

    pub fn descend_mut(&mut self, i: usize) -> Option<&mut Trie> {
        match self.children.get_mut(i) {
            None => panic!("Invalid letter: {i}"),
            // See comment in descend()
            Some(c) => match c {
                Some(d) => Some(d),
                None => None,
            }
        }
    }

    pub fn find_word_chars(&self, mut chars: std::str::Chars) -> Option<&Trie> {
        if let Some(letter) = chars.next() {
            let i = idx(letter) as usize;
            self.descend(i)?.find_word_chars(chars)
        } else {
            Some(self)
        }
    }

    pub fn find_word(&self, word: &str) -> Option<&Trie> {
        self.find_word_chars(word.chars())
    }

    pub fn from_file(filename: &str) -> io::Result<Trie> {
        let mut t = Trie::new();
        let lines = util::read_lines(filename)?;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word_line) = line {
                let word = word_line.trim();
                if !word.is_empty() {
                    t.add_word_chars(word.chars());
                }
            }
        }
        Ok(t)
    }

    pub fn size(&self) -> u32 {
        (if self.is_word { 1 } else { 0 })
            + self
                .children
                .iter()
                .flatten()
                .map(|n| n.size())
                .sum::<u32>()
    }

    pub fn num_nodes(&self) -> u32 {
        1 + self
            .children
            .iter()
            .flatten()
            .map(|n| n.num_nodes())
            .sum::<u32>()
    }
}

pub fn reverse_lookup(root: &Trie, node: &Trie) -> Option<String> {
    if root as *const _ == node as *const _ {
        return Some(String::from(""));
    } else {
        for i in 0..NUM_LETTERS {
            if let Some(child) = root.descend(i) {
                if let Some(rest) = reverse_lookup(child, node) {
                    let mut s = String::from(idx_to_char(i));
                    s.push_str(&rest);
                    return Some(s);
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_shallow_trie() {
        let mut t = Trie::new();
        t.add_word("a");
        t.add_word("b");

        assert!(t.starts_word(idx('a')));
        assert!(t.starts_word(idx('b')));
        assert!(!t.starts_word(idx('c')));
        assert_eq!(t.num_nodes(), 3);
        assert_eq!(t.size(), 2);
    }

    #[test]
    fn test_add_words() {
        let mut t = Trie::new();
        t.add_word("agriculture");
        t.add_word("culture");
        t.add_word("boggle");
        t.add_word("tea");
        t.add_word("sea");
        t.add_word("teapot");

        assert_eq!(t.size(), 6);

        assert!(t.is_word("agriculture"));
        assert!(t.is_word("culture"));
        assert!(t.is_word("boggle"));
        assert!(t.is_word("tea"));
        assert!(t.is_word("teapot"));
        assert!(!t.is_word("teap"));
        assert!(!t.is_word("random"));
        assert!(!t.is_word("cultur"));
    }

    #[test]
    fn test_marks() {
        let mut t = Trie::new();
        t.add_word("agriculture");
        t.add_word("culture");
        t.add_word("boggle");
        t.add_word("tea");
        t.add_word("sea");
        t.add_word("teapot");

        let mut wd = t.descend_mut(idx('t')).unwrap();
        wd = wd.descend_mut(idx('e')).unwrap();
        wd = wd.descend_mut(idx('a')).unwrap();
        assert_eq!(0, wd.mark);
        wd.mark = 12345;

        assert_eq!(t.find_word("tea").unwrap().mark, 12345);
    }

    #[test]
    fn test_reverse_lookup() {
        let mut t = Trie::new();
        t.add_word("agriculture");
        t.add_word("culture");
        t.add_word("boggle");
        t.add_word("tea");
        t.add_word("sea");
        t.add_word("teapot");

        let mut wd = t.descend(idx('t')).unwrap();
        wd = wd.descend(idx('e')).unwrap();
        wd = wd.descend(idx('a')).unwrap();

        assert_eq!(reverse_lookup(&t, wd), Some(String::from("tea")));
    }
}

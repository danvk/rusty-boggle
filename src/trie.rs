use std::str::Chars;

mod trie {
    pub const NUM_LETTERS: usize = 26;
    pub const Q: i8 = ('q' as i8) - ('a' as i8);

    fn idx(letter: char) -> i8 {
        if letter < 'a' || letter > 'z' {
            panic!("Invalid letter {letter}")
        }
        return (letter as i8) - ('a' as i8);
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
                children: Default::default(),
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

        pub fn add_word_chars(&mut self, mut chars: std::str::Chars) {
            if let Some(letter) = chars.next() {
                let i = idx(letter) as usize;
                if let Some(c) = self.children.get_mut(i) {
                    if let Some(child) = c {
                        child.add_word_chars(chars);
                    } else {
                        let mut new_child = Trie::new();
                        new_child.add_word_chars(chars);
                        self.children[i] = Some(Box::from(new_child));
                    }
                }
            } else {
                self.is_word = true;
            }
        }

        pub fn add_word(&mut self, word: &str) {
            self.add_word_chars(word.chars());
        }

        fn from_file(filename: &str) -> Trie {
            panic!("Not implemented")
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
}
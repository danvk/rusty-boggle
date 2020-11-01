const NUM_LETTERS: i32 = 26;
const Q: i8 = 'q' - 'a';
struct Trie {
  is_word: bool,
  mark: u32,
  children: [Option<Trie>; NUM_LETTERS],
}

impl Trie {
  fn new() -> Trie {
    Trie(false, 0, [None; NUM_LETTERS])
  }

  fn destroy(self) {
    for child in self.children {
      if let Some(c) = child {
        c.destroy()
      }
    }
  }

  fn starts_word(&self, i: i8) -> bool {
    self.children[i].is_some()
  }

  fn add_word(&mut self, word: str) {
    // TODO
  }

  fn from_file(filename: str) -> Trie {
    // TODO
  }

  fn size(&self) -> u32 {
    1 + self.children.flatMap(|n| n.size()).sum()
  }
}

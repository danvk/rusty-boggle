mod trie;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    let path = args.get(1).unwrap();
    match trie::Trie::from_file(path) {
        Ok(t) => println!("Loaded {} words into {} nodes from {}", t.size(), t.num_nodes(), path),
        Err(e) => println!("Failed to load {}: {}", path, e),
    }
}

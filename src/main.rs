mod trie;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected one argument, got {}: {:?}", args.len(), args);
    }

    let path = args.get(1).unwrap();
    if let Some(t) = trie::trie::Trie::from_file(path) {
        println!("Loaded {} words into {} nodes from {}", t.size(), t.num_nodes(), path);
    } else {
        println!("Failed to load {}", path);
    }
}

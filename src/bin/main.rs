extern crate trie_tree;

use trie_tree::trie_tree::TrieTree;

fn main() {
    let mut tree = TrieTree::new();
    tree.insert("hoge");
    tree.insert("hot");
    dbg!(tree);
}
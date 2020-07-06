extern crate trie_tree;

use trie_tree::trie_tree::TrieTree;

fn main() {
    let mut tree = TrieTree::new();
    tree.insert("A");
    tree.insert("to");
    tree.insert("tea");
    tree.insert("ted");
    tree.insert("ten");
    tree.insert("i");
    tree.insert("in");
    tree.insert("inn");
    dbg!(&tree);
}
extern crate trie_tree;

use trie_tree::trie_tree::TrieTree;

fn main() {
    let mut tree = TrieTree::new();
    let paths = vec!["/", "/static/*"];
    for key in &paths {
        tree.add(key);
    }
    dbg!(&tree);
    let queries = vec![
        "/static/index.html",
        "/static/style.css",
        "/static/index.js",
    ];
    for query in &queries {
        assert!(tree.find(query));
    }
}

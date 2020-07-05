/// Node of trie tree.
#[derive(Clone, Debug, Default)]
pub struct TrieTree {
    pub path: Option<String>,
    children: Vec<Box<TrieTree>>,
}

impl TrieTree {
    pub fn new() -> Self {
        Default::default()
    }

    fn new_child(path: &str) -> Self {
        Self {
            path: Some(path.to_string()),
            children: Vec::new(),
        }
    }

    /// Return how many common character paths of two `TrieTree` nodes have
    fn longest_common_prefix(&self, other: &TrieTree) -> usize {
        let mut pos = 0;
        if let (Some(path_self), Some(path_other)) = (&self.path, &other.path) {
            for (char_self, char_other) in path_self.chars().zip(path_other.chars()) {
                if char_self == char_other {
                    pos += 1;
                } else {
                    break;
                }
            }
        }
        pos
    }
}

#[cfg(test)]
mod tests {
    use crate::trie_tree::TrieTree;

    #[test]
    fn test_lcp() {
        let node_x = TrieTree {
            path: Some("abcde".to_string()),
            children: Vec::new(),
        };
        let node_y = TrieTree {
            path: Some("abcfghi".to_string()),
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix(&node_y), 3);
    }
}

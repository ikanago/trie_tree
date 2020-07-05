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

    /// Return how many common character path of `TrieTree` nodes and an arugument have.
    fn longest_common_prefix(&self, other: &str) -> usize {
        let mut pos = 0;
        if let Some(path_self) = &self.path {
            for (char_self, char_other) in path_self.chars().zip(other.chars()) {
                if char_self == char_other {
                    pos += 1;
                } else {
                    break;
                }
            }
        }
        pos
    }

    pub fn insert(&mut self, new_path: &str) {
        let lcp = self.longest_common_prefix(new_path);

        dbg!(lcp);
        if lcp == 0 {
            if self.children.len() == 0 {
                self.children.push(Box::new(TrieTree::new_child(new_path)));
            } else {
                for child in &mut self.children {
                    child.insert(new_path);
                }
            }
            return;
        }
        // If length of longest common prefix is not 0, `self.path` cannot be `None`.
        let path = self.path.clone().unwrap();
        // if `new_path` is prefix of `self.path`, it is ignored.
        if path.len() > lcp {
            let common_prefix = &path[..lcp];
            let path_remaining = &path[lcp..];
            let new_path_remaining = &new_path[lcp..];

            let mut new_child = self.clone();
            new_child.path = Some(path_remaining.to_string());
            self.path = Some(common_prefix.to_string());
            self.children = Vec::new();
            self.children.push(Box::new(new_child));
            self.children
                .push(Box::new(TrieTree::new_child(new_path_remaining)))
        }
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
        assert_eq!(node_x.longest_common_prefix("abchoge"), 3);
    }
}

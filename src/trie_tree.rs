/// Node of trie tree.
#[derive(Clone, Debug, Default)]
pub struct TrieTree {
    pub path: String,
    children: Vec<Box<TrieTree>>,
}

impl TrieTree {
    pub fn new() -> Self {
        Default::default()
    }

    fn new_child(path: &str) -> Self {
        Self {
            path: path.to_string(),
            children: Vec::new(),
        }
    }

    /// Return how many common character path of `TrieTree` nodes and an arugument have.
    fn longest_common_prefix(&self, other: &str) -> usize {
        let mut pos = 0;
        for (char_self, char_other) in self.path.chars().zip(other.chars()) {
            if char_self == char_other {
                pos += 1;
            } else {
                break;
            }
        }
        pos
    }

    pub fn insert(&mut self, new_path: &str) {
        // For the first time to insert node to root.
        if self.path.len() == 0 && self.children.len() == 0 {
            self.children.push(Box::new(TrieTree::new_child(new_path)));
            return;
        }
        if self.path == new_path {
            return;
        }

        let lcp = self.longest_common_prefix(new_path);
        // If length of longest common prefix is not 0, `self.path` cannot be `None`.
        let path = self.path.clone();
        if path.len() > lcp {
            let common_prefix = &path[..lcp];
            let path_remaining = &path[lcp..];
            let new_path_remaining = &new_path[lcp..];

            let mut new_child = self.clone();
            new_child.path = path_remaining.to_string();
            self.path = common_prefix.to_string();
            self.children = vec![
                Box::new(new_child),
                Box::new(TrieTree::new_child(new_path_remaining)),
            ]
        } else {
            let new_path_remaining = &new_path[lcp..];
            let mut is_inserted = false;
            for child in &mut self.children {
                match (*child).path.chars().next() {
                    // Because more than 2 children node do not have same prefix,
                    // just check first character of key for each child.
                    Some(first_char)
                        if first_char == new_path_remaining.chars().next().unwrap() =>
                    {
                        is_inserted = true;
                        child.insert(new_path_remaining);
                        break;
                    }
                    _ => continue,
                }
            }
            if !is_inserted {
                self.children
                    .push(Box::new(TrieTree::new_child(new_path_remaining)));
            }
        }
    }

    pub fn find(&self, key: &str) -> bool {
        if key.len() == 0 {
            return false;
        }
        let lcp = self.longest_common_prefix(key);
        let key_remaining = &key[lcp..];
        if key_remaining.len() == 0 {
            return true;
        }

        for child in &self.children {
            match (*child).path.chars().next() {
                // Because more than 2 children node do not have same prefix,
                // just check first character of key for each child.
                Some(first_char) if first_char == key_remaining.chars().next().unwrap() => {
                    return child.find(key_remaining);
                }
                _ => continue,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::trie_tree::TrieTree;

    #[test]
    fn test_lcp() {
        let node_x = TrieTree {
            path: "abcde".to_string(),
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix("abchoge"), 3);
    }

    #[test]
    fn test_lcp_root() {
        let node_x = TrieTree {
            path: "".to_string(),
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix("abchoge"), 0);
    }

    #[test]
    fn test_find() {
        let mut tree = TrieTree::new();
        let keys = vec!["/", "to", "tea", "ted", "ten", "i", "in", "inn"];
        for key in &keys {
            tree.insert(key);
        }
        for key in keys {
            assert!(tree.find(key));
        }
    }

    // Generate random alphanumeric string.
    fn random_string() -> String {
        extern crate rand;
        use rand::distributions::Alphanumeric;
        use rand::random;
        use rand::Rng;
        let length = random::<usize>() % 500 + 1;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect::<String>()
    }

    #[test]
    fn test_find_random() {
        let mut tree = TrieTree::new();
        let count = 1000;
        let keys = (0..count).map(|_| random_string()).collect::<Vec<String>>();
        for key in &keys {
            tree.insert(key);
        }
        for key in keys {
            assert!(tree.find(&key));
        }
    }
}

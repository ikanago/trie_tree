/// Check if the path has wild card at the end of the path.
fn includes_wildcard(path: &str) -> bool {
    path.ends_with("/*")
}

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
        let mut child = Self {
            path: path.to_string(),
            children: Vec::new(),
        };
        dbg!(&child);
        if includes_wildcard(path) && !path.starts_with('*') {
            child.split_wildcard();
        }
        child
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

    pub fn add(&mut self, new_path: &str) {
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
        // For example, `self.path` is "static" and longest common prefix is "stat".
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
            // When longest common prefix is exactly the same as `self.path`.
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
                        child.add(new_path_remaining);
                        break;
                    }
                    _ => continue,
                }
            }
            // If there is no child to match new path, just insert it.
            if !is_inserted {
                self.children
                    .push(Box::new(TrieTree::new_child(new_path_remaining)));
            }
        }
    }

    fn split_wildcard(&mut self) {
        if includes_wildcard(&self.path) {
            self.path = self.path.trim_end_matches('*').to_string();
            self.children.push(Box::new(Self {
                path: "*".to_string(),
                children: Vec::new(),
            }));
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
            if &child.path == "*" {
                return true;
            }
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
            tree.add(key);
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
            tree.add(key);
        }
        for key in keys {
            assert!(tree.find(&key));
        }
    }

    #[test]
    fn test_find_with_wildcard() {
        let mut tree = TrieTree::new();
        let paths = vec!["/", "/index.html", "/static/*"];
        for key in &paths {
            tree.add(key);
        }
        let queries = vec![
            "/",
            "/index.html",
            "/static/index.html",
            "/static/style.css",
            "/static/index.js",
        ];
        for query in &queries {
            assert!(tree.find(query));
        }
    }
}

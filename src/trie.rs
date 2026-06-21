use std::collections::{BTreeMap, VecDeque};

type NodeId = usize;

#[derive(Debug, Clone)]
struct TrieNode {
    children: BTreeMap<char, NodeId>,
    is_end: bool,
    suffix_link: NodeId,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            is_end: false,
            suffix_link: 0,
        }
    }

    pub fn add_child(&mut self, character: char, child_id: NodeId) {
        self.children.insert(character, child_id);
    }

    pub fn set_as_end(&mut self) {
        self.is_end = true
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    trie_nodes: Vec<TrieNode>,
}

impl Trie {
    fn add_new_node(&mut self, parent_id: NodeId, child_character: char) -> NodeId {
        self.trie_nodes.push(TrieNode::new());
        let new_id = self.trie_nodes.len() - 1;
        let parent = self.get_mut_node(parent_id);
        parent.add_child(child_character, new_id);
        new_id
    }

    fn get_mut_node(&mut self, id: NodeId) -> &mut TrieNode {
        self.trie_nodes
            .get_mut(id)
            .expect("NodeId must be on Trie.")
    }

    fn get_node(&self, id: NodeId) -> &TrieNode {
        self.trie_nodes.get(id).expect("NodeID must be on Trie.")
    }

    fn add_keyword(&mut self, keyword: &str) {
        let mut node_id: NodeId = 0;
        for character in keyword.chars() {
            let node = self.get_mut_node(node_id);
            node_id = match node.children.get(&character) {
                Some(child) => *child,
                None => {
                    let new_id = self.add_new_node(node_id, character);
                    new_id
                }
            }
        }
        self.get_mut_node(node_id).set_as_end();
    }

    fn build_suffix_links(&mut self) {
        let mut deque = VecDeque::from(vec![0]);
        while !deque.is_empty() {
            let this_id = deque.pop_front().unwrap();
            let children: Vec<(char, NodeId)> = self
                .get_node(this_id)
                .children
                .iter()
                .map(|(c, id)| (*c, *id))
                .collect();
            let suffix_id = self.get_node(this_id).suffix_link;
            for (character, id) in children {
                deque.push_back(id);
                if this_id == 0 {
                    continue;
                };
                let mut suffix_set = false;
                let mut this_suffix_id = suffix_id;
                while !suffix_set {
                    let suffix_node = self.get_node(this_suffix_id);
                    if let Some(next_suffix) = suffix_node.children.get(&character) {
                        self.get_mut_node(id).suffix_link = *next_suffix;
                        suffix_set = true
                    } else if this_suffix_id == 0 {
                        suffix_set = true
                    } else {
                        this_suffix_id = suffix_node.suffix_link;
                    }
                }
            }
        }
    }

    fn follow(&self, path: &str) -> Option<NodeId> {
        let mut id = 0;
        for character in path.chars() {
            id = *self.get_node(id).children.get(&character)?;
        }
        Some(id)
    }

    pub fn build_trie(keywords: &[&str]) -> Self {
        let mut trie = Self::default();
        for word in keywords {
            trie.add_keyword(word);
        }
        trie.build_suffix_links();
        trie
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            trie_nodes: vec![TrieNode::new()],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn traverse_trie_in_bfs(trie: Trie) -> String {
        let mut string = String::from("");
        let mut queue: VecDeque<NodeId> = VecDeque::from(vec![0]);
        while !queue.is_empty() {
            if let Some(id) = queue.pop_front() {
                let node = trie.get_node(id);
                for (character, new_id) in &node.children {
                    string.push(*character);
                    queue.push_back(*new_id)
                }
            }
        }
        string.to_string()
    }

    #[test]
    fn add_keyword_builds_trie() {
        let k1 = "abcd";
        let keywords = [k1];
        let trie = Trie::build_trie(&keywords);
        let res = traverse_trie_in_bfs(trie.clone());
        assert_eq!(res, "abcd");
    }

    #[test]
    fn add_2_keyword_builds_trie() {
        let k1 = "abcd";
        let k2 = "efgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let res = traverse_trie_in_bfs(trie.clone());
        assert_eq!(res, "aebfcgdh");
    }

    #[test]
    fn add_2_keyword_with_same_first_character_builds_trie() {
        let k1 = "abcd";
        let k2 = "afgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let res = traverse_trie_in_bfs(trie.clone());
        assert_eq!(res, "abfcgdh");
    }

    #[test]
    fn trie_shares_common_prefix_between_keywords() {
        let k1 = "abcd";
        let k2 = "afgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow("a").expect("a should exist.");
        let node = trie.get_node(id);
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn trie_does_not_duplicate_existing_path() {
        let k1 = "abcd";
        let k2 = "afgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow("a").expect("a should exist.");
        let node = trie.get_node(id);
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn final_letter_is_end_true() {
        let k1 = "abcd";
        let keywords = [k1];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow(k1).expect("path should exist.");
        let node = trie.get_node(id);
        assert!(node.is_end);
    }

    #[test]
    fn mid_trie_keyword_end_is_end() {
        let k1 = "abcd";
        let k2 = "ab";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow(k2).expect("path should exist.");
        let node = trie.get_node(id);
        assert!(node.is_end);
    }

    #[test]
    fn only_end_words_is_end() {
        let k1 = "abcd";
        let k2 = "ab";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let non_end_paths = ["a", "abc"];
        for path in non_end_paths {
            let id = trie.follow(path).expect("path should exist.");
            let node = trie.get_node(id);
            assert!(!node.is_end);
        }
    }

    #[test]
    fn end_on_multiple_branches_is_end() {
        let k1 = "abcd";
        let k2 = "efgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        for path in keywords {
            let id = trie.follow(path).expect("path should exist.");
            let node = trie.get_node(id);
            assert!(node.is_end);
        }
    }

    #[test]
    fn all_suffix_links_on_single_keyword_point_to_root() {
        let k1 = "abcd";
        let keywords = [k1];
        let trie = Trie::build_trie(&keywords);
        let prefixes: Vec<String> = (1..=k1.len()).map(|i| k1[..i].to_string()).collect();
        for path in prefixes {
            let id = trie.follow(&path).expect("path should exist.");
            let node = trie.get_node(id);
            assert_eq!(node.suffix_link, 0);
        }
    }

    #[test]
    fn suffix_link_on_letter_in_root_points_to_node() {
        let k1 = "abcad";
        let keywords = [k1];
        let trie = Trie::build_trie(&keywords);
        let path = "abca";
        let id = trie.follow(&path).expect("path should exist.");
        let node = trie.get_node(id);
        assert_eq!(node.suffix_link, 1);
    }

    #[test]
    fn suffix_link_points_to_another_branch() {
        let k1 = "abced";
        let k2 = "efgh"; 
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let path = "abce";
        let id = trie.follow(&path).expect("path should exist.");
        let node = trie.get_node(id);
        let obs_suffix = node.suffix_link;
        let exp_path = "e";
        let exp_id = trie.follow(&exp_path).expect("path should exist.");
        assert_eq!(obs_suffix, exp_id);
    }

    #[test]
    fn multi_char_suffix_link_points_to_another_branch() {
        let k1 = "abcefd";
        let k2 = "efgh"; 
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let path = "abcef";
        let id = trie.follow(&path).expect("path should exist.");
        let node = trie.get_node(id);
        let obs_suffix = node.suffix_link;
        let exp_path = "ef";
        let exp_id = trie.follow(&exp_path).expect("path should exist.");
        assert_eq!(obs_suffix, exp_id);
    }

    #[test]
    fn multi_hop_suffix_fallback() {
        let k1 = "ate";
        let k2 = "coats"; 
        let k3 = "ta";
        let keywords = [k1, k2, k3];
        let trie = Trie::build_trie(&keywords);
        let path = "coat";
        let id = trie.follow(&path).expect("path should exist.");
        let node = trie.get_node(id);
        let obs_suffix = node.suffix_link;
        let suffix_1_path = "at";
        let suffix_1_id = trie.follow(&suffix_1_path).expect("path should exist.");
        let suffix_1_node = trie.get_node(suffix_1_id);
        assert_eq!(obs_suffix, suffix_1_id);
        let suffix_1_suffix = suffix_1_node.suffix_link;
        let suffix_2_path = "t";
        let suffix_2_id = trie.follow(&suffix_2_path).expect("path should exist.");
        assert_eq!(suffix_1_suffix, suffix_2_id);
    }

    #[test]
    fn new_keyword_suffix_link_to_root() {
        let k1 = "abcd";
        let k2 = "e"; 
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let path = "e";
        let id = trie.follow(&path).expect("path should exist.");
        let node = trie.get_node(id);
        let obs_suffix = node.suffix_link;
        assert_eq!(obs_suffix, 0);
    }
}

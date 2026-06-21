use std::collections::BTreeMap;

type NodeId = usize;

#[derive(Debug, Clone)]
struct TrieNode {
    children: BTreeMap<char, NodeId>,
    pub is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            is_end: false
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
        let parent = self.get_mut_node(parent_id).expect("Parent ID should have bbeen on Trie.");
        parent.add_child(child_character, new_id);
        new_id
    }

    fn get_mut_node(&mut self, id: NodeId) -> Option<&mut TrieNode> {
        self.trie_nodes.get_mut(id)
    }

    fn get_node(&self, id: NodeId) -> Option<&TrieNode> {
        self.trie_nodes.get(id)
    }

    fn add_keyword(&mut self, keyword: &str) {
        let mut node_id: NodeId = 0;
        for character in keyword.chars() {
            let node = self.get_mut_node(node_id).expect("There should be a node here.");
            node_id = match node.children.get(&character) {
                Some(child) => *child,
                None => {
                    let new_id = self.add_new_node(node_id, character);
                    new_id
                }
            }
        } 
        self.get_mut_node(node_id).expect("There should be a node here.").set_as_end();
    }

    fn follow(&self, path: &str) -> Option<NodeId> {
        let mut id = 0;
        for character in path.chars() {
            id = *self.get_node(id)?.children.get(&character)?;
        }
        Some(id)
    }

    pub fn build_trie(keywords: &[&str]) -> Self {
        let mut trie = Self::default();
        for word in keywords {
            trie.add_keyword(word);
        }
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
                let node = trie.get_node(id).unwrap();
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
        let node = trie.get_node(id).unwrap();
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn trie_does_not_duplicate_existing_path() {
        let k1 = "abcd";
        let k2 = "afgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow("a").expect("a should exist.");
        let node = trie.get_node(id).unwrap();
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn final_letter_is_end_true () {
        let k1 = "abcd";
        let keywords = [k1];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow(k1).expect("path should exist.");
        let node = trie.get_node(id).unwrap();
        assert!(node.is_end);
    }

    #[test]
    fn mid_trie_keyword_end_is_end() {
        let k1 = "abcd";
        let k2 = "ab";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow(k2).expect("path should exist.");
        let node = trie.get_node(id).unwrap();
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
            let node = trie.get_node(id).unwrap();
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
            let node = trie.get_node(id).unwrap();
            assert!(node.is_end);
        }
    }
}

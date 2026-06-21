use std::collections::BTreeMap;

type NodeId = usize;

#[derive(Debug, Clone)]
struct TrieNode {
    children: BTreeMap<char, NodeId>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
        }
    }

    pub fn add_child(&mut self, character: char, child_id: NodeId) {
        self.children.insert(character, child_id);
    }
}

#[derive(Debug, Clone)]
struct Trie {
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

    pub fn add_keyword(&mut self, keyword: &String) {
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
        let keyword = "abcd".to_string(); 
        let mut trie = Trie::default();
        trie.add_keyword(&keyword);
        let res = traverse_trie_in_bfs(trie.clone());
        assert_eq!(res, "abcd");
    }

    #[test]
    fn add_2_keyword_builds_trie() {
        let k1 = "abcd".to_string(); 
        let k2 = "efgh".to_string();
        let mut trie = Trie::default();
        trie.add_keyword(&k1);
        trie.add_keyword(&k2);
        let res = traverse_trie_in_bfs(trie.clone());
        assert_eq!(res, "aebfcgdh");
    }

    #[test]
    fn add_2_keyword_with_same_first_character_builds_trie() {
        let k1 = "abcd".to_string(); 
        let k2 = "afgh".to_string();
        let mut trie = Trie::default();
        trie.add_keyword(&k1);
        trie.add_keyword(&k2);
        let res = traverse_trie_in_bfs(trie.clone());
        assert_eq!(res, "abfcgdh");
    }
}

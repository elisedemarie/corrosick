use std::collections::{HashMap, VecDeque};

pub type NodeId = usize;

#[derive(Debug, Clone, Default)]
struct TrieNode {
    children: HashMap<char, NodeId>,
    end_length: Option<usize>,
}

impl TrieNode {
    fn add_child(&mut self, character: char, child_id: NodeId) {
        self.children.insert(character, child_id);
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    trie_nodes: Vec<TrieNode>,
    suffix_links: Vec<NodeId>,
    output_links: Vec<Option<NodeId>>,
}

impl Trie {
    #[cfg(test)]
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

    fn get_node(&self, node_id: NodeId) -> &TrieNode {
        self.trie_nodes.get(node_id).expect("Should be node.")
    }

    pub fn character_at_node(&self, node_id: NodeId, character: char) -> Option<&NodeId> {
        self.get_node(node_id).children.get(&character)
    }

    pub fn get_suffix(&self, node_id: NodeId) -> NodeId {
        self.suffix_links
            .get(node_id)
            .copied()
            .expect("Should be node on trie.")
    }

    pub fn get_end(&self, node_id: NodeId) -> Option<usize> {
        self.get_node(node_id).end_length
    }

    pub fn get_output(&self, node_id: NodeId) -> Option<NodeId> {
        self.output_links
            .get(node_id)
            .copied()
            .expect("Should be node on trie.")
    }

    fn add_new_node(&mut self, parent_id: NodeId, child_character: char) -> NodeId {
        self.trie_nodes.push(TrieNode::default());
        self.suffix_links.push(0);
        self.output_links.push(None);
        let new_id = self.trie_nodes.len() - 1;
        let parent = &mut self.trie_nodes[parent_id];
        parent.add_child(child_character, new_id);
        new_id
    }

    fn add_keyword(&mut self, keyword: &str) {
        if keyword.is_empty() {
            return;
        }
        let mut node_id: NodeId = 0;
        for character in keyword.chars() {
            node_id = match self.trie_nodes[node_id].children.get(&character).copied() {
                Some(child) => child,
                None => self.add_new_node(node_id, character),
            }
        }
        self.trie_nodes[node_id].end_length = Some(keyword.len());
    }

    fn build_suffix_links(&mut self) {
        let Self {
            trie_nodes,
            suffix_links,
            output_links,
        } = self;
        let mut deque = VecDeque::from([0usize]);
        while let Some(this_id) = deque.pop_front() {
            let suffix_id = suffix_links[this_id];
            let suffix_node = &trie_nodes[suffix_id];
            output_links[this_id] = suffix_node
                .end_length
                .map(|_| suffix_id)
                .or_else(|| output_links[suffix_id]);
            for (&character, &id) in &trie_nodes[this_id].children {
                deque.push_back(id);
                if this_id == 0 {
                    continue;
                };
                let mut this_suffix_id = suffix_id;
                loop {
                    let suffix_node = &trie_nodes[this_suffix_id];
                    if let Some(next_suffix) = suffix_node.children.get(&character) {
                        suffix_links[id] = *next_suffix;
                        break;
                    } else if this_suffix_id == 0 {
                        break;
                    } else {
                        this_suffix_id = suffix_links[this_suffix_id];
                    }
                }
            }
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            trie_nodes: vec![TrieNode::default()],
            suffix_links: vec![0],
            output_links: vec![None],
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
                let mut children: Vec<_>= node.children.iter().collect();
                children.sort_by_key(|(c, _)| *c);
                for (character, new_id) in children {
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
        assert!(node.end_length.is_some());
    }

    #[test]
    fn mid_trie_keyword_end_is_end() {
        let k1 = "abcd";
        let k2 = "ab";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let id = trie.follow(k2).expect("path should exist.");
        let node = trie.get_node(id);
        assert!(node.end_length.is_some());
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
            assert!(node.end_length.is_none());
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
            assert!(node.end_length.is_some());
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
            assert_eq!(trie.get_suffix(id), 0);
        }
    }

    #[test]
    fn suffix_link_on_letter_in_root_points_to_node() {
        let k1 = "abcad";
        let keywords = [k1];
        let trie = Trie::build_trie(&keywords);
        let path = "abca";
        let id = trie.follow(path).expect("path should exist.");
        assert_eq!(trie.get_suffix(id), 1);
    }

    #[test]
    fn suffix_link_points_to_another_branch() {
        let k1 = "abced";
        let k2 = "efgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let path = "abce";
        let id = trie.follow(path).expect("path should exist.");
        let obs_suffix = trie.get_suffix(id);
        let exp_path = "e";
        let exp_id = trie.follow(exp_path).expect("path should exist.");
        assert_eq!(obs_suffix, exp_id);
    }

    #[test]
    fn multi_char_suffix_link_points_to_another_branch() {
        let k1 = "abcefd";
        let k2 = "efgh";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let path = "abcef";
        let id = trie.follow(path).expect("path should exist.");
        let obs_suffix = trie.get_suffix(id);
        let exp_path = "ef";
        let exp_id = trie.follow(exp_path).expect("path should exist.");
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
        let id = trie.follow(path).expect("path should exist.");
        let obs_suffix = trie.get_suffix(id);
        let suffix_1_path = "at";
        let suffix_1_id = trie.follow(suffix_1_path).expect("path should exist.");
        assert_eq!(obs_suffix, suffix_1_id);
        let suffix_1_suffix = trie.get_suffix(suffix_1_id);
        let suffix_2_path = "t";
        let suffix_2_id = trie.follow(suffix_2_path).expect("path should exist.");
        assert_eq!(suffix_1_suffix, suffix_2_id);
    }

    #[test]
    fn new_keyword_suffix_link_to_root() {
        let k1 = "abcd";
        let k2 = "e";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let path = "e";
        let id = trie.follow(path).expect("path should exist.");
        assert_eq!(trie.get_suffix(id), 0);
    }

    #[test]
    fn keyword_containing_keyword_has_output_node() {
        let k1 = "abcd";
        let k2 = "bc";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let path = "abc";
        let id = trie.follow(path).expect("path should exist.");
        let exp_path = "bc";
        let exp_id = trie.follow(exp_path).expect("path should exist.");
        assert_eq!(trie.get_output(id).unwrap(), exp_id)
    }

    #[test]
    fn output_links_are_none_for_single_keyword() {
        let k1 = "abcd";
        let keywords = [k1];
        let trie = Trie::build_trie(&keywords);
        let prefixes: Vec<String> = (1..=k1.len()).map(|i| k1[..i].to_string()).collect();
        for path in prefixes {
            let id = trie.follow(&path).expect("path should exist.");
            assert_eq!(trie.get_output(id), None);
        }
    }

    #[test]
    fn output_link_is_inherited() {
        let k1 = "c";
        let k2 = "bcx";
        let k3 = "abc";
        let keywords = [k1, k2, k3];
        let trie = Trie::build_trie(&keywords);
        let bc_id = trie.follow("bc").expect("path should exist.");
        assert!(trie.get_node(bc_id).end_length.is_none());
        let abc_id = trie.follow("abc").expect("path should exist");
        let exp_id = trie.follow("c").expect("path should exist");
        assert_eq!(trie.get_output(abc_id).unwrap(), exp_id)
    }

    #[test]
    fn multiple_kwds_picked_up_in_output_links() {
        let k1 = "abcd";
        let k2 = "bcd";
        let k3 = "cd";
        let keywords = [k1, k2, k3];
        let trie = Trie::build_trie(&keywords);
        let k1_id = trie.follow(k1).unwrap();
        let k1_output = trie.get_output(k1_id).unwrap();
        let k2_id = trie.follow(k2).unwrap();
        assert_eq!(k1_output, k2_id);
        let k2_output = trie.get_output(k2_id).unwrap();
        let k3_id = trie.follow(k3).unwrap();
        assert_eq!(k2_output, k3_id);
    }

    #[test]
    fn public_getters_match_trie_node_fields() {
        let k1 = "abcd";
        let k2 = "cd";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let a_id = trie.follow("a").unwrap();
        assert_eq!(trie.character_at_node(0, 'a'), Some(&a_id));
        assert_eq!(trie.character_at_node(0, 'z'), None);
        let abcd_id = trie.follow(k1).unwrap();
        assert_eq!(trie.get_suffix(abcd_id), trie.follow(k2).unwrap());
        assert_eq!(trie.get_end(abcd_id), Some(4));
        assert_eq!(trie.get_end(a_id), None);
        assert_eq!(trie.get_output(abcd_id), trie.follow(k2));
    }

    #[test]
    fn subset_keyword_pucks_up_output() {
        let k1 = "abcd";
        let k2 = "cd";
        let keywords = [k1, k2];
        let trie = Trie::build_trie(&keywords);
        let k1_id = trie.follow(k1).unwrap();
        let k1_output = trie.get_output(k1_id).unwrap();
        let k2_id = trie.follow(k2).unwrap();
        let k2_output = trie.get_output(k2_id);
        assert_eq!(k1_output, k2_id);
        assert!(k2_output.is_none());
    }
}

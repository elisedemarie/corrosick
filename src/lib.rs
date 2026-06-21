use crate::trie::Trie;

mod trie;

pub struct Match {
    trie: Trie,
}

impl Match {
    pub fn new(keywords: &[&str]) -> Self {
        let trie = Trie::build_trie(&keywords);
        Self { trie }
    }
}

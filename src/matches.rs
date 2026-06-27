use std::str::CharIndices;

use crate::trie::{NodeId, Trie};

pub struct Match<'a> {
    pub start: usize,
    pub text: &'a str,
}

impl<'a> Match<'a> {
    pub fn from_match_end(corpus: &'a str, length: usize, end: usize, character: char) -> Self {
        let end = end + character.len_utf8() - 1;
        let start = end - (length - 1);
        let text = &corpus[start..=end];
        Self { start, text }
    }
}

pub struct Matches<'a, 'b> {
    corpus: &'a str,
    trie: &'b Trie,
    chars: CharIndices<'a>,
    pointer: NodeId,
    pending_output: Option<(NodeId, usize, char)>,
}

impl<'a, 'b> Matches<'a, 'b> {
    pub fn new(corpus: &'a str, trie: &'b Trie) -> Self {
        Self {
            corpus,
            trie,
            chars: corpus.char_indices(),
            pointer: 0,
            pending_output: None,
        }
    }

    fn get_outputs(
        &self,
        pointer: NodeId,
        text_pointer: usize,
        character: char,
    ) -> Option<Match<'a>> {
        let length = self.trie.check_end(pointer)?;
        Some(Match::from_match_end(
            self.corpus,
            length,
            text_pointer,
            character,
        ))
    }

    fn exhaust_pending(&mut self) -> Option<Match<'a>> {
        let (output_id, text_pointer, character) = self.pending_output.take()?;
        let next_output = self.trie.get_output(output_id)?;
        self.pending_output = Some((next_output, text_pointer, character));
        self.get_outputs(next_output, text_pointer, character)
    }
}

impl<'a, 'b> Iterator for Matches<'a, 'b> {
    type Item = Match<'a>;
    fn next(&mut self) -> Option<Match<'a>> {
        if let found_match @ Some(_) = self.exhaust_pending() {
            return found_match;
        }
        let mut found_match = None;
        let mut pointer = self.pointer;
        while let Some((text_pointer, character)) = self.chars.next() {
            loop {
                if let Some(node_id) = self.trie.character_at_node(pointer, character) {
                    pointer = *node_id;
                    break;
                }
                if pointer == 0 {
                    break;
                };
                pointer = self.trie.get_suffix(pointer);
            }
            if self.trie.get_output(pointer).is_some() {
                self.pending_output = Some((pointer, text_pointer, character));
            }
            if let Some(length) = self.trie.check_end(pointer) {
                found_match = Some(Match::from_match_end(
                    self.corpus,
                    length,
                    text_pointer,
                    character,
                ));
                break;
            }
            if let output_match @ Some(_) = self.exhaust_pending() {
                found_match = output_match;
                break;
            }
        }
        self.pointer = pointer;
        found_match
    }
}

#[cfg(test)]
mod tests {
    use super::Matches;
    use crate::trie::Trie;

    #[test]
    fn output_link_match_is_yielded_after_direct_match() {
        let trie = Trie::build_trie(&["abcd", "cd"]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        assert_eq!(res, vec!["abcd", "cd"])
    }

    #[test]
    fn output_link_chain_is_fully_exhausted() {
        let trie = Trie::build_trie(&["abcd", "bcd", "cd"]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        assert_eq!(res, vec!["abcd", "bcd", "cd"])
    }

    #[test]
    fn output_link_matches_have_correct_starts() {
        let trie = Trie::build_trie(&["abcd", "cd"]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<(usize, &str)> = matches.map(|m| (m.start, m.text)).collect();
        res.sort();
        assert_eq!(res, vec![(0, "abcd"), (2, "cd")])
    }

    #[test]
    fn pending_output_does_not_block_subsequent_matches() {
        let trie = Trie::build_trie(&["abcd", "cd", "ef"]);
        let corpus = "abcdef";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        assert_eq!(res, vec!["abcd", "cd", "ef"])
    }
}

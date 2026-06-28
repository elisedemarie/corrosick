use std::str::CharIndices;

use crate::trie::{NodeId, Trie};

/// A match found in the search corpus.
#[derive(Clone, Debug)]
pub struct Match<'a> {
    /// The byte offset of the first character of the match within the corpus.
    ///
    /// For ASCII text this equals the character index but for text containing
    /// multi-byte UTF-8 characters the byte offset and character index will be different.
    pub start: usize,
    /// The matched text (borrowed from the corpus).
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

#[derive(Clone, Debug)]
pub struct Matches<'a, 'b> {
    corpus: &'a str,
    trie: &'b Trie,
    chars: CharIndices<'a>,
    pointer: NodeId,
    pending_output: Option<(NodeId, usize, char)>,
}

impl<'a, 'b> Matches<'a, 'b> {
    pub(crate) fn new(corpus: &'a str, trie: &'b Trie) -> Self {
        Self {
            corpus,
            trie,
            chars: corpus.char_indices(),
            pointer: 0,
            pending_output: None,
        }
    }

    fn get_output(
        &self,
        pointer: NodeId,
        text_pointer: usize,
        character: char,
    ) -> Option<Match<'a>> {
        let length = self.trie.get_end(pointer)?;
        Some(Match::from_match_end(
            self.corpus,
            length,
            text_pointer,
            character,
        ))
    }

    fn next_pending(&mut self) -> Option<Match<'a>> {
        let (output_id, text_pointer, character) = self.pending_output.take()?;
        let next_output = self.trie.get_output(output_id)?;
        self.pending_output = Some((next_output, text_pointer, character));
        self.get_output(next_output, text_pointer, character)
    }
}

impl<'a, 'b> Iterator for Matches<'a, 'b> {
    type Item = Match<'a>;
    fn next(&mut self) -> Option<Match<'a>> {
        if let Some(found_match) = self.next_pending() {
            return Some(found_match);
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
            if let Some(length) = self.trie.get_end(pointer) {
                found_match = Some(Match::from_match_end(
                    self.corpus,
                    length,
                    text_pointer,
                    character,
                ));
                break;
            }
            if let output_match @ Some(_) = self.next_pending() {
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
    fn finds_keyword_in_corpus() {
        let k1 = "abcd";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn does_not_find_keyword_not_in_corpus() {
        let k1 = "efgh";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        let exp: Vec<&str> = vec![];
        assert_eq!(res, exp)
    }

    #[test]
    fn finds_keyword_at_start_of_corpus() {
        let k1 = "abcd";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcdefgh";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn finds_keyword_at_end_of_corpus() {
        let k1 = "efgh";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcdefgh";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn finds_keyword_in_middle_of_corpus() {
        let k1 = "cd";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcdefgh";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn match_start_is_correct_index() {
        let k1 = "abcd";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "efghabcd";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<(usize, &str)> = matches.map(|m| (m.start, m.text)).collect();
        assert_eq!(res, vec![(4, k1)])
    }

    #[test]
    fn finds_keyword_appearing_twice() {
        let k1 = "abc";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcabc";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        assert_eq!(res, vec![k1, k1])
    }

    #[test]
    fn finds_multiple_non_subset_keywords() {
        let k1 = "abc";
        let k2 = "def";
        let trie = Trie::build_trie(&[k1, k2]);
        let corpus = "abcdef";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        let mut exp = vec![k1, k2];
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn finds_overlapping_keywords() {
        let k1 = "aa";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "aaaa";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        let mut exp = vec![k1, k1, k1];
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn empty_corpus_returns_nothing() {
        let k1 = "abcd";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        let exp: Vec<&str> = vec![];
        assert_eq!(res, exp)
    }

    #[test]
    fn keyword_longer_than_corpus_returns_nothing() {
        let k1 = "abcdefg";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        let exp: Vec<&str> = vec![];
        assert_eq!(res, exp)
    }

    #[test]
    fn single_char_keyword_is_found() {
        let k1 = "a";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn matches_are_case_sensitive() {
        let k1 = "abcd";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "ABCD";
        let matches = Matches::new(corpus, &trie);
        let res: Vec<&str> = matches.map(|m| m.text).collect();
        let exp: Vec<&str> = vec![];
        assert_eq!(res, exp)
    }

    #[test]
    fn multi_byte_char_in_corpus_does_not_panic() {
        let k1 = "llo";
        let trie = Trie::build_trie(&[k1]);
        let corpus = "héllo";
        let matches = Matches::new(corpus, &trie);
        matches.for_each(|_| {})
    }

    #[test]
    fn output_link_match_is_yielded_after_direct_match() {
        let k1 = "abcd";
        let k2 = "cd";
        let trie = Trie::build_trie(&[k1, k2]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        let mut exp = vec![k1, k2];
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn output_link_chain_is_fully_exhausted() {
        let k1 = "abcd";
        let k2 = "bcd";
        let k3 = "cd";
        let trie = Trie::build_trie(&[k1, k2, k3]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        let mut exp = vec![k1, k2, k3];
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn output_link_matches_have_correct_starts() {
        let k1 = "abcd";
        let k2 = "cd";
        let trie = Trie::build_trie(&[k1, k2]);
        let corpus = "abcd";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<(usize, &str)> = matches.map(|m| (m.start, m.text)).collect();
        res.sort();
        assert_eq!(res, vec![(0, k1), (2, k2)])
    }

    #[test]
    fn pending_output_does_not_block_subsequent_matches() {
        let k1 = "abcd";
        let k2 = "cd";
        let k3 = "ef";
        let trie = Trie::build_trie(&[k1, k2, k3]);
        let corpus = "abcdef";
        let matches = Matches::new(corpus, &trie);
        let mut res: Vec<&str> = matches.map(|m| m.text).collect();
        res.sort();
        let mut exp = vec![k1, k2, k3];
        exp.sort();
        assert_eq!(res, exp)
    }
}

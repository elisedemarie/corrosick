use crate::{matches::Matches, trie::Trie};

/// A set of keywords that can be searched for in any text.
///
/// Build once with [`AhoCorasick::new`]. Call [`find_matches`] to return an
/// iterator with found matches in that text.
///
/// [`find_matches`]: AhoCorasick::find_matches
#[derive(Clone, Debug)]
pub struct AhoCorasick {
    trie: Trie,
}

impl AhoCorasick {
    /// Builds the automaton from a set of keywords.
    ///
    /// Construction is O(m) where m is the total length of all keywords.
    /// Empty keywords are ignored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corrosick::AhoCorasick;
    ///
    /// let ac = AhoCorasick::new(&["foo", "bar", "baz"]);
    /// ```
    pub fn new(keywords: &[&str]) -> Self {
        let trie = Trie::build_trie(keywords);
        Self { trie }
    }

    /// Returns an iterator over all matches found in `corpus`.
    ///
    /// Matches are yielded in the order their final character appears in the
    /// text. If multiple keywords end at the same position, the longest match
    /// is yielded first followed by shorter ones.
    ///
    /// The search runs in O(n + z) time where n is the length of the corpus
    /// and z is the number of matches.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corrosick::AhoCorasick;
    ///
    /// let ac = AhoCorasick::new(&["he", "she", "his", "hers"]);
    /// let matches: Vec<&str> = ac.find_matches("ushers").map(|m| m.text).collect();
    /// assert_eq!(matches, vec!["she", "he", "hers"]);
    /// ```
    pub fn find_matches<'a>(&self, corpus: &'a str) -> Matches<'a, '_> {
        Matches::new(corpus, &self.trie)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_key_word_in_pattern_and_text() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "abcd";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn does_not_find_key_word_not_in_text() {
        let k1 = "efgh";
        let keywords = [k1];
        let corpus = "abcd";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        let exp: Vec<&str> = vec![];
        assert_eq!(res, exp)
    }

    #[test]
    fn finds_key_word_suffixed_in_text() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "effghabcd";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn finds_key_word_prefixed_in_text() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "abcdefgh";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn finds_keyword_infixed_in_text() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "efghabcdijkl";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn finds_both_keywords() {
        let k1 = "abcd";
        let k2 = "efgh";
        let keywords = [k1, k2];
        let corpus = "abcdefgh";
        let ac = AhoCorasick::new(&keywords);
        let mut res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        res.sort();
        let mut exp = Vec::from(keywords);
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn finds_single_char_kwds() {
        let k1 = "a";
        let k2 = "b";
        let k3 = "c";
        let k4 = "d";
        let k5 = "e";
        let keywords = [k1, k2, k3, k4, k5];
        let corpus = "abcdefgh";
        let ac = AhoCorasick::new(&keywords);
        let mut res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        res.sort();
        let mut exp = Vec::from(keywords);
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn finds_kwd_in_corpus_twice() {
        let k1 = "abc";
        let keywords = [k1];
        let corpus = "abc is a letter of the abc phabet";
        let ac = AhoCorasick::new(&keywords);
        let mut res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        res.sort();
        let mut exp = Vec::from([k1, k1]);
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn finds_overlapping_kwds() {
        let k1 = "aa";
        let keywords = [k1];
        let corpus = "aaaaaa";
        let ac = AhoCorasick::new(&keywords);
        let mut res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        res.sort();
        let mut exp = Vec::from([k1, k1, k1, k1, k1]);
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn empty_corpus_returns_nothing() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "";
        let ac = AhoCorasick::new(&keywords);
        let mut res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        res.sort();
        let mut exp: Vec<&str> = Vec::from([]);
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn kwd_longer_than_corpus_returns_nothing() {
        let k1 = "abcdefg";
        let keywords = [k1];
        let corpus = "abcd";
        let ac = AhoCorasick::new(&keywords);
        let mut res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        res.sort();
        let mut exp: Vec<&str> = Vec::from([]);
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn calling_on_different_corpora_returns_matches() {
        let k1 = "abc";
        let k2 = "def";
        let k3 = "ghi";
        let k4 = "jkl";
        let keywords = [k1, k2, k3, k4];
        let corpus_1 = "abcdef";
        let corpus_2 = "ghijkl";
        let ac = AhoCorasick::new(&keywords);
        let mut res_1: Vec<&str> = ac.find_matches(corpus_1).map(|it| it.text).collect();
        res_1.sort();
        let mut exp_1 = Vec::from([k1, k2]);
        exp_1.sort();
        assert_eq!(res_1, exp_1);
        let mut res_2: Vec<&str> = ac.find_matches(corpus_2).map(|it| it.text).collect();
        res_2.sort();
        let mut exp_2 = Vec::from([k3, k4]);
        exp_2.sort();
        assert_eq!(res_2, exp_2)
    }

    #[test]
    fn repeated_kwds_both_match() {
        let k1 = "abc";
        let k2 = "abc";
        let keywords = [k1, k2];
        let corpus = "abcd";
        let ac = AhoCorasick::new(&keywords);
        let mut res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        res.sort();
        let mut exp: Vec<&str> = Vec::from([k1]);
        exp.sort();
        assert_eq!(res, exp)
    }

    #[test]
    fn match_start_is_correct_index() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "efghabcdijkl";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<_> = ac.find_matches(corpus).collect();
        assert_eq!(res[0].start, 4)
    }

    #[test]
    fn matches_are_case_sensitive() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "ABCD";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        let exp: Vec<&str> = vec![];
        assert_eq!(res, exp)
    }

    #[test]
    fn single_char_corpus_matches_single_char_kwd() {
        let k1 = "a";
        let keywords = [k1];
        let corpus = "a";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn kwd_at_very_end_of_corpus_is_found() {
        let k1 = "xyz";
        let keywords = [k1];
        let corpus = "abcxyz";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn kwd_surrounded_by_punctuation_and_whitespace_is_found() {
        let k1 = "abcd";
        let keywords = [k1];
        let corpus = "hello, abcd! how are you";
        let ac = AhoCorasick::new(&keywords);
        let res: Vec<&str> = ac.find_matches(corpus).map(|it| it.text).collect();
        assert_eq!(res, vec![k1])
    }

    #[test]
    fn multi_byte_char_before_match_matches() {
        let k1 = "llo";
        let keywords = [k1];
        let corpus = "héllo";
        let ac = AhoCorasick::new(&keywords);
        ac.find_matches(corpus).for_each(|_| {})
    }

    #[test]
    fn empty_kwd_does_not_panic() {
        let k1 = "";
        let keywords = [k1];
        let corpus = "abcd";
        let ac = AhoCorasick::new(&keywords);
        ac.find_matches(corpus).for_each(|_| {})
    }

    #[test]
    fn kwd_containing_multi_byte_char_matches() {
        let k1 = "bé";
        let keywords = [k1];
        let corpus = "abé";
        let ac = AhoCorasick::new(&keywords);
        ac.find_matches(corpus).for_each(|_| {})
    }
}

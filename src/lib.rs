//! An implementation of the [Aho-Corasick algorithm] for fast multi-pattern string search.
//!
//! Aho-Corasick searches a text for multiple keywords simultaneously in linear time —
//! O(n + m + z), where n is the length of the text, m is the total length of all keywords,
//! and z is the number of matches found.
//!
//! # Example
//!
//! ```rust
//! use corrosick::AhoCorasick;
//!
//! let keywords = ["fox", "dog"];
//! let ac = AhoCorasick::new(&keywords);
//!
//! let matches: Vec<&str> = ac.find_matches("the quick brown fox jumps over the lazy dog")
//!     .map(|m| m.text)
//!     .collect();
//!
//! assert_eq!(matches, vec!["fox", "dog"]);
//! ```
//!
//! [Aho-Corasick algorithm]: https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm

mod aho_corasick;
mod matches;
mod trie;

pub use aho_corasick::AhoCorasick;
pub use matches::Match;

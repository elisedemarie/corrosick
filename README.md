# corrosick

A Rust implementation of the [Aho-Corasick algorithm](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm) for fast multi-pattern string search.

## Backstory

In 2024, while working as a Data Scientist, I needed access to a fast text search tool to process gigabytes of text data searching for a set of patterns. We were running in an offline environment and it was going to take weeks to get access to any Python package. We did, however, have access to C++. In one week, I learnt about the Aho-Corasick algorithm and taught myself enough C++ to write an implementation. It worked and was performant enough but it was jank. 

Memory leaks and bad patterns everywhere but I have come a long way since then. I ditched C++ for Rust pretty quickly but now, almost 2 years on, I wanted to challenge myself to re-write the algorithm. This crate and repo is a demonstration of how my skills have developed, a new Aho-Corasick algorithm written in just over a week (I have another job now so couldn’t devote as much time as I did the first time round!!).

Also worth noting. This repo was **not written with AI**. There's definitly a place and value to those tools but the point of this project was so show and work on my own raw Rust abilities.

## What it does

Searches a text for any number of keywords simultaneously in a single pass. Rather than scanning the text once per keyword, Aho-Corasick builds an automaton from all keywords upfront and searches in linear time — O(n + m + z), where n is the length of the text, m is the total length of all keywords, and z is the number of matches.

## Usage

```rust
use corrosick::AhoCorasick;

let keywords = ["fox", "dog"];
let ac = AhoCorasick::new(&keywords);

let matches: Vec<&str> = ac
	.find_matches("the quick brown fox jumps over the lazy dog")
	.map(|m| m.text)
	.collect();

assert_eq!(matches, vec!["fox", "dog"]);
```

Build the automaton once with `AhoCorasick::new`, then call `find_matches` on as many texts as you like. Each `Match` gives you the matched `text` and its `start` byte offset in the corpus.

## Match ordering

Matches are yielded in the order their final character appears in the text. When multiple keywords end at the same position (e.g. `"he"` and `"she"` both ending on the same character), the longest match is yielded first.

## UTF-8

The crate operates on `char` boundaries and handles multi-byte UTF-8 characters correctly. `Match::start` is a byte offset, not a character index.

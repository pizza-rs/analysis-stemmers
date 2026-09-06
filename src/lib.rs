#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused)]
#[cfg(test)]
extern crate std;

extern crate alloc;
use crate::engine::analysis::Token;
use crate::engine::analysis::TokenFilter;
use crate::engine::analysis::Tokenizer;
use alloc::borrow::Cow;
use alloc::vec::Vec;
pub use pizza_common as common;
pub use pizza_engine as engine;

mod snowball;

pub mod algorithms;
pub mod register;

pub use register::register_all;
pub use register::register_language_analyzers;
pub use register::register_token_filters;

/// Stemmer tokenizer. Several algorithms are supported, see [`algorithms`] or
/// https://github.com/pizza-rs/pizza-stemmers for a list of all available algorithms.
///
/// ❗️❗️ Tokens are expected to be lowercased beforehand.
#[derive(Clone)]
pub struct StemmerTokenizer {
    algorithm: algorithms::Algorithm,
}

impl StemmerTokenizer {
    /// Creates a new `StemmerTokenizer` [`StemmerTokenizer`] for a given language or variant algorithm.
    pub fn new(algorithm: algorithms::Algorithm) -> StemmerTokenizer {
        StemmerTokenizer { algorithm }
    }
}

#[cfg(feature = "english_porter_2")]
impl Default for StemmerTokenizer {
    /// Creates a new `StemmerTokenizer` [`StemmerTokenizer`] the default algorithm [`algorithms::english_porter_2`].
    fn default() -> Self {
        StemmerTokenizer::new(algorithms::english_porter_2)
    }
}

/// Implement the Tokenizer trait for StemmerTokenizer
impl Tokenizer for StemmerTokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
        // Tokenize the input text
        let tokens = tokenize_text(text);

        // Transform tokens by applying stemming algorithm
        tokens
            .into_iter()
            .map(|token| {
                // Apply stemming to the token text
                let stemmed_text = match (self.algorithm)(&token.term) {
                    Cow::Owned(stemmed_str) => Cow::Owned(stemmed_str),
                    Cow::Borrowed(stemmed_str) => Cow::Owned(stemmed_str.into()), // Convert to owned
                };

                // println!("{}=>{}",token.term,stemmed_text);

                // Create a new Token with the stemmed text
                Token {
                    term: stemmed_text,
                    start_offset: token.start_offset,
                    end_offset: token.end_offset,
                    position: token.position,
                }
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct StemmerFilter {
    algorithm: algorithms::Algorithm,
}

impl StemmerFilter {
    /// Creates a new `StemmerFilter` with the specified stemming algorithm.
    pub fn new(algorithm: algorithms::Algorithm) -> StemmerFilter {
        StemmerFilter { algorithm }
    }
}

impl TokenFilter for StemmerFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>){
        // Apply stemming to the token text
        let stemmed_text = match (self.algorithm)(&token.term) {
            Cow::Owned(stemmed_str) => Cow::Owned(stemmed_str),
            Cow::Borrowed(stemmed_str) => Cow::Owned(stemmed_str.into()), // Convert to owned
        };

        // Update the token with the stemmed text
        token.term = stemmed_text;

        // No new tokens are produced, so return None
        (false,None)
    }
}

fn tokenize_text<'a>(text: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut position = 0;

    let mut char_indices = text.char_indices().peekable();

    while let Some((start, ch)) = char_indices.next() {
        if ch.is_whitespace() {
            // Skip whitespace characters
            continue;
        }

        let token_start = start;
        let mut token_end = token_start;

        // Determine the end of the token
        while let Some(&(next_start, next_ch)) = char_indices.peek() {
            if next_ch.is_whitespace() {
                break;
            }
            token_end = next_start;
            char_indices.next();
        }

        // Handle the end of the last token
        if let Some((last_end, last_ch)) = char_indices.peek() {
            if last_ch.is_whitespace() {
                token_end = *last_end;
            }
        } else {
            token_end = text.len();
        }

        let term = &text[token_start..token_end];

        tokens.push(Token {
            term: Cow::Borrowed(term),
            start_offset: token_start as u32,
            end_offset: token_end as u32,
            position,
        });

        position += 1;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_single_word() {
        let text = "word";
        let tokens = tokenize_text(text);

        println!("{:?}", tokens);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].term, Cow::Borrowed("word"));
        assert_eq!(tokens[0].start_offset, 0);
        assert_eq!(tokens[0].end_offset, 4);
        assert_eq!(tokens[0].position, 0);
    }

    #[test]
    fn test_multiple_words() {
        let text = "this is a test";
        let tokens = tokenize_text(text);
        println!("{:?}", tokens);

        assert_eq!(tokens.len(), 4);

        assert_eq!(tokens[0].term, Cow::Borrowed("this"));
        assert_eq!(tokens[0].start_offset, 0);
        assert_eq!(tokens[0].end_offset, 4);
        assert_eq!(tokens[0].position, 0);

        assert_eq!(tokens[1].term, Cow::Borrowed("is"));
        assert_eq!(tokens[1].start_offset, 5);
        assert_eq!(tokens[1].end_offset, 7);
        assert_eq!(tokens[1].position, 1);

        assert_eq!(tokens[2].term, Cow::Borrowed("a"));
        assert_eq!(tokens[2].start_offset, 8);
        assert_eq!(tokens[2].end_offset, 9);
        assert_eq!(tokens[2].position, 2);

        assert_eq!(tokens[3].term, Cow::Borrowed("test"));
        assert_eq!(tokens[3].start_offset, 10);
        assert_eq!(tokens[3].end_offset, 14);
        assert_eq!(tokens[3].position, 3);
    }

    #[test]
    fn test_leading_trailing_spaces() {
        let text = "  leading and trailing  ";
        let tokens = tokenize_text(text);
        println!("{:?}", tokens);

        assert_eq!(tokens.len(), 3);

        assert_eq!(tokens[0].term, Cow::Borrowed("leading"));
        assert_eq!(tokens[0].start_offset, 2);
        assert_eq!(tokens[0].end_offset, 9);
        assert_eq!(tokens[0].position, 0);

        assert_eq!(tokens[1].term, Cow::Borrowed("and"));
        assert_eq!(tokens[1].start_offset, 10);
        assert_eq!(tokens[1].end_offset, 13);
        assert_eq!(tokens[1].position, 1);

        assert_eq!(tokens[2].term, Cow::Borrowed("trailing"));
        assert_eq!(tokens[2].start_offset, 14);
        assert_eq!(tokens[2].end_offset, 22);
        assert_eq!(tokens[2].position, 2);
    }

    #[test]
    fn test_basic_tokenization_and_stemming() {
        // Create a StemmerTokenizer with a specific algorithm
        let tokenizer = StemmerTokenizer::new(algorithms::english_porter_2);

        let text = "running runners";
        let tokens = tokenizer.tokenize(text);

        assert_eq!(tokens.len(), 2);

        // Check the first token
        assert_eq!(tokens[0].term, Cow::Borrowed("run")); // Assuming stemming reduces "running" to "run"
        assert_eq!(tokens[0].start_offset, 0);
        assert_eq!(tokens[0].end_offset, 7);
        assert_eq!(tokens[0].position, 0);

        // Check the second token
        assert_eq!(tokens[1].term, Cow::Borrowed("runner")); // Assuming stemming reduces "runners" to "runner"
        assert_eq!(tokens[1].start_offset, 8);
        assert_eq!(tokens[1].end_offset, 15);
        assert_eq!(tokens[1].position, 1);
    }

    #[test]
    fn test_tokenization_with_whitespace() {
        let tokenizer = StemmerTokenizer::new(algorithms::english_porter_2);

        let text = "  running   quickly  ";
        let tokens = tokenizer.tokenize(text);

        assert_eq!(tokens.len(), 2);

        // Check the first token
        assert_eq!(tokens[0].term, Cow::Borrowed("run")); // Assuming stemming reduces "running" to "run"
        assert_eq!(tokens[0].start_offset, 2);
        assert_eq!(tokens[0].end_offset, 9);
        assert_eq!(tokens[0].position, 0);

        // Check the second token
        assert_eq!(tokens[1].term, Cow::Borrowed("quick")); // Assuming stemming reduces "quickly" to "quick"
        assert_eq!(tokens[1].start_offset, 12);
        assert_eq!(tokens[1].end_offset, 19);
        assert_eq!(tokens[1].position, 1);
    }

    #[test]
    fn test_french_stemming() {
        let tokenizer = StemmerTokenizer::new(algorithms::french);

        let text = "manges mangeons";
        let tokens = tokenizer.tokenize(text);

        assert_eq!(tokens.len(), 2);

        // Check the first token
        assert_eq!(tokens[0].term, Cow::Borrowed("mang")); // Assuming stemming reduces "manges" to "mang"
        assert_eq!(tokens[0].start_offset, 0);
        assert_eq!(tokens[0].end_offset, 6);
        assert_eq!(tokens[0].position, 0);

        // Check the second token
        assert_eq!(tokens[1].term, Cow::Borrowed("mangeon")); // Assuming stemming reduces "mangeons" to "mangeon"
        assert_eq!(tokens[1].start_offset, 7);
        assert_eq!(tokens[1].end_offset, 15);
        assert_eq!(tokens[1].position, 1);
    }

    #[test]
    fn test_english_stemming_filter() {
        // Create a StemmerTokenizer with a specific algorithm
        let tokenizer = engine::analysis::WhitespaceTokenizer::new();

        // Tokenize some input text
        let text = "running runner ran";
        let tokens = tokenizer.tokenize(text);

        println!("{:?}", tokens);

        // Create a StemmerFilter using the same algorithm
        let filter = StemmerFilter::new(algorithms::english_porter_2);

        // Apply the filter to each token
        let filtered_tokens: Vec<Token> = tokens
            .into_iter()
            .map(|mut token| {
            let _ = filter.filter(&mut token);
            token
        })
            .collect();

        // Print the filtered tokens for verification
        for token in &filtered_tokens {
            println!("{:?}", token);
        }

        // Expected results (adjust based on your stemming algorithm's output)
        assert_eq!(filtered_tokens.len(), 3);

        // Check the first token
        assert_eq!(filtered_tokens[0].term, Cow::Borrowed("run")); // Example expected result
        assert_eq!(filtered_tokens[0].start_offset, 0);
        assert_eq!(filtered_tokens[0].end_offset, 7);
        assert_eq!(filtered_tokens[0].position, 0);

        // Check the second token
        assert_eq!(filtered_tokens[1].term, Cow::Borrowed("runner")); // Example expected result
        assert_eq!(filtered_tokens[1].start_offset, 8);
        assert_eq!(filtered_tokens[1].end_offset, 14);
        assert_eq!(filtered_tokens[1].position, 1);

        // Check the third token
        assert_eq!(filtered_tokens[2].term, Cow::Borrowed("ran")); // Example expected result
        assert_eq!(filtered_tokens[2].start_offset, 15);
        assert_eq!(filtered_tokens[2].end_offset, 18);
        assert_eq!(filtered_tokens[2].position, 2);
    }
}

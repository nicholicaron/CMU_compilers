// Scanner takes in raw source code as a series of characters and groups it into a series
// of chunks called tokens ("words", and "punctuation" that make up the language's grammar)
//
// Each word is classified into a syntactic category (e.g. "number", "string", "identifier")
//
// Regular expressions are used to define the set of valid words in the source language

use std::fs;
use std::io::*;
mod token;

pub fn run_file(path: String) -> Result<()> {
    if let Ok(file) = fs::read_to_string(path) {
        scan(file);
    } else {
        let file_io_error = Error::from(ErrorKind::NotFound);
        return Err(file_io_error);
    }
    Ok(())
}

pub fn run_prompt() -> Result<()> {
    println!("Please enter the file to be compiled: ");

    match stdin().lines().next().unwrap() {
        Ok(path) => {
            run_file(path).unwrap();
            Ok(())
        }
        Err(name_parsing_error) => Err(name_parsing_error),
    }
}

// TO DO:
// Associate line numbers and column numbers with tokens
    // CharIndices stores index -- can this be used to derive line/col number, which can then be
    // stored in a Token Struct?
// Do we tokenize or discard spaces/newlines?
    //  I would assume tokenizing them would work best with keeping track of line/col numbers
// How do we categorize keywords?
//
fn scan(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut char_indices = source.char_indices().peekable();

    // let's use CharIndices to keep track of state when we need to check if a token is single or multi-character
    // CharIndices provides peekable method to let us conditionally advance the current index if we have a multi-character token
    // Using while let here instead of for in to avoid moving the iterator produced by char_indices
    while let Some((index, character)) = char_indices.next() {
        let token = match character {
            // match a single token
            '+' => Token::Plus,
            '=' => match char_indices.next_if_eq(&(index + 1, '=')) {
                Some(_) => Token::EqualEqual,
                None => Token::Equal,
            },
            '!' => match char_indices.next_if_eq(&(index + 1, '=')) {
                Some(_) => Token::NotEqual,
                None => Token::Invalid("!".to_string()),
            },
            // Checking for strings

            // take_while method conditionally consumes and returns elements of an iterator as long as its predicate function evaluates to true
            // when the predicate function evals to false, the iterator terminates
            // take_while takes possession of the original iterator, so we will instead borrow it mutable via by_ref()
            // elements consumed by take_while are also removed from original iterator so we don't have to worry about double counting
            '"' => {
                let mut last_char_matched: char;

                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_index, character)| {
                        last_char_matched = *character;
                        *character != '"'
                    })
                    // call to map ditches the index value, keeping the character so we can append it to s
                    .map(|(_index, character)| character)
                    .collect();

                match last_char_matched {
                    '"' => Token::StringLiteral(s),
                    _ => Token::Invalid("Unterminated literal".to_string()),
                }
            }
            n if char::is_numeric(n) => {
                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_index, character)| char::is_numeric(*character))
                    .map(|(_index, character)| character)
                    .collect();

                let number: u32 = s.parse::<u32>().unwrap();
                Token::Number(number)
            }
            _ => Token::Invalid(format!("{}", character)),
        };
        tokens.push(token);
    }
    tokens
}

enum Token {
    Plus,
    Equal,
    EqualEqual,
    NotEqual,
    Number(u32),
    StringLiteral(String),
    Invalid(String),
}

// identifier
enum Id {

}

//
enum Separator {
    
}

enum UnOp{

}

enum 

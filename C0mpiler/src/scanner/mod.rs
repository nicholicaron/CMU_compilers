// Scanner takes in raw source code as a series of characters and groups it into a series
// of chunks called tokens ("words", and "punctuation" that make up the language's grammar)
//
// Each word is classified into a syntactic category (e.g. "number", "string", "identifier")
//
// Regular expressions are used to define the set of valid words in the source language

use std::fs;
use std::io::*;
mod token;
use crate::scanner::token::{Token, BinOp, AsnOp, PostOp};
// Id, Num, StrLit, ChrLit, LibLit, SChar, CChar, Sep, UnOp, BinOp, AsnOp, PostOp};


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
    let mut tokens = Vec::<Token>::new();
    let mut char_indices = source.char_indices().peekable();

    while let Some((index, character)) = char_indices.next() {
        let token = match character {
            // match a single token
            '+' => match char_indices.next() {
                Some((_, ' ')) => Token::BinOp(BinOp::Plus),
                Some((_, '=')) => Token::AsnOp(AsnOp::IncAsn),
                Some((_, '+')) => Token::PostOp(PostOp::Inc),
                _ => break, // What is the error case here? S_e? 
            },
            /*
            '-' => match char_indices.peek() {
                ' ' => Minus,
                '=' => DecAsn,
                '-' => Dec,
                _ => todo!(); // What is the error case here? S_e? 
            },
            '*' => match char_indices.next_if_eq(&(index + 1, '=')) {
                Some(_) => MultAsn,
                // How can we check for associativity? Differentiate between BinOp::IntTimes &
                // UnOp::Pointer ??
                None => todo!(),
            },
            '/' => match char_indices.next_if_eq(&(index + 1, '=')) {
                Some(_) => DivAsn,
                None => Divide,
            },
            '%' => match char_indices.next_if_eq(&(index + 1, '=')) {
                Some(_) => ModAsn,
                None => Modulo,
            },
            '<' => match char_indices.peek() {
                ' ' => Less,
                '<' => match char_indices.peek() {
                    ' ' => ShiftLeft,
                    '<' => match char_indices.next_if_eq(&(index + ))
                }

            },
            '>' => match char_indices.next_if_eq(&(index + 1, '<')) {
                Some(_) => match char_indices.next_if_eq(&(index + 2, '=')) {
                    Some(_) => RShiftAsn,
                    None => RShift,
                },
                None => Greater,
            },
            '=' => match char_indices.next_if_eq(&(index + 1, '=')) {
                Some(_) => Equality,
                None => EqAsn,
            },
            '!' => match char_indices.next_if_eq(&(index + 1, '=')) {
                Some(_) => ,
                None => ,
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
                    '"' => StrLit::StringLiteral(s),
                    _ => ,
                }
            }
            n if char::is_numeric(n) => {
                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_index, character)| char::is_numeric(*character))
                    .map(|(_index, character)| character)
                    .collect();

                let number: u32 = s.parse::<u32>().unwrap();
                DecNum(number),    
            }
        };
        */
        tokens.push(token);
        };
    }
    tokens
}


// Scanner takes in raw source code as a series of characters and groups it into a series
// of chunks called tokens ("words", and "punctuation" that make up the language's grammar)
//
// Each word is classified into a syntactic category (e.g. "number", "string", "identifier")
//
// Regular expressions are used to define the set of valid words in the source language
#[allow(unused_imports)]
use std::fs;
use std::io::*;
pub mod token;
use crate::scanner::token::{
    AsnOp, BinOp, CChar, ChrLit, Id, LibLit, Num, PostOp, SChar, Sep, StrLit, Token, UnOp,
};

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
pub fn scan(source: String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut char_indices = source.char_indices().peekable();

    while let Some((_index, character)) = char_indices.next() {
        //
        // Need to handle corner cases around + and -
        // e.g. identifier-name should not resolve to BinOp Minus and should be part of identifier
        // name
        // Is this a matter of precedence? Checking for Identifiers befor BinOps
        //      Handled by the parser?
        // Also, is the wildcard follow up resolving to BinOp::Minus a safe conclusion?
        //
        // Additionally, where is it appropriate do disambiguate between next, next_if_eq, and
        // peek?
        // Generally sticking with next() for now
        //      - After Consuming one character, and conditionally checking next char, use next_if()
        //      to avoid consuming
        //
        // Should token be wrapped in Option then unwrapped?
        //
        let token = match character {
            // match a single token
            '+' => match char_indices.peek() {
                // "+="
                Some((_, '=')) => {
                    // If we have a match, consume next elem
                    char_indices.next();
                    Token::AsnOp(AsnOp::IncAsn)
                }
                // "++"
                Some((_, '+')) => {
                    char_indices.next();
                    Token::PostOp(PostOp::Inc)
                }
                // '+'
                _ => Token::BinOp(BinOp::Plus),
            },
            '-' => match char_indices.peek() {
                // "-="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::DecAsn)
                }
                // "--"
                Some((_, '-')) => {
                    char_indices.next();
                    Token::PostOp(PostOp::Dec)
                }
                // '-'
                // Some(_) => Token::BinOp(BinOp::Minus),
                // _ => continue,
                _ => Token::BinOp(BinOp::Minus),
            },
            '*' => match char_indices.peek() {
                // "*="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::MultAsn)
                }
                Some(_) => {
                    match tokens.last() {
                        // Should be preceded by a number OR an identifier that resolves to a number
                        // "num *"
                        Some(Token::Num(_)) | Some(Token::Id(_)) => Token::BinOp(BinOp::IntTimes),
                        // "*identifier"
                        _ => Token::UnOp(UnOp::Pointer),
                    }
                }
                _ => continue,
            },
            // NOTE: We are not currently validating previous token for division operator, as we
            // did for disambiguating the multiplication operator and pointers
            //
            // This is because there is no ambiguity for resolving the token:
            //      I assume this syntax check will occur in the parser
            //
            '/' => match char_indices.peek() {
                // "/="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::DivAsn)
                }
                // '/'
                _ => Token::BinOp(BinOp::Divide),
            },
            '%' => match char_indices.peek() {
                // "%="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::ModAsn)
                }
                // '%'
                _ => Token::BinOp(BinOp::Modulo),
            },
            '<' => match char_indices.peek() {
                Some((_, '<')) => {
                    char_indices.next();
                    match char_indices.peek() {
                        // "<<="
                        Some((_, '=')) => {
                            char_indices.next();
                            Token::AsnOp(AsnOp::LShiftAsn)
                        }
                        // "<<"
                        _ => Token::BinOp(BinOp::ShiftLeft),
                    }
                }
                // '<'
                _ => Token::BinOp(BinOp::Less),
            },
            '>' => match char_indices.peek() {
                Some((_, '>')) => {
                    char_indices.next();
                    match char_indices.peek() {
                        // ">>="
                        Some((_, '=')) => {
                            char_indices.next();
                            Token::AsnOp(AsnOp::RShiftAsn)
                        }
                        // ">>"
                        _ => Token::BinOp(BinOp::ShiftRight),
                    }
                }
                // '>'
                _ => Token::BinOp(BinOp::Greater),
            },
            '=' => match char_indices.peek() {
                // "=="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::BinOp(BinOp::Equality)
                }
                // '='
                _ => Token::AsnOp(AsnOp::EqAsn),
            },
            '!' => match char_indices.peek() {
                // "!="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::BinOp(BinOp::Disequality)
                }
                // '!'
                _ => Token::UnOp(UnOp::LogicalNOT),
            },
            /*
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
            },
            */
            _ => continue,
        };
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus() {
        let res1 = vec![Token::AsnOp(AsnOp::IncAsn)];
        let res2 = vec![Token::PostOp(PostOp::Inc)];
        let res3 = vec![Token::BinOp(BinOp::Plus)];

        assert_eq!(scan("+=".to_string()), res1);
        assert_eq!(scan("++".to_string()), res2);
        assert_eq!(scan("+".to_string()), res3);
        assert_eq!(scan("+ ".to_string()), res3);
        assert_eq!(scan(" +".to_string()), res3);
        assert_eq!(scan(" + ".to_string()), res3);
    }

    #[test]
    fn minus() {
        let res1 = vec![Token::AsnOp(AsnOp::DecAsn)];
        let res2 = vec![Token::PostOp(PostOp::Dec)];
        let res3 = vec![Token::BinOp(BinOp::Minus)];

        assert_eq!(scan("-=".to_string()), res1);
        assert_eq!(scan("--".to_string()), res2);
        assert_eq!(scan("-".to_string()), res3);
        assert_eq!(scan("- ".to_string()), res3);
        assert_eq!(scan(" -".to_string()), res3);
        assert_eq!(scan(" - ".to_string()), res3);
    }

    #[test]
    fn asterisk() {
        let res1 = vec![Token::AsnOp(AsnOp::MultAsn)];
        // let res2 = vec![Token::Num(1), Token::BinOp(BinOp::IntTimes)];
        let res3 = vec![
            Token::UnOp(UnOp::Pointer), /*, Token::Identifier("null") */
        ];

        assert_eq!(scan("*=".to_string()), res1);
        // assert_eq!(scan("1 *".to_string()), res2);
        assert_eq!(scan("*null".to_string()), res3);
    }

    #[test]
    fn divide() {
        let res1 = vec![Token::AsnOp(AsnOp::DivAsn)];
        let res2 = vec![Token::BinOp(BinOp::Divide)];

        assert_eq!(scan("/=".to_string()), res1);
        assert_eq!(scan("/".to_string()), res2);
    }

    #[test]
    fn modulo() {
        let res1 = vec![Token::AsnOp(AsnOp::ModAsn)];
        let res2 = vec![Token::BinOp(BinOp::Modulo)];

        assert_eq!(scan("%=".to_string()), res1);
        assert_eq!(scan("%".to_string()), res2);
    }

    #[test]
    fn less_than_sign() {
        let res1 = vec![Token::AsnOp(AsnOp::LShiftAsn)];
        let res2 = vec![Token::BinOp(BinOp::ShiftLeft)];
        let res3 = vec![Token::BinOp(BinOp::Less)];

        assert_eq!(scan("<<=".to_string()), res1);
        assert_eq!(scan("<<".to_string()), res2);
        assert_eq!(scan("<".to_string()), res3);
    }

    #[test]
    fn greater_than_sign() {
        let res1 = vec![Token::AsnOp(AsnOp::RShiftAsn)];
        let res2 = vec![Token::BinOp(BinOp::ShiftRight)];
        let res3 = vec![Token::BinOp(BinOp::Greater)];

        assert_eq!(scan(">>=".to_string()), res1);
        assert_eq!(scan(">>".to_string()), res2);
        assert_eq!(scan(">".to_string()), res3);
    }

    #[test]
    fn equals() {
        let res1 = vec![Token::BinOp(BinOp::Equality)];
        let res2 = vec![Token::AsnOp(AsnOp::EqAsn)];

        assert_eq!(scan("==".to_string()), res1);
        assert_eq!(scan("=".to_string()), res2);
    }

    #[test]
    fn bang() {
        let res1 = vec![Token::BinOp(BinOp::Disequality)];
        let res2 = vec![Token::UnOp(UnOp::LogicalNOT)];

        assert_eq!(scan("!=".to_string()), res1);
        assert_eq!(scan("!".to_string()), res2);
    }
}

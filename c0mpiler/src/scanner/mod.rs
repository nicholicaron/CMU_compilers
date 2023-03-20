#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::fs;
use std::io::*;
pub mod token;
use crate::scanner::token::{
    AsnOp, BinOp, CChar, ChrLit, DecNum, Esc, Id, Keyword, LibLit, Num, PostOp, SChar, Sep, StrLit,
    Token, UnOp,
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
                // "->"
                Some((_, '>')) => {
                    char_indices.next();
                    Token::BinOp(BinOp::FieldDeref)
                }
                _ => match tokens.last() {
                    // "<identifier or number>-"
                    Some(Token::Num(_)) | Some(Token::Id(_)) => Token::BinOp(BinOp::Minus),
                    // "<!identifier or !number>-<identifier or number>"
                    _ => Token::UnOp(UnOp::UnaryMinus),
                },
            },
            '*' => match char_indices.peek() {
                // "*="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::MultAsn)
                }
                _ => {
                    match tokens.last() {
                        // Should be preceded by a number OR an identifier that resolves to a number
                        // "num *"
                        Some(Token::Num(_)) | Some(Token::Id(_)) => {
                            println!("{:?}", tokens.last().unwrap());
                            Token::BinOp(BinOp::IntTimes)
                        }
                        // pointer initialization: "<keyword::type> *"
                        // pointer dereference: "*<identifier>"
                        _ => Token::UnOp(UnOp::Pointer),
                    }
                }
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
                //"<="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::BinOp(BinOp::LessEq)
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
                // ">="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::BinOp(BinOp::GreaterEq)
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
            '&' => match char_indices.peek() {
                // "&&"
                Some((_, '&')) => {
                    char_indices.next();
                    Token::BinOp(BinOp::LogicalAND)
                }
                // "&="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::ANDAsn)
                }
                // '&'
                _ => Token::BinOp(BinOp::BitwiseAND),
            },
            '^' => match char_indices.peek() {
                // "^="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::XORAsn)
                }
                _ => Token::BinOp(BinOp::BitwiseXOR),
            },
            '|' => match char_indices.peek() {
                // "||"
                Some((_, '|')) => {
                    char_indices.next();
                    Token::BinOp(BinOp::LogicalOR)
                }
                // "|="
                Some((_, '=')) => {
                    char_indices.next();
                    Token::AsnOp(AsnOp::ORAsn)
                }
                // '|'
                _ => Token::BinOp(BinOp::BitwiseOR),
            },
            '.' => Token::BinOp(BinOp::FieldSelect),
            '?' => Token::BinOp(BinOp::CondEq),
            ':' => Token::BinOp(BinOp::CondAsn),
            '~' => Token::UnOp(UnOp::BitwiseNOT),
            '(' => Token::Sep(Sep::LParen),
            ')' => Token::Sep(Sep::RParen),
            '{' => Token::Sep(Sep::LCurly),
            '}' => Token::Sep(Sep::RCurly),
            '[' => Token::Sep(Sep::LBracket),
            ']' => Token::Sep(Sep::RBracket),
            ',' => Token::Sep(Sep::Comma),
            ';' => Token::Sep(Sep::SemiColon),
            // Esc Sequences: Note, we have to escape the backslash character so '\\' evals to '\'
            '\\' => match char_indices.peek() {
                Some((_, 'a')) => {
                    char_indices.next();
                    Token::Esc(Esc::Alert)
                }
                Some((_, 'b')) => {
                    char_indices.next();
                    Token::Esc(Esc::Backspace)
                }
                Some((_, 'f')) => {
                    char_indices.next();
                    Token::Esc(Esc::FormfeedPgBrk)
                }
                Some((_, 'n')) => {
                    char_indices.next();
                    Token::Esc(Esc::Newline)
                }
                Some((_, 'r')) => {
                    char_indices.next();
                    Token::Esc(Esc::CarriageReturn)
                }
                Some((_, 't')) => {
                    char_indices.next();
                    Token::Esc(Esc::HorizontalTab)
                }
                Some((_, 'v')) => {
                    char_indices.next();
                    Token::Esc(Esc::VerticalTab)
                }
                Some((_, '\\')) => {
                    char_indices.next();
                    Token::Esc(Esc::Backslash)
                }
                Some((_, '\'')) => {
                    char_indices.next();
                    Token::Esc(Esc::Apostrophe)
                }
                Some((_, '\"')) => {
                    char_indices.next();
                    Token::Esc(Esc::DoubleQuote)
                }
                _ => continue,
            },
            // Checking for strings

            // take_while method conditionally consumes and returns elements of an iterator as long as its predicate function evaluates to true
            // when the predicate function evals to false, the iterator terminates
            // take_while takes possession of the original iterator, so we will instead borrow it mutable via by_ref()
            // elements consumed by take_while are also removed from original iterator so we don't have to worry about double counting
            '"' => {
                let mut last_char_matched: char = '"';
                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_index, character)| {
                        last_char_matched = *character;
                        *character != '\"'
                    })
                    // call to map ditches the index value, keeping the character so we can append it to s
                    .map(|(_index, character)| character)
                    .collect();

                match last_char_matched {
                    '"' => Token::StrLit(StrLit::StringLiteral(s)),
                    _ => panic!(),
                }
            }
            c if char::is_alphabetic(c) => {
                let mut s = c.to_string();
                let mut stop_flag = false;
                while !stop_flag {
                    if let Some((_index, next_char)) = char_indices.peek() {
                        if (next_char.is_alphanumeric() || *next_char == '_') && *next_char != ' ' {
                            if let Some((_index, next_char)) = char_indices.next() {
                                s.push(next_char);
                            }
                        } else {
                            stop_flag = true;
                        }
                    } else {
                        stop_flag = true;
                    }
                }

                match s.as_str() {
                    "int" => Token::Keyword(Keyword::Int),
                    "bool" => Token::Keyword(Keyword::Bool),
                    "string" => Token::Keyword(Keyword::String),
                    "char" => Token::Keyword(Keyword::Char),
                    "void" => Token::Keyword(Keyword::Void),
                    "struct" => Token::Keyword(Keyword::Struct),
                    "typedef" => Token::Keyword(Keyword::Typedef),
                    "if" => Token::Keyword(Keyword::If),
                    "else" => Token::Keyword(Keyword::Else),
                    "while" => Token::Keyword(Keyword::While),
                    "for" => Token::Keyword(Keyword::For),
                    "continue" => Token::Keyword(Keyword::Continue),
                    "break" => Token::Keyword(Keyword::Break),
                    "return" => Token::Keyword(Keyword::Return),
                    "assert" => Token::Keyword(Keyword::Assert),
                    "error" => Token::Keyword(Keyword::Error),
                    "true" => Token::Keyword(Keyword::True),
                    "false" => Token::Keyword(Keyword::False),
                    "NULL" => Token::Keyword(Keyword::Null),
                    "alloc" => Token::Keyword(Keyword::Alloc),
                    "alloc_array" => Token::Keyword(Keyword::AllocArray),
                    _ => Token::Id(Id::Id(s)),
                }
            }
            // How to handle hexnums?
            n if char::is_numeric(n) => {
                let mut num: String = n.to_string();
                let mut stop_flag = false;
                while !stop_flag {
                    if let Some((_index, next_digit)) = char_indices.peek() {
                        if next_digit.is_digit(10) {
                            if let Some((_index, next_digit)) = char_indices.next() {
                                num.push(next_digit);
                            }
                        } else {
                            stop_flag = true;
                        }
                    } else {
                        stop_flag = true;
                    }
                }

                let num: u32 = num.parse::<u32>().unwrap();
                Token::Num(Num::DecNum(DecNum::DecNum(num)))
            }
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
    fn Id() {
        let Hello = vec![Token::Id(Id::Id("hello".to_string()))];
        let World = vec![Token::Id(Id::Id("world".to_string()))];
        let HelloWorld = vec![
            Token::Id(Id::Id("hello".to_string())),
            Token::Id(Id::Id("world".to_string())),
        ];

        assert_eq!(scan("hello".to_string()), Hello);
        assert_eq!(scan("world".to_string()), World);
        assert_eq!(scan("hello world".to_string()), HelloWorld);
    }

    #[test]
    fn StrLit() {
        let Hello = vec![Token::StrLit(StrLit::StringLiteral("hello".to_string()))];
        let World = vec![Token::StrLit(StrLit::StringLiteral("world!".to_string()))];
        let HelloWorld = vec![Token::StrLit(StrLit::StringLiteral(
            "hello world!".to_string(),
        ))];
        let HelloNum = vec![Token::StrLit(StrLit::StringLiteral("hello 42".to_string()))];

        assert_eq!(scan("\"hello\"".to_string()), Hello);
        assert_eq!(scan("\"world!\"".to_string()), World);
        assert_eq!(scan("\"hello world!\"".to_string()), HelloWorld);
        assert_eq!(scan("\"hello 42\"".to_string()), HelloNum);
    }

    #[test]
    fn Keyword() {
        let Int = vec![Token::Keyword(Keyword::Int)];
        let Bool = vec![Token::Keyword(Keyword::Bool)];
        let String = vec![Token::Keyword(Keyword::String)];
        let Char = vec![Token::Keyword(Keyword::Char)];
        let Void = vec![Token::Keyword(Keyword::Void)];
        let Struct = vec![Token::Keyword(Keyword::Struct)];
        let Typedef = vec![Token::Keyword(Keyword::Typedef)];
        let If = vec![Token::Keyword(Keyword::If)];
        let Else = vec![Token::Keyword(Keyword::Else)];
        let While = vec![Token::Keyword(Keyword::While)];
        let For = vec![Token::Keyword(Keyword::For)];
        let Continue = vec![Token::Keyword(Keyword::Continue)];
        let Break = vec![Token::Keyword(Keyword::Break)];
        let Return = vec![Token::Keyword(Keyword::Return)];
        let Assert = vec![Token::Keyword(Keyword::Assert)];
        let Error = vec![Token::Keyword(Keyword::Error)];
        let True = vec![Token::Keyword(Keyword::True)];
        let False = vec![Token::Keyword(Keyword::False)];
        let Null = vec![Token::Keyword(Keyword::Null)];
        let Alloc = vec![Token::Keyword(Keyword::Alloc)];
        let AllocArray = vec![Token::Keyword(Keyword::AllocArray)];

        assert_eq!(scan("int".to_string()), Int);
        assert_eq!(scan("bool".to_string()), Bool);
        assert_eq!(scan("string".to_string()), String);
        assert_eq!(scan("char".to_string()), Char);
        assert_eq!(scan("void".to_string()), Void);
        assert_eq!(scan("struct".to_string()), Struct);
        assert_eq!(scan("typedef".to_string()), Typedef);
        assert_eq!(scan("if".to_string()), If);
        assert_eq!(scan("else".to_string()), Else);
        assert_eq!(scan("while".to_string()), While);
        assert_eq!(scan("for".to_string()), For);
        assert_eq!(scan("continue".to_string()), Continue);
        assert_eq!(scan("break".to_string()), Break);
        assert_eq!(scan("return".to_string()), Return);
        assert_eq!(scan("assert".to_string()), Assert);
        assert_eq!(scan("error".to_string()), Error);
        assert_eq!(scan("true".to_string()), True);
        assert_eq!(scan("false".to_string()), False);
        assert_eq!(scan("NULL".to_string()), Null);
        assert_eq!(scan("alloc".to_string()), Alloc);
        assert_eq!(scan("alloc_array".to_string()), AllocArray);
    }

    #[test]
    fn DecNum() {
        let answer_to_universe = vec![Token::Num(Num::DecNum(DecNum::DecNum(42)))];
        let multiple_numbers = vec![
            Token::Num(Num::DecNum(DecNum::DecNum(1))),
            Token::Num(Num::DecNum(DecNum::DecNum(23))),
            Token::Num(Num::DecNum(DecNum::DecNum(4567))),
        ];

        assert_eq!(scan("42".to_string()), answer_to_universe);
        assert_eq!(scan("1 23 4567".to_string()), multiple_numbers);
    }

    #[test]
    fn Esc() {
        let Alert = vec![Token::Esc(Esc::Alert)];
        let Backspace = vec![Token::Esc(Esc::Backspace)];
        let FormfeedPgBrk = vec![Token::Esc(Esc::FormfeedPgBrk)];
        let Newline = vec![Token::Esc(Esc::Newline)];
        let CarriageReturn = vec![Token::Esc(Esc::CarriageReturn)];
        let HorizontalTab = vec![Token::Esc(Esc::HorizontalTab)];
        let VerticalTab = vec![Token::Esc(Esc::VerticalTab)];
        let Backslash = vec![Token::Esc(Esc::Backslash)];
        let Apostrophe = vec![Token::Esc(Esc::Apostrophe)];
        let DoubleQuote = vec![Token::Esc(Esc::DoubleQuote)];

        assert_eq!(scan("\\a".to_string()), Alert);
        assert_eq!(scan("\\b".to_string()), Backspace);
        assert_eq!(scan("\\f".to_string()), FormfeedPgBrk);
        assert_eq!(scan("\\n".to_string()), Newline);
        assert_eq!(scan("\\r".to_string()), CarriageReturn);
        assert_eq!(scan("\\t".to_string()), HorizontalTab);
        assert_eq!(scan("\\v".to_string()), VerticalTab);
        assert_eq!(scan("\\\\".to_string()), Backslash);
        assert_eq!(scan("\\'".to_string()), Apostrophe);
        assert_eq!(scan("\\\" ".to_string()), DoubleQuote);
    }

    #[test]
    fn Sep() {
        let LParen = vec![Token::Sep(Sep::LParen)];
        let RParen = vec![Token::Sep(Sep::RParen)];
        let LBracket = vec![Token::Sep(Sep::LBracket)];
        let RBracket = vec![Token::Sep(Sep::RBracket)];
        let LCurly = vec![Token::Sep(Sep::LCurly)];
        let RCurly = vec![Token::Sep(Sep::RCurly)];
        let Comma = vec![Token::Sep(Sep::Comma)];
        let SemiColon = vec![Token::Sep(Sep::SemiColon)];

        assert_eq!(scan("(".to_string()), LParen);
        assert_eq!(scan(")".to_string()), RParen);
        assert_eq!(scan("[".to_string()), LBracket);
        assert_eq!(scan("]".to_string()), RBracket);
        assert_eq!(scan("{".to_string()), LCurly);
        assert_eq!(scan("}".to_string()), RCurly);
        assert_eq!(scan(",".to_string()), Comma);
        assert_eq!(scan(";".to_string()), SemiColon);
    }

    #[test]
    fn UnOp() {
        let LogicalNOT = vec![Token::UnOp(UnOp::LogicalNOT)];
        let BitwiseNOT = vec![Token::UnOp(UnOp::BitwiseNOT)];
        let UnaryMinus = vec![Token::UnOp(UnOp::UnaryMinus)];
        let Pointer = vec![
            Token::UnOp(UnOp::Pointer),
            Token::Id(Id::Id("variable".to_string())),
        ];

        assert_eq!(scan("!".to_string()), LogicalNOT);
        assert_eq!(scan("~".to_string()), BitwiseNOT);
        assert_eq!(scan("-".to_string()), UnaryMinus);
        assert_eq!(scan("*variable".to_string()), Pointer);
    }

    #[test]
    fn BinOp() {
        let CondEq = vec![Token::BinOp(BinOp::CondEq)];
        let FieldSelect = vec![Token::BinOp(BinOp::FieldSelect)];
        let FieldDeref = vec![Token::BinOp(BinOp::FieldDeref)];
        let IntTimes = vec![
            Token::Num(Num::DecNum(DecNum::DecNum(12))),
            Token::BinOp(BinOp::IntTimes),
        ];
        let Divide = vec![Token::BinOp(BinOp::Divide)];
        let Modulo = vec![Token::BinOp(BinOp::Modulo)];
        let Plus = vec![Token::BinOp(BinOp::Plus)];
        let Minus = vec![
            Token::Num(Num::DecNum(DecNum::DecNum(1))),
            Token::BinOp(BinOp::Minus),
        ];
        let ShiftLeft = vec![Token::BinOp(BinOp::ShiftLeft)];
        let Less = vec![Token::BinOp(BinOp::Less)];
        let LessEq = vec![Token::BinOp(BinOp::LessEq)];
        let ShiftRight = vec![Token::BinOp(BinOp::ShiftRight)];
        let Greater = vec![Token::BinOp(BinOp::Greater)];
        let GreaterEq = vec![Token::BinOp(BinOp::GreaterEq)];
        let Equality = vec![Token::BinOp(BinOp::Equality)];
        let BitwiseAND = vec![Token::BinOp(BinOp::BitwiseAND)];
        let LogicalAND = vec![Token::BinOp(BinOp::LogicalAND)];
        let BitwiseXOR = vec![Token::BinOp(BinOp::BitwiseXOR)];
        let Disequality = vec![Token::BinOp(BinOp::Disequality)];
        let BitwiseOR = vec![Token::BinOp(BinOp::BitwiseOR)];
        let LogicalOR = vec![Token::BinOp(BinOp::LogicalOR)];
        let CondAsn = vec![Token::BinOp(BinOp::CondAsn)];

        assert_eq!(scan("?".to_string()), CondEq);
        assert_eq!(scan(".".to_string()), FieldSelect);
        assert_eq!(scan("->".to_string()), FieldDeref);
        assert_eq!(scan("12 *".to_string()), IntTimes);
        assert_eq!(scan("/".to_string()), Divide);
        assert_eq!(scan("%".to_string()), Modulo);
        assert_eq!(scan("+".to_string()), Plus);
        assert_eq!(scan("1 -".to_string()), Minus);
        assert_eq!(scan("<<".to_string()), ShiftLeft);
        assert_eq!(scan("<".to_string()), Less);
        assert_eq!(scan("<=".to_string()), LessEq);
        assert_eq!(scan(">>".to_string()), ShiftRight);
        assert_eq!(scan(">".to_string()), Greater);
        assert_eq!(scan(">=".to_string()), GreaterEq);
        assert_eq!(scan("==".to_string()), Equality);
        assert_eq!(scan("&".to_string()), BitwiseAND);
        assert_eq!(scan("&&".to_string()), LogicalAND);
        assert_eq!(scan("^".to_string()), BitwiseXOR);
        assert_eq!(scan("!=".to_string()), Disequality);
        assert_eq!(scan("|".to_string()), BitwiseOR);
        assert_eq!(scan("||".to_string()), LogicalOR);
        assert_eq!(scan(":".to_string()), CondAsn);
    }

    #[test]
    fn AsnOp() {
        let EqAsn = vec![Token::AsnOp(AsnOp::EqAsn)];
        let IncAsn = vec![Token::AsnOp(AsnOp::IncAsn)];
        let DecAsn = vec![Token::AsnOp(AsnOp::DecAsn)];
        let MultAsn = vec![Token::AsnOp(AsnOp::MultAsn)];
        let DivAsn = vec![Token::AsnOp(AsnOp::DivAsn)];
        let ModAsn = vec![Token::AsnOp(AsnOp::ModAsn)];
        let LShiftAsn = vec![Token::AsnOp(AsnOp::LShiftAsn)];
        let RShiftAsn = vec![Token::AsnOp(AsnOp::RShiftAsn)];
        let ANDAsn = vec![Token::AsnOp(AsnOp::ANDAsn)];
        let XORAsn = vec![Token::AsnOp(AsnOp::XORAsn)];
        let ORAsn = vec![Token::AsnOp(AsnOp::ORAsn)];

        assert_eq!(scan("=".to_string()), EqAsn);
        assert_eq!(scan("+=".to_string()), IncAsn);
        assert_eq!(scan("-=".to_string()), DecAsn);
        assert_eq!(scan("*=".to_string()), MultAsn);
        assert_eq!(scan("/=".to_string()), DivAsn);
        assert_eq!(scan("%=".to_string()), ModAsn);
        assert_eq!(scan("<<=".to_string()), LShiftAsn);
        assert_eq!(scan(">>=".to_string()), RShiftAsn);
        assert_eq!(scan("&=".to_string()), ANDAsn);
        assert_eq!(scan("^=".to_string()), XORAsn);
        assert_eq!(scan("|=".to_string()), ORAsn);
    }

    #[test]
    fn PostOp() {
        let Inc = vec![Token::PostOp(PostOp::Inc)];
        let Dec = vec![Token::PostOp(PostOp::Dec)];

        assert_eq!(scan("++".to_string()), Inc);
        assert_eq!(scan("--".to_string()), Dec);
    }

    #[test]
    fn combination_w_spaces() {
        let res1 = vec![
            Token::BinOp(BinOp::CondEq),
            Token::BinOp(BinOp::FieldDeref),
            Token::BinOp(BinOp::Modulo),
            Token::BinOp(BinOp::ShiftLeft),
            Token::AsnOp(AsnOp::LShiftAsn),
            Token::BinOp(BinOp::Greater),
            Token::BinOp(BinOp::Disequality),
            Token::BinOp(BinOp::Equality),
            Token::AsnOp(AsnOp::EqAsn),
            Token::BinOp(BinOp::LogicalOR),
            Token::AsnOp(AsnOp::IncAsn),
            Token::AsnOp(AsnOp::XORAsn),
            Token::PostOp(PostOp::Inc),
            Token::Sep(Sep::RCurly),
            Token::Esc(Esc::CarriageReturn),
            Token::Sep(Sep::LBracket),
            Token::Sep(Sep::SemiColon),
            Token::UnOp(UnOp::BitwiseNOT),
            Token::AsnOp(AsnOp::ModAsn),
        ];

        let program = vec![
            Token::Keyword(Keyword::Int),
            Token::Id(Id::Id("main".to_string())),
            Token::Sep(Sep::LParen),
            Token::Sep(Sep::RParen),
            Token::Sep(Sep::LCurly),
            Token::Id(Id::Id("printf".to_string())),
            Token::Sep(Sep::LParen),
            Token::StrLit(StrLit::StringLiteral("Hello world!".to_string())),
            Token::Sep(Sep::RParen),
            Token::Sep(Sep::SemiColon),
            Token::Keyword(Keyword::Bool),
            Token::Id(Id::Id("this_works".to_string())),
            Token::AsnOp(AsnOp::EqAsn),
            Token::Keyword(Keyword::True),
            Token::Sep(Sep::SemiColon),
            Token::Keyword(Keyword::Return),
            Token::Num(Num::DecNum(DecNum::DecNum(0))),
            Token::Sep(Sep::SemiColon),
            Token::Sep(Sep::RCurly),
        ];

        assert_eq!(
            scan("? -> % << <<= > != == = || += ^= ++ } \\r [ ; ~ %=".to_string()),
            res1
        );
        // OFF BY ONE ERROR FROM RECOGNIZING Strings/Nums is throwing us off here
        // Trouble shoot further
        assert_eq!(
            scan(
                r#"int main() {
                    printf("Hello world!");
                    bool this_works = true;
                    return 0;
                }"#
                .to_string()
            ),
            program
        );
    }

    #[test]
    fn combination_no_spaces() {
        let res1 = vec![
            Token::BinOp(BinOp::CondEq),
            Token::BinOp(BinOp::FieldDeref),
            Token::BinOp(BinOp::Modulo),
            Token::BinOp(BinOp::ShiftLeft),
            Token::AsnOp(AsnOp::LShiftAsn),
            Token::BinOp(BinOp::Greater),
            Token::BinOp(BinOp::Disequality),
            Token::BinOp(BinOp::Equality),
            Token::AsnOp(AsnOp::EqAsn),
            Token::BinOp(BinOp::LogicalOR),
            Token::AsnOp(AsnOp::IncAsn),
            Token::AsnOp(AsnOp::XORAsn),
            Token::PostOp(PostOp::Inc),
            Token::Sep(Sep::RCurly),
            Token::Esc(Esc::CarriageReturn),
            Token::Sep(Sep::LBracket),
            Token::Sep(Sep::SemiColon),
            Token::UnOp(UnOp::BitwiseNOT),
            Token::AsnOp(AsnOp::ModAsn),
        ];

        assert_eq!(scan("?->%<<<<=>!====||+=^=++}\\r[;~%=".to_string()), res1);
    }
}

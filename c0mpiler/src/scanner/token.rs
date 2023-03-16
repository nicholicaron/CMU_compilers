#![allow(dead_code)]
// Enumerate keywords?
#[derive(PartialEq, Debug)]
pub enum Token {
    Id(Id),
    Num(Num),
    StrLit(StrLit),
    ChrLit(ChrLit),
    LibLit(LibLit),
    // How to differentiate between SChar::Esc and CChar::Esc?
    SChar(SChar),
    CChar(CChar),
    Sep(Sep),
    UnOp(UnOp),
    BinOp(BinOp),
    AsnOp(AsnOp),
    PostOp(PostOp),
    Keyword(Keyword),
    Esc(Esc),
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
    Int,
    Bool,
    String,
    Char,
    Void,
    Struct,
    Typedef,
    If,
    Else,
    While,
    For,
    Continue,
    Break,
    Return,
    Assert,
    Error,
    True,
    False,
    Null,
    Alloc,
    AllocArray,
}

#[derive(PartialEq, Debug)]
pub enum Id {
    Id(String),
}

#[derive(PartialEq, Debug)]
pub enum Num {
    DecNum(DecNum),
    HexNum,
}

#[derive(PartialEq, Debug)]
pub enum DecNum {
    DecNum(u32),
}

#[derive(PartialEq, Debug)]
pub enum HexNum {
    // implicit conversion from base-16 to decimal
    HexNumber(u32),
}

#[derive(PartialEq, Debug)]
pub enum StrLit {
    StringLiteral(String),
}

#[derive(PartialEq, Debug)]
pub enum ChrLit {
    CharacterLiteral(char),
}

#[derive(PartialEq, Debug)]
pub enum LibLit {
    LibraryLiteral(String),
}

#[derive(PartialEq, Debug)]
pub enum SChar {
    Esc,
    Nchar,
}

#[derive(PartialEq, Debug)]
pub enum CChar {
    Esc,
    LChar,
}

// Any ASCII Char except "
#[derive(PartialEq, Debug)]
pub enum NChar {
    NChar(char),
}

// Any ASCII Char except >
#[derive(PartialEq, Debug)]
pub enum LChar {
    LChar(char),
}

#[derive(PartialEq, Debug)]
pub enum Esc {
    Alert,          // " \a "
    Backspace,      // " \b "
    FormfeedPgBrk,  // " \f "
    Newline,        // " \n "
    CarriageReturn, // " \r "
    HorizontalTab,  // " \t "
    VerticalTab,    // " \v "
    Backslash,      // " \\ "
    Apostrophe,     // " \' "
    DoubleQuote,    // " \" "
}

#[derive(PartialEq, Debug)]
pub enum Sep {
    LParen,    // '('
    RParen,    // ')'
    LBracket,  // '['
    RBracket,  // ']'
    LCurly,    // '{'
    RCurly,    // '}'
    Comma,     // ','
    SemiColon, // ';'
}

#[derive(PartialEq, Debug)]
pub enum UnOp {
    LogicalNOT, // '!'
    BitwiseNOT, // '~'
    UnaryMinus, // '-'
    Pointer,    // '*'
}

#[derive(PartialEq, Debug)]
pub enum BinOp {
    CondEq,      // '?'
    FieldSelect, // '.'
    FieldDeref,  // "->"
    IntTimes,    // '*'
    Divide,      // '/'
    Modulo,      // '%'
    Plus,        // '+'
    Minus,       // '-'
    ShiftLeft,   // "<<"
    Less,        // '<'
    LessEq,      // "<="
    ShiftRight,  // ">>"
    Greater,     // '>'
    GreaterEq,   // ">="
    Equality,    // "=="
    BitwiseAND,  // '&'
    LogicalAND,  // "&&"
    BitwiseXOR,  // '^'
    Disequality, // "!="
    BitwiseOR,   // '|'
    LogicalOR,   // "||"
    CondAsn,     // ":"
}

#[derive(PartialEq, Debug)]
pub enum AsnOp {
    EqAsn,     // '='
    IncAsn,    // "+="
    DecAsn,    // "-="
    MultAsn,   // "*="
    DivAsn,    // "/="
    ModAsn,    // "%="
    LShiftAsn, // "<<="
    RShiftAsn, // ">>="
    ANDAsn,    // "&="
    XORAsn,    // "^="
    ORAsn,     // "|="
}

#[derive(PartialEq, Debug)]
pub enum PostOp {
    Inc, // "++"
    Dec, // "--"
}

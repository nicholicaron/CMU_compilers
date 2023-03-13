#[derive(PartialEq)]
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
}

#[derive(PartialEq)]
enum Id {
    Identifier(String),
}

#[derive(PartialEq)]
enum Num {
    DecNum,
    HexNum,
}

#[derive(PartialEq)]
enum DecNum {
    DecNumber(u32),
}

#[derive(PartialEq)]
enum HexNum {
    // implicit conversion from base-16 to decimal
    HexNumber(u32),
}

#[derive(PartialEq)]
enum StrLit {
    StringLiteral(String),
}

#[derive(PartialEq)]
enum ChrLit {
    CharacterLiteral(char),
}

#[derive(PartialEq)]
enum LibLit {
    LibraryLiteral(String),
}

#[derive(PartialEq)]
enum SChar {
    Esc,
    Nchar,
}

#[derive(PartialEq)]
enum CChar {
    Esc,
    LChar,
}

// Any ASCII Char except "
#[derive(PartialEq)]
enum NChar {
    NChar(char),
}

// Any ASCII Char except >
#[derive(PartialEq)]
enum LChar {
    LChar(char),
}

#[derive(PartialEq)]
enum Esc {
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

#[derive(PartialEq)]
enum Sep {
    LParen,    // '('
    RParen,    // ')'
    LBracket,  // '['
    RBracket,  // ']'
    LCurly,    // '{'
    RCurly,    // '}'
    Comma,     // ','
    SemiColon, // ';'
}

#[derive(PartialEq)]
enum UnOp {
    LogicalNOT, // '!'
    BitwiseNOT, // '~'
    UnaryMinus, // '-'
    Pointer,    // '*'
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum PostOp {
    Inc, // "++"
    Dec, // "--"
}

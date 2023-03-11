pub enum Token {
    Id,
    Num,
    StrLit,
    ChrLit,
    LibLit,
    // How to differentiate between SChar::Esc and CChar::Esc?
    SChar,
    CChar,
    Sep,
    UnOp,
    BinOp,
    AsnOp,
    PostOp,
}

enum Id {
    Identifier(String),
}

enum Num {
    DecNum,
    HexNum,
}

enum DecNum {
    DecNumber(u32),
}

enum HexNum {
    // implicit conversion from base-16 to decimal
    HexNumber(u32),
}

enum StrLit {
    StringLiteral(String),
}

enum ChrLit {
    CharacterLiteral(char),
}

enum LibLit {
    LibraryLiteral(String),
}

enum SChar {
    Esc,
    Nchar,
}

enum CChar {
    Esc,
    LChar,
}

// Any ASCII Char except "
enum NChar {
    NChar(char),
}

// Any ASCII Char except >
enum LChar {
    LChar(char),
}

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

enum UnOp {
    LogicalNOT, // '!'
    BitwiseNOT, // '~'
    UnaryMinus, // '-'
    Pointer,    // '*'
}

enum BinOp {
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

enum AsnOp {
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

enum PostOp {
    PostInc, // "++"
    PostDec, // "--"
}

// For quick access, here's the R5RS Standard.
// https://conservatory.scheme.org/schemers/Documents/Standards/R5RS/HTML/

enum TokenKind {
    LParen,     // (
    RParen,     // )
    PLUS,       // +
    MINUS,      // -
    SLASH,      // /
    STAR,       // *
    PESO,       // $
    BANG,       // !
    Ampersand,  // &
    BSlash,     // \
    Colon,      // :
    LThan,      // <
    Equal,      // =
    GThan,      // >
    Question,   // ?
    Underscore, // _
    Tilde,      // ~
    Quote,      // '
    Comma,      // ,
    QuasiQuote, // `
    CommaAt,    // ,@
    HashLParen, // #(
    True,       // #t,
    False,      // #f
    Number(i64),
    Float(f64),
    Identifier(String),
    String(String),
    Char(char),
    Define,
    Lambda,
    Arrow,   // =>
    SetBang, // set!
    Begin,   // begin
    Cond,    // cond
    And,     // and
    Or,      // or
    LetRec,  // letrec
    Else,    // else
    If,
    Symbol(String),
    Eof,
    Ellipsis, // ...
}

struct Token {
    column: usize,
    line: usize,
    kind: TokenKind,
}

struct Lexer {
    source: String,
    column: usize,
    index: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            source: input,
            column: 1,
            line: 1,
            index: 0,
        }
    }

    pub fn lex() -> Vec<Token> {
        todo!();
    }

    fn is_at_end(&self) -> bool {
        self.source.len() >= self.index
    }

    fn peek() {
        todo!();
    }
}

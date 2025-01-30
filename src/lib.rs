// For quick access, here's the R5RS Standard.
// https://conservatory.scheme.org/schemers/Documents/Standards/R5RS/HTML/

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
struct Token {
    column: usize,
    line: usize,
    kind: TokenKind,
}

struct Lexer {
    source: Vec<char>,
    column: usize,
    index: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
            // I do this here because doing input.chars().nth() everytime I need
            // to index a character ends up being unperformant since it has to
            // rebuild the iterator everytime it's called.
            column: 1,
            line: 1,
            index: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut v = vec![];
        while !self.is_at_end() {
            v.push(self.scan_token());
        }
        v
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source.get(self.index + 1).copied().unwrap_or('\0')
    }

    fn bite(&self) -> char {
        self.source.get(self.index).copied().unwrap_or('\0')
    }

    fn scan_token(&mut self) -> Token {
        let kind = match self.bite() {
            '(' => TokenKind::LParen,
            '\0' => TokenKind::Eof,
            _ => todo!(),
        };

        let t = Token {
            column: self.column,
            line: self.line,
            kind,
        };

        self.index += 1;

        t
    }
}

mod tests {
    use super::*;

    #[test]
    fn lexer_canonical() {
        let mut l = Lexer::new("(+ 2 3)");
        let v = l.lex();
        assert_eq!(v[0].kind, TokenKind::LParen);
    }
}

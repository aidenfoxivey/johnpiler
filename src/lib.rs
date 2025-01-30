// For quick access, here's the R5RS Standard.
// https://conservatory.scheme.org/schemers/Documents/Standards/R5RS/HTML/

#[derive(Debug, PartialEq)]
enum TokenKind {
    LParen,     // (
    RParen,     // )
    Plus,       // +
    Minus,      // -
    Slash,      // /
    Star,       // *
    Peso,       // $
    Bang,       // !
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
            if let Some(tok) = self.scan_token() {
                v.push(tok);
            }
        }
        v
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source.get(self.index + 1).copied().unwrap_or('\0')
    }

    fn bite(&mut self) -> char {
        let c = self.source.get(self.index).copied().unwrap_or('\0');
        self.index += 1;
        c
    }

    fn make_token(&mut self, kind: TokenKind) -> Option<Token> {
        let token: Token = Token {
            column: self.column,
            line: self.line,
            kind,
        };
        self.index += 1;
        Some(token)
    }

    fn scan_token(&mut self) -> Option<Token> {
        match self.bite() {
            '(' => self.make_token(TokenKind::LParen),
            ')' => self.make_token(TokenKind::RParen),
            '-' => self.make_token(TokenKind::Minus),
            '+' => self.make_token(TokenKind::Plus),
            '\0' => self.make_token(TokenKind::Eof),
            '#' => {
                let next = self.bite();
                if next == 't' {
                    return self.make_token(TokenKind::True);
                } else if next == 'f' {
                    return self.make_token(TokenKind::False);
                } else if next == '(' {
                    return self.make_token(TokenKind::HashLParen);
                }

                todo!();
            }
            ' ' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_canonical() {
        let v = Lexer::new("(+ 2 3)").lex();
        assert_eq!(v[0].kind, TokenKind::LParen);
    }

    #[test]
    fn lexer_true() {
        // Sorry, I know this isn't a valid AST for scheme. (At least I don't
        // this it is.)
        let v = Lexer::new("( #t #f )").lex();
        assert_eq!(v.len(), 4);
        assert_eq!(v[0].kind, TokenKind::LParen);
        assert_eq!(v[1].kind, TokenKind::True);
        assert_eq!(v[2].kind, TokenKind::False);
        assert_eq!(v[3].kind, TokenKind::RParen);
    }
}

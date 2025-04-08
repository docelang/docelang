#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Let,
    Fn,
    Ident,
    Number,
    Equals,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Pipe,
    Arrow,
    Colon,
    Comma,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub position: usize,
}

impl Token {
    pub fn new(kind: TokenKind, text: &str, position: usize) -> Self {
        Self {
            kind,
            text: text.to_string(),
            position,
        }
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < chars.len() {
        let ch = chars[pos];

        match ch {
            c if c.is_whitespace() => {
                pos += 1;
            }

            '=' => {
                tokens.push(Token::new(TokenKind::Equals, "=", pos));
                pos += 1;
            }

            '{' => {
                tokens.push(Token::new(TokenKind::LBrace, "{", pos));
                pos += 1;
            }

            '}' => {
                tokens.push(Token::new(TokenKind::RBrace, "}", pos));
                pos += 1;
            }

            '(' => {
                tokens.push(Token::new(TokenKind::LParen, "(", pos));
                pos += 1;
            }

            ')' => {
                tokens.push(Token::new(TokenKind::RParen, ")", pos));
                pos += 1;
            }

            ':' => {
                tokens.push(Token::new(TokenKind::Colon, ":", pos));
                pos += 1;
            }

            ',' => {
                tokens.push(Token::new(TokenKind::Comma, ",", pos));
                pos += 1;
            }

            '|' => {
                tokens.push(Token::new(TokenKind::Pipe, "|", pos));
                pos += 1;
            }

            '-' => {
                if pos + 1 < chars.len() && chars[pos + 1] == '>' {
                    tokens.push(Token::new(TokenKind::Arrow, "->", pos));
                    pos += 2;
                } else {
                    pos += 1; // bisa ditangani sebagai minus nanti
                }
            }

            c if c.is_ascii_digit() => {
                let start = pos;
                while pos < chars.len() && chars[pos].is_ascii_digit() {
                    pos += 1;
                }
                let text: String = chars[start..pos].iter().collect();
                tokens.push(Token::new(TokenKind::Number, &text, start));
            }

            c if c.is_alphabetic() || c == '_' => {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    pos += 1;
                }
                let text: String = chars[start..pos].iter().collect();
                let kind = match text.as_str() {
                    "let" => TokenKind::Let,
                    "fn" => TokenKind::Fn,
                    _ => TokenKind::Ident,
                };
                tokens.push(Token::new(kind, &text, start));
            }

            _ => {
                pos += 1; // skip unknown character
            }
        }
    }

    tokens.push(Token::new(TokenKind::EOF, "", pos));
    tokens
}

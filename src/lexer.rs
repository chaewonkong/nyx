use std::iter::Peekable;

use crate::token::{Span, Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a str,
    chars: Peekable<std::str::CharIndices<'a>>,
    pos: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src,
            chars: src.char_indices().peekable(),
            pos: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        if let Some((_, c)) = self.chars.peek() {
            return Some(*c);
        }
        return None;
    }

    // 다음 글자를 먹고 위치를 전진시킨다
    fn bump(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.chars.next();
            self.pos += 1;
            return Some(c);
        }

        None
    }

    fn lex_number(&mut self) -> TokenKind {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.bump();
                s.push(c);
            } else {
                break;
            }
        }

        TokenKind::Int(s.parse().unwrap()) // TODO: handle overflow
    }

    fn lex_ident(&mut self) -> TokenKind {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.bump();
                s.push(c);
            } else {
                break;
            }
        }

        TokenKind::Ident(s)
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c == ' ' {
                self.bump();
            } else {
                break;
            }
        }
        let start = self.pos;
        let kind = match self.peek() {
            None => TokenKind::Eof,
            Some('\n') => {
                self.bump();
                TokenKind::Newline
            }
            Some('=') => {
                self.bump();
                TokenKind::Eq
            }
            Some('+') => {
                self.bump();
                TokenKind::Plus
            }
            Some('-') => {
                self.bump();
                TokenKind::Minus
            }
            Some('*') => {
                self.bump();
                TokenKind::Aesterisk
            }
            Some('/') => {
                self.bump();
                TokenKind::Slash
            }
            Some(c) if c.is_ascii_digit() => self.lex_number(),
            Some(c) if c == '_' || c.is_alphabetic() => self.lex_ident(),
            Some(c) => {
                self.bump();
                TokenKind::Unknown(c)
            }
        };

        Token {
            kind,
            span: Span {
                start: start,
                end: self.pos,
            },
        }
    }

    pub fn text(&self, span: Span) -> &'a str {
        &self.src[span.start as usize..span.end as usize]
    }
}

// helper for test
fn kinds(src: &str) -> Vec<TokenKind> {
    let mut lx = Lexer::new(src);
    let mut out = Vec::new();

    loop {
        let t = lx.next_token();
        let done = t.kind == TokenKind::Eof;
        out.push(t.kind);

        if done {
            break;
        }
    }
    out
}

fn tokens(src: &str) -> Vec<Token> {
    let mut lx = Lexer::new(src);
    let mut out = Vec::new();

    loop {
        let t = lx.next_token();
        let done = t.kind == TokenKind::Eof;
        out.push(t);

        if done {
            break;
        }
    }
    out
}

#[test]
fn lexes_numbers() {
    assert_eq!(kinds("0"), vec![TokenKind::Int(0), TokenKind::Eof]);
    assert_eq!(kinds("42"), vec![TokenKind::Int(42), TokenKind::Eof]);

    assert_eq!(
        kinds("12+34"),
        vec![
            TokenKind::Int(12),
            TokenKind::Plus,
            TokenKind::Int(34),
            TokenKind::Eof
        ]
    );

    assert_eq!(
        kinds("12abc"),
        vec![
            TokenKind::Int(12),
            TokenKind::Ident(String::from("abc")),
            TokenKind::Eof
        ]
    );
}

#[test]
fn number_span_covers_all_digits() {
    let ts = tokens("  123");
    assert_eq!(ts[0].span, Span { start: 2, end: 5 });
}

#[test]
fn lexes_ident() {
    assert_eq!(
        kinds("_x"),
        vec![TokenKind::Ident("_x".into()), TokenKind::Eof]
    );
    assert_eq!(
        kinds("x1"),
        vec![TokenKind::Ident("x1".into()), TokenKind::Eof]
    );
    assert_eq!(
        kinds("1abc"),
        vec![
            TokenKind::Int(1),
            TokenKind::Ident("abc".into()),
            TokenKind::Eof
        ],
    );
}

#[test]
fn number_span_covers_all_idents() {
    let ts = tokens("  abc");
    assert_eq!(ts[0].span, Span { start: 2, end: 5 });
}

#[test]
fn bump_updates_chars_and_pos() {
    let mut lx = Lexer::new("abcde");
    let c = lx.bump();

    assert_eq!(lx.pos, 1);
    assert_eq!(lx.peek(), Some('b'));
    assert_eq!(c, Some('a'))
}

#[test]
fn bump_at_end_returns_none() {
    let mut lx = Lexer::new("");
    assert_eq!(lx.bump(), None);
    assert_eq!(lx.pos, 0);
}

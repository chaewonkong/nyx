use std::iter::Peekable;

use crate::toekn::{Token, TokenKind};

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

    pub fn next_token(&mut self) -> Token {
        todo!()
    }
}

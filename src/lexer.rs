use std::{
    iter::{Copied, Peekable},
    slice::Iter,
};

type ByteIterator<'s> = Copied<Iter<'s, u8>>;
pub struct Lexer<'s> {
    _content: &'s [u8],
    stream: Peekable<ByteIterator<'s>>,
    offset: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(_content: &'s [u8]) -> Self {
        let stream = _content.iter().copied().peekable();
        Self {
            _content,
            stream,
            offset: 0,
        }
    }
}

impl<'s> Lexer<'s> {
    #[inline]
    const fn is_valid_bf_token(ch: &u8) -> bool {
        matches!(*ch, b'+' | b'-' | b'<' | b'>' | b',' | b'.' | b'[' | b']')
    }
    #[inline]
    fn skip_invalid_token(&mut self) {
        while let Some(s) = self.stream.peek() {
            if !Self::is_valid_bf_token(s) {
                self.offset += 1;
                self.stream.next();
            } else {
                break;
            }
        }
    }

    #[inline]
    pub fn next_token(&mut self) -> Option<OpKind> {
        self.skip_invalid_token();
        let tok = self.stream.next()?;
        self.offset += 1;
        Some(OpKind::from_u8(tok))
    }
    pub const fn pos(&self) -> usize {
        self.offset
    }
}
impl Iterator for Lexer<'_> {
    type Item = OpKind;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpKind {
    Inc = b'+',
    Dec = b'-',
    Left = b'<',
    Right = b'>',
    Output = b'.',
    Input = b',',
    JumpIfZero = b'[',
    JumpIfNonzero = b']',
}
impl OpKind {
    #[inline]
    const fn from_u8(ch: u8) -> Self {
        match ch {
            b'+' => Self::Inc,
            b'-' => Self::Dec,
            b'<' => Self::Left,
            b'>' => Self::Right,
            b'.' => Self::Output,
            b',' => Self::Input,
            b'[' => Self::JumpIfZero,
            b']' => Self::JumpIfNonzero,
            _ => unreachable!(),
        }
    }
}

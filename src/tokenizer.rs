// use crate::ref_ring::RefRing;
use memchr::memchr;
use memchr::memmem::Finder;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};

static FINDER_END_OF_COMMENT: Lazy<Finder<'static>> = Lazy::new(|| Finder::new("*/"));

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum TokenType {
  OpenParentheses,
  CloseParentheses,
  Space,
  Word,
  String,
  Div,
  Comment,
  UnicodeRange,
  Unknown,
}

impl std::fmt::Display for TokenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TokenType::OpenParentheses => write!(f, "("),
      TokenType::CloseParentheses => write!(f, ")"),
      TokenType::Space => write!(f, "space"),
      TokenType::Word => write!(f, "word"),
      TokenType::String => write!(f, "string"),
      TokenType::Div => write!(f, "div"),
      TokenType::Comment => write!(f, "comment"),
      TokenType::UnicodeRange => write!(f, "unicode-range"),
      TokenType::Unknown => write!(f, "unknown"),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// quarter nary tuple (token_type, content, start_offset, end_offset), content is a slice with range `start_offset..end_offset`
pub struct Token<'a>(pub TokenType, pub &'a str, pub usize, pub usize);

impl<'a> Token<'a> {
  pub fn new(kind: TokenType, content: &'a str, pos: usize, next: usize) -> Token {
    Token(kind, content, pos, next)
  }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
  pub value: &'a str,
  length: usize,
  pos: RefCell<usize>,
  // buffer: RefCell<RefRing<'a>>,
}

impl<'a> Tokenizer<'a> {
  pub fn new(value: &'a str) -> Tokenizer<'a> {
    let length = value.len();
    Tokenizer {
      value,
      length,
      pos: RefCell::new(0),
      // buffer: RefCell::new(Default::default()),
    }
  }

  // #[inline]
  // fn push(&self, t: &'a str) {
  //   self.buffer.borrow_mut().push(t);
  // }

  #[inline]
  pub fn position(&self) -> usize {
    *self.pos.borrow()
  }

  pub fn end_of_file(&self) -> bool {
    self.position() >= self.length
  }

  #[inline]
  fn pos_plus_one(&self) {
    self.pos.replace_with(|it| *it + 1);
  }

  pub fn next_token(&self) -> Token<'a> {
    let mut code = char_code_at(self.value, self.position());

    let current_token: Token;

    match code {
      0..=32 => {
        let mut next = self.position();
        loop {
          next += 1;
          code = char_code_at(self.value, next);
          if !matches!(code, 1..=32) {
            break;
          }
        }

        current_token = Token(
          TokenType::Space,
          self.value[self.position()..next].into(),
          self.position(),
          next,
        );

        self.pos.replace(next);
      }
      quote @ (b'\'' | b'"') => {
        let mut next = self.position();
        loop {
          let mut escaped = false;
          next = match index_of_byte(self.value, quote, next + 1) {
            Some(next) => {
              let mut escape_pos = next;
              while char_code_at(self.value, escape_pos - 1) == b'\\' {
                escape_pos -= 1;
                escaped = !escaped;
              }
              next
            }
            None => self.length - 1,
          };

          if !escaped {
            break;
          }
        }

        current_token = Token(
          TokenType::String,
          sub_str(self.value, self.position(), next + 1),
          self.position(),
          next + 1,
        );
        self.pos.replace(next + 1);
      }
      b'/' if char_code_at(self.value, self.position() + 1) == b'*' => {
        let next = index_of_end_comment(self.value, self.position()).unwrap_or(self.length);
        current_token = Token(
          TokenType::Comment,
          sub_str(self.value, self.position() + 2, next),
          self.position(),
          next + 1,
        );
        self.pos.replace(next + 2);
      }
      b'/' | b',' | b':' | b'(' | b')' => {
        let start = self.position();
        current_token = Token(get_token_type(code), get_str(code), start, start + 1);
        self.pos_plus_one();
      }
      _ => {
        let mut next = index_of_word_end(self.value, self.position());
        if next == self.position() {
          next += 1;
        }
        let content = sub_str(self.value, self.position(), next);
        if unicode_range(content) {
          current_token = Token::new(TokenType::UnicodeRange, content, self.position(), next);
        } else {
          current_token = Token::new(TokenType::Word, content, self.position(), next);
        }
        self.pos.replace(next);
      }
    }

    current_token
  }
}

#[inline]
fn index_of_end_comment(value: &str, from_index: usize) -> Option<usize> {
  let (_, last) = value.split_at(from_index);
  FINDER_END_OF_COMMENT
    .find(last.as_bytes())
    .map(|v| v + from_index)
}

#[inline]
fn index_of_word_end(s: &str, start: usize) -> usize {
  let bytes = s.as_bytes();
  let mut i = start;
  let len = bytes.len();

  while i < len {
    match bytes[i] {
      0..=32 | b'\'' | b'"' | b',' | b':' | b'/' | b'*' | b'(' | b')' => {
        return i;
      }
      b'\\' => i += 2,
      _ => i += 1,
    };
  }
  i
}

#[inline]
fn unicode_range(s: &str) -> bool {
  if s.len() < 3 {
    return false;
  }

  let mut iter = s.as_bytes().iter();
  matches!(iter.next(), Some(b'u' | b'U'))
    && matches!(iter.next(), Some(b'+'))
    && iter.all(|x| matches!(x, b'a'..=b'f' |b'A'..=b'F' | b'0'..=b'9' | b'?' | b'-'))
}

#[inline]
fn index_of_byte(value: &str, search_value: u8, from_index: usize) -> Option<usize> {
  let (_, last) = value.split_at(from_index);
  memchr(search_value, last.as_bytes()).map(|v| v + from_index)
}

#[inline]
fn sub_str(s: &str, start: usize, end: usize) -> &str {
  if end + 1 > s.len() {
    // Safety: NEVER out-of-bounds
    unsafe { s.get_unchecked(start..) }
  } else {
    // Safety: NEVER out-of-bounds
    unsafe { s.get_unchecked(start..end) }
  }
}

#[inline]
fn char_code_at(s: &str, n: usize) -> u8 {
  if n >= s.len() {
    b'\0'
  } else {
    s.as_bytes()[n]
  }
}

/// SAFETY: YOU SHOULD NEVER CALL THIS FUNCTION WITH THE PARAM OTHER THAN THESE BELOW.
const fn get_str(ch: u8) -> &'static str {
  match ch {
    b'(' => "(",
    b')' => ")",
    b'/' => "/",
    b',' => ",",
    b':' => ":",
    _ => unreachable!(),
  }
}

/// SAFETY: YOU SHOULD NEVER CALL THIS FUNCTION WITH THE PARAM OTHER THAN THESE BELOW.
const fn get_token_type(ch: u8) -> TokenType {
  match ch {
    b'(' => TokenType::OpenParentheses,
    b')' => TokenType::CloseParentheses,
    b'/' => TokenType::Div,
    b',' => TokenType::Div,
    b':' => TokenType::Div,
    _ => unreachable!(),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_char_code_at() {
    let s = "0123456789abc";
    assert_eq!(char_code_at(s, 0), b'0');
    assert_eq!(char_code_at(s, 1), b'1');
    assert_eq!(char_code_at(s, 100), 0);
  }

  #[test]
  fn test_sub_str() {
    let s = "0123456789abc";
    assert_eq!(sub_str(s, 0, 0), "");
    assert_eq!(sub_str(s, 1, 3), "12");
    assert_eq!(sub_str(s, 10, 13), "abc");
    assert_eq!(sub_str(s, 10, 100), "abc");
  }
}

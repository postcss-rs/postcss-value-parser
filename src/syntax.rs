use crate::tokenizer::{Token, Tokenizer};

pub(crate) struct Lexer<'a> {
  inner: Tokenizer<'a>,
}

impl<'a> Lexer<'a> {
  pub(crate) fn new(value: &'a str) -> Self {
    Self {
      inner: Tokenizer::new(value),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if !self.inner.end_of_file() {
      let token = self.inner.next_token();
      Some(token)
    } else {
      None
    }
  }
}

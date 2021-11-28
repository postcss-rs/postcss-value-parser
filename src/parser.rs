use crate::node::{self, Node, Word};
use crate::syntax::Lexer;
use crate::tokenizer::{Token, TokenType};
use std::borrow::Cow;
use std::iter::Peekable;

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  value: &'a str,
  pos: usize,
}

impl<'a> Parser<'a> {
  pub fn new(value: &'a str) -> Self {
    Self {
      lexer: Lexer::new(value).peekable(),
      value,
      pos: 0,
    }
  }

  pub fn parse(mut self) -> Vec<Node<'a>> {
    let mut nodes: Vec<Node<'a>> = vec![];
    while let Some(syntax) = self.peek() {
      match syntax {
        (TokenType::Space, _) => {
          nodes.push(self.parse_space().into());
        }
        (TokenType::Comment, _) => {
          nodes.push(self.parse_comment().into());
        }
        (TokenType::String, _) => {
          nodes.push(self.parse_string().into());
        }
        (TokenType::Div, _) => {
          nodes.push(self.parse_div().into());
        }
        (TokenType::UnicodeRange, _) => {
          nodes.push(self.parse_unicode_range().into());
        }
        (TokenType::Word, _) => {
          let word = self.bump();
          if matches!(self.peek(), Some((TokenType::OpenParentheses, ..))) {
            self.bump();
            nodes.push(self.parse_function(word.1, word.2).into());
          } else {
            nodes.push(
              node::Word {
                source_index: word.2,
                value: Cow::Borrowed(word.1),
              }
              .into(),
            );
          }
        }
        (TokenType::OpenParentheses, _) => {
          let token = self.bump();
          nodes.push(self.parse_function("", token.2).into());
        }
        (TokenType::CloseParentheses, _) => {
          let token = self.bump();
          let mut end = token.3;
          while matches!(
            self.peek(),
            Some((TokenType::CloseParentheses | TokenType::Word, _))
          ) {
            end = self.bump().3;
          }
          nodes.push(
            Word {
              source_index: token.2,
              value: Cow::Borrowed(&self.value[token.2..end]),
            }
            .into(),
          );
        }
        _ => {
          self.bump();
        }
      };
    }
    nodes
  }

  #[inline]
  pub fn parse_comment(&mut self) -> node::Comment<'a> {
    let token = self.bump();
    node::Comment {
      source_index: token.2,
      value: Cow::Borrowed(token.1),
    }
  }

  #[inline]
  pub fn parse_space(&mut self) -> node::Space<'a> {
    let token = self.bump();
    node::Space {
      source_index: token.2,
      value: Cow::Borrowed(token.1),
    }
  }

  #[inline]
  pub fn parse_div(&mut self) -> node::Div<'a> {
    let token = self.bump();
    node::Div {
      source_index: token.2,
      value: Cow::Borrowed(token.1),
    }
  }

  #[inline]
  pub fn parse_word(&mut self) -> node::Word<'a> {
    let token = self.bump();
    node::Word {
      source_index: token.2,
      value: Cow::Borrowed(token.1),
    }
  }

  #[inline]
  pub fn parse_string(&mut self) -> node::String<'a> {
    let token = self.bump();
    node::String {
      source_index: token.2,
      value: Cow::Borrowed(token.1),
    }
  }

  #[inline]
  pub fn parse_unicode_range(&mut self) -> node::UnicodeRange<'a> {
    let token = self.bump();
    node::UnicodeRange {
      source_index: token.2,
      value: Cow::Borrowed(token.1),
    }
  }

  #[inline]
  pub fn parse_url_word(&mut self) -> node::Word<'a> {
    let token = self.bump();
    let mut end = token.3;
    while matches!(
      self.peek(),
      Some((TokenType::Word, _) | (TokenType::Div, "/" | ":"))
    ) {
      end = self.bump().3;
    }
    Word {
      source_index: token.2,
      value: Cow::Borrowed(&self.value[token.2..end]),
    }
  }

  #[inline]
  pub fn parse_function(&mut self, name: &'a str, index: usize) -> node::Function<'a> {
    let mut nodes: Vec<Node<'a>> = vec![];
    while let Some(syntax) = self.peek() {
      match syntax {
        (TokenType::Space, _) => {
          nodes.push(self.parse_space().into());
        }
        (TokenType::Comment, _) => {
          nodes.push(self.parse_comment().into());
        }
        (TokenType::String, _) => {
          nodes.push(self.parse_string().into());
        }
        (TokenType::Div, "/") if name == "url" => {
          nodes.push(self.parse_url_word().into());
        }
        (TokenType::Div, "/") if name != "calc" => {
          nodes.push(self.parse_div().into());
        }
        (TokenType::Div, "," | ":") => {
          nodes.push(self.parse_div().into());
        }
        (TokenType::Div, _) => {
          nodes.push(self.parse_word().into());
        }
        (TokenType::UnicodeRange, _) => {
          nodes.push(self.parse_unicode_range().into());
        }
        (TokenType::Word, _) if name == "url" => {
          nodes.push(self.parse_url_word().into());
        }
        (TokenType::Word, _) => {
          let word = self.bump();
          if matches!(self.peek(), Some((TokenType::OpenParentheses, ..))) {
            self.bump();
            nodes.push(self.parse_function(word.1, word.2).into());
          } else {
            nodes.push(
              node::Word {
                source_index: word.2,
                value: Cow::Borrowed(word.1),
              }
              .into(),
            );
          }
        }
        (TokenType::OpenParentheses, _) => {
          let token = self.bump();
          nodes.push(self.parse_function("", token.2).into());
        }
        (TokenType::CloseParentheses, _) => {
          self.bump();
          break;
        }
        _ => {
          self.bump();
        }
      };
    }

    node::Function {
      source_index: index,
      value: Cow::Borrowed(name),
      nodes,
    }
  }

  pub fn peek(&mut self) -> Option<(TokenType, &'a str)> {
    self.lexer.peek().map(|token| (token.0, token.1))
  }

  pub fn bump(&mut self) -> Token<'a> {
    let token = self.lexer.next().unwrap();
    self.pos = token.3;
    token
  }
}

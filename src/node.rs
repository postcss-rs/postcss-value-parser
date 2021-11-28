use std::borrow::Cow;
use std::convert::From;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Word<'a> {
  pub source_index: usize,
  pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Space<'a> {
  pub source_index: usize,
  pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Comment<'a> {
  pub source_index: usize,
  pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnicodeRange<'a> {
  pub source_index: usize,
  pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Div<'a> {
  pub source_index: usize,
  pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct String<'a> {
  pub source_index: usize,
  pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function<'a> {
  pub source_index: usize,
  pub value: Cow<'a, str>,
  pub nodes: Vec<Node<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node<'a> {
  Word(Word<'a>),
  Space(Space<'a>),
  Comment(Comment<'a>),
  UnicodeRange(UnicodeRange<'a>),
  Div(Div<'a>),
  String(String<'a>),
  Function(Function<'a>),
}

impl<'a> From<Word<'a>> for Node<'a> {
  fn from(item: Word<'a>) -> Self {
    Node::Word(item)
  }
}

impl<'a> From<Space<'a>> for Node<'a> {
  fn from(item: Space<'a>) -> Self {
    Node::Space(item)
  }
}

impl<'a> From<Comment<'a>> for Node<'a> {
  fn from(item: Comment<'a>) -> Self {
    Node::Comment(item)
  }
}

impl<'a> From<UnicodeRange<'a>> for Node<'a> {
  fn from(item: UnicodeRange<'a>) -> Self {
    Node::UnicodeRange(item)
  }
}

impl<'a> From<Div<'a>> for Node<'a> {
  fn from(item: Div<'a>) -> Self {
    Node::Div(item)
  }
}

impl<'a> From<String<'a>> for Node<'a> {
  fn from(item: String<'a>) -> Self {
    Node::String(item)
  }
}

impl<'a> From<Function<'a>> for Node<'a> {
  fn from(item: Function<'a>) -> Self {
    Node::Function(item)
  }
}

pub trait ClosableNode {
  /// Whether the parsed CSS value ended before the node was properly closed
  fn unclosed(&self) -> bool;
}

pub trait AdjacentAwareNode<'a> {
  /// The token at the start of the node
  fn before(&self) -> Cow<'a, str>;
  /// The token at the end of the node
  fn after(&self) -> Cow<'a, str>;
}

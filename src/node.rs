use std::borrow::Cow;

pub struct Word<'a> {
  pub source_index: u32,
  pub value: Cow<'a, str>,
}

pub struct Space<'a> {
  pub source_index: u32,
  pub value: Cow<'a, str>,
}

pub struct Comment<'a> {
  pub source_index: u32,
  pub value: Cow<'a, str>,
}

pub struct UnicodeRange<'a> {
  pub source_index: u32,
  pub value: Cow<'a, str>,
}

pub struct Div<'a> {
  pub source_index: u32,
  pub value: Cow<'a, str>,
}

pub struct String<'a> {
  pub source_index: u32,
  pub value: Cow<'a, str>,
}

pub struct Function<'a> {
  pub source_index: u32,
  pub value: Cow<'a, str>,
  pub nodes: Vec<Node<'a>>,
}

pub enum Node<'a> {
  Word(Word<'a>),
  Space(Space<'a>),
  Comment(Comment<'a>),
  UnicodeRange(UnicodeRange<'a>),
  Div(Div<'a>),
  String(String<'a>),
  Function(Function<'a>),
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

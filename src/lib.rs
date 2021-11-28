mod parser;

pub mod node;
pub mod ref_ring;
pub mod syntax;
pub mod tokenizer;
pub mod unit;

use node::Node;
use parser::Parser;

pub fn parse<'a>(value: &'a str) -> Vec<Node<'a>> {
  Parser::new(value).parse()
}

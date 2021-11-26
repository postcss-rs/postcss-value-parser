pub mod node;
pub mod ref_ring;
pub mod tokenizer;
pub mod unit;

use tokenizer::Tokenizer;

fn main() {
  let value = "abc";
  let processor = Tokenizer::new(value);
  while !processor.end_of_file() {
    println!("{:?}", processor.next_token());
  }
}

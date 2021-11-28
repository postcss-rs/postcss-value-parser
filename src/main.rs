use postcss_value_parser::parse;
use postcss_value_parser::tokenizer::Tokenizer;

fn main() {
  let value = "url( /gfx/img/bg.jpg ";
  let processor = Tokenizer::new(value);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token());
  }
  println!("tokens:\n{:#?}", tokens);

  let value = "url( /gfx/img/bg.jpg ";
  println!("parser:\n{:#?}", parse(value));
}

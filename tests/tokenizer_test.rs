use postcss_value_parser::tokenizer::TokenType::*;
use postcss_value_parser::tokenizer::{Token, Tokenizer};

fn tokenize(value: &str) -> Vec<Token> {
  let processor = Tokenizer::new(value);
  let mut tokens = vec![];
  while !processor.end_of_file() {
    tokens.push(processor.next_token())
  }
  tokens
}

fn run(value: &str, tokens: Vec<Token>) {
  assert_eq!(tokenize(value), tokens);
}

#[test]
fn tokenizes_empty_file() {
  run("", vec![]);
}

#[test]
fn tokenizes_space() {
  run("\r\n \u{c}\t", vec![Token(Space, "\r\n \u{c}\t", 0, 5)]);
}

#[test]
fn tokenizes_word() {
  run("ab", vec![Token(Word, "ab", 0, 2)]);
}

#[test]
fn tokenizes_should_process_escaped_parentheses_open() {
  run("\\(", vec![Token(Word, "\\(", 0, 2)]);
}

#[test]
fn tokenizes_should_process_escaped_parentheses_close() {
  run("\\)", vec![Token(Word, "\\)", 0, 2)]);
}

#[test]
fn tokenizes_should_process_escaped_parentheses_both() {
  run("\\(\\)", vec![Token(Word, "\\(\\)", 0, 4)]);
}

#[test]
fn tokenizes_should_process_escaped_parentheses_both_with_space() {
  run(
    "\\( \\)",
    vec![
      Token(Word, "\\(", 0, 2),
      Token(Space, " ", 2, 3),
      Token(Word, "\\)", 3, 5),
    ],
  );
}

#[test]
fn tokenizes_should_process_space_in_parentheses() {
  run(
    "( )",
    vec![
      Token(OpenParentheses, "(", 0, 1),
      Token(Space, " ", 1, 2),
      Token(CloseParentheses, ")", 2, 3),
    ],
  );
}

#[test]
fn tokenizes_should_process_word_in_parentheses() {
  run(
    "( | )",
    vec![
      Token(OpenParentheses, "(", 0, 1),
      Token(Space, " ", 1, 2),
      Token(Word, "|", 2, 3),
      Token(Space, " ", 3, 4),
      Token(CloseParentheses, ")", 4, 5),
    ],
  );
}

#[test]
fn tokenizes_should_process_nested_parentheses() {
  run(
    "((()))",
    vec![
      Token(OpenParentheses, "(", 0, 1),
      Token(OpenParentheses, "(", 1, 2),
      Token(OpenParentheses, "(", 2, 3),
      Token(CloseParentheses, ")", 3, 4),
      Token(CloseParentheses, ")", 4, 5),
      Token(CloseParentheses, ")", 5, 6),
    ],
  );
}

#[test]
fn tokenizes_should_process_divider_slash() {
  run("/", vec![Token(Div, "/", 0, 1)]);
}

#[test]
fn tokenizes_should_process_divider_colon() {
  run(":", vec![Token(Div, ":", 0, 1)]);
}

#[test]
fn tokenizes_should_process_divider_comma() {
  run(",", vec![Token(Div, ",", 0, 1)]);
}

#[test]
fn tokenizes_should_process_complex_divider() {
  run(
    " , ",
    vec![
      Token(Space, " ", 0, 1),
      Token(Div, ",", 1, 2),
      Token(Space, " ", 2, 3),
    ],
  );
}

#[test]
fn tokenizes_should_process_divider_in_parentheses() {
  run(
    "( , )",
    vec![
      Token(OpenParentheses, "(", 0, 1),
      Token(Space, " ", 1, 2),
      Token(Div, ",", 2, 3),
      Token(Space, " ", 3, 4),
      Token(CloseParentheses, ")", 4, 5),
    ],
  );
}

#[test]
fn tokenizes_should_process_two_spaced_divider() {
  run(
    " , : ",
    vec![
      Token(Space, " ", 0, 1),
      Token(Div, ",", 1, 2),
      Token(Space, " ", 2, 3),
      Token(Div, ":", 3, 4),
      Token(Space, " ", 4, 5),
    ],
  );
}

#[test]
fn tokenizes_should_process_empty_double_quoted_strings() {
  run("\"\"", vec![Token(String, "\"\"", 0, 2)]);
}

#[test]
fn tokenizes_should_process_empty_double_quoted_strings_unclosed() {
  run("\"", vec![Token(String, "\"", 0, 1)]);
}

#[test]
fn tokenizes_should_process_empty_single_quoted_strings() {
  run("''", vec![Token(String, "''", 0, 2)]);
}

#[test]
fn tokenizes_should_process_escaped_double_quotes() {
  run(
    "\"word\\\"word\"",
    vec![Token(String, "\"word\\\"word\"", 0, 12)],
  );
}

#[test]
fn tokenizes_should_process_escaped_single_quotes() {
  run("'word\\'word'", vec![Token(String, "'word\\'word'", 0, 12)]);
}

#[test]
fn tokenizes_should_process_single_quotes_inside_double_quotes() {
  run(
    "\"word'word\"",
    vec![Token(String, "\"word\'word\"", 0, 11)],
  );
}

#[test]
fn tokenizes_should_process_double_quotes_inside_single_quotes() {
  run("'word\"word'", vec![Token(String, "'word\"word'", 0, 11)]);
}

#[test]
fn tokenizes_should_process_unclosed_quotes() {
  run("\"word", vec![Token(String, "\"word", 0, 5)]);
}

#[test]
fn tokenizes_should_process_unclosed_quotes_with_ended_backslash() {
  run("\"word\\", vec![Token(String, "\"word\\", 0, 6)]);
}

#[test]
fn tokenizes_should_process_quoted_strings_and_words() {
  run(
    "word1'string'word2",
    vec![
      Token(Word, "word1", 0, 5),
      Token(String, "'string'", 5, 13),
      Token(Word, "word2", 13, 18),
    ],
  );
}

#[test]
fn tokenizes_should_process_escaped_symbols_as_words() {
  run(
    " \\\"word\\'\\ \\\t ",
    vec![
      Token(Space, " ", 0, 1),
      Token(Word, "\\\"word\\'\\ \\\t", 1, 13),
      Token(Space, " ", 13, 14),
    ],
  );
}

#[test]
fn tokenizes_should_process_font_value() {
  run(
    "bold italic 12px \t /3 'Open Sans', Arial, \"Helvetica Neue\", sans-serif",
    vec![
      Token(Word, "bold", 0, 4),
      Token(Space, " ", 4, 5),
      Token(Word, "italic", 5, 11),
      Token(Space, " ", 11, 12),
      Token(Word, "12px", 12, 16),
      Token(Space, " \t ", 16, 19),
      Token(Div, "/", 19, 20),
      Token(Word, "3", 20, 21),
      Token(Space, " ", 21, 22),
      Token(String, "'Open Sans'", 22, 33),
      Token(Div, ",", 33, 34),
      Token(Space, " ", 34, 35),
      Token(Word, "Arial", 35, 40),
      Token(Div, ",", 40, 41),
      Token(Space, " ", 41, 42),
      Token(String, "\"Helvetica Neue\"", 42, 58),
      Token(Div, ",", 58, 59),
      Token(Space, " ", 59, 60),
      Token(Word, "sans-serif", 60, 70),
    ],
  );
}

#[test]
fn tokenizes_should_process_color_value() {
  run(
    "rgba( 29, 439 , 29 )",
    vec![
      Token(Word, "rgba", 0, 4),
      Token(OpenParentheses, "(", 4, 5),
      Token(Space, " ", 5, 6),
      Token(Word, "29", 6, 8),
      Token(Div, ",", 8, 9),
      Token(Space, " ", 9, 10),
      Token(Word, "439", 10, 13),
      Token(Space, " ", 13, 14),
      Token(Div, ",", 14, 15),
      Token(Space, " ", 15, 16),
      Token(Word, "29", 16, 18),
      Token(Space, " ", 18, 19),
      Token(CloseParentheses, ")", 19, 20),
    ],
  );
}

#[test]
fn tokenizes_should_process_url_function() {
  run(
    "url( /gfx/img/bg.jpg )",
    vec![
      Token(Word, "url", 0, 3),
      Token(OpenParentheses, "(", 3, 4),
      Token(Space, " ", 4, 5),
      Token(Div, "/", 5, 6),
      Token(Word, "gfx", 6, 9),
      Token(Div, "/", 9, 10),
      Token(Word, "img", 10, 13),
      Token(Div, "/", 13, 14),
      Token(Word, "bg.jpg", 14, 20),
      Token(Space, " ", 20, 21),
      Token(CloseParentheses, ")", 21, 22),
    ],
  );
}

#[test]
fn tokenizes_should_process_url_function_with_quoted_first_argument() {
  run(
    "url( \"/gfx/img/bg.jpg\" hello )",
    vec![
      Token(Word, "url", 0, 3),
      Token(OpenParentheses, "(", 3, 4),
      Token(Space, " ", 4, 5),
      Token(String, "\"/gfx/img/bg.jpg\"", 5, 22),
      Token(Space, " ", 22, 23),
      Token(Word, "hello", 23, 28),
      Token(Space, " ", 28, 29),
      Token(CloseParentheses, ")", 29, 30),
    ],
  );
}

#[test]
fn tokenizes_should_process_division_with_spaces() {
  run(
    "calc(1 / 2)",
    vec![
      Token(Word, "calc", 0, 4),
      Token(OpenParentheses, "(", 4, 5),
      Token(Word, "1", 5, 6),
      Token(Space, " ", 6, 7),
      Token(Div, "/", 7, 8),
      Token(Space, " ", 8, 9),
      Token(Word, "2", 9, 10),
      Token(CloseParentheses, ")", 10, 11),
    ],
  );
}

#[test]
fn tokenizes_should_process_multiplication_without_spaces() {
  run(
    "calc(1*2)",
    vec![
      Token(Word, "calc", 0, 4),
      Token(OpenParentheses, "(", 4, 5),
      Token(Word, "1", 5, 6),
      Token(Word, "*", 6, 7),
      Token(Word, "2", 7, 8),
      Token(CloseParentheses, ")", 8, 9),
    ],
  );
}

#[test]
fn tokenizes_should_process_comments() {
  run(
    "/*before*/ 1px /*between*/ 1px /*after*/",
    vec![
      Token(Comment, "before", 0, 9),
      Token(Space, " ", 10, 11),
      Token(Word, "1px", 11, 14),
      Token(Space, " ", 14, 15),
      Token(Comment, "between", 15, 25),
      Token(Space, " ", 26, 27),
      Token(Word, "1px", 27, 30),
      Token(Space, " ", 30, 31),
      Token(Comment, "after", 31, 39),
    ],
  );
}

#[test]
fn tokenizes_should_process_comments_inside_functions() {
  run(
    "rgba( 0, 55/55, 0/*,.5*/ )",
    vec![
      Token(Word, "rgba", 0, 4),
      Token(OpenParentheses, "(", 4, 5),
      Token(Space, " ", 5, 6),
      Token(Word, "0", 6, 7),
      Token(Div, ",", 7, 8),
      Token(Space, " ", 8, 9),
      Token(Word, "55", 9, 11),
      Token(Div, "/", 11, 12),
      Token(Word, "55", 12, 14),
      Token(Div, ",", 14, 15),
      Token(Space, " ", 15, 16),
      Token(Word, "0", 16, 17),
      Token(Comment, ",.5", 17, 23),
      Token(Space, " ", 24, 25),
      Token(CloseParentheses, ")", 25, 26),
    ],
  );
}

#[test]
fn tokenizes_should_process_unclosed_comments() {
  run(
    "/*comment*/ 1px /* unclosed ",
    vec![
      Token(Comment, "comment", 0, 10),
      Token(Space, " ", 11, 12),
      Token(Word, "1px", 12, 15),
      Token(Space, " ", 15, 16),
      Token(Comment, " unclosed ", 16, 29),
    ],
  );
}

#[test]
fn tokenizes_should_process_escape_character() {
  run(
    "Hawaii \\35 -0",
    vec![
      Token(Word, "Hawaii", 0, 6),
      Token(Space, " ", 6, 7),
      Token(Word, "\\35", 7, 10),
      Token(Space, " ", 10, 11),
      Token(Word, "-0", 11, 13),
    ],
  );
}

#[test]
fn tokenizes_should_process_unicode_range_single_codepoint() {
  run("U+26", vec![Token(UnicodeRange, "U+26", 0, 4)]);
}

#[test]
fn tokenizes_should_process_unicode_range_single_codepoint2() {
  run("U+0-7F", vec![Token(UnicodeRange, "U+0-7F", 0, 6)]);
}

#[test]
fn tokenizes_should_process_unicode_range_codepoint_range() {
  run(
    "U+0025-00FF",
    vec![Token(UnicodeRange, "U+0025-00FF", 0, 11)],
  );
}

#[test]
fn tokenizes_should_process_unicode_range_wildcard_range() {
  run("U+4??", vec![Token(UnicodeRange, "U+4??", 0, 5)]);
}

#[test]
fn tokenizes_should_process_unicode_range_multiple_values() {
  run(
    "U+0025-00FF, U+4??",
    vec![
      Token(UnicodeRange, "U+0025-00FF", 0, 11),
      Token(Div, ",", 11, 12),
      Token(Space, " ", 12, 13),
      Token(UnicodeRange, "U+4??", 13, 18),
    ],
  );
}

#[test]
fn tokenizes_should_process_invalid_unicode_range_as_word() {
  run("U+4??Z", vec![Token(Word, "U+4??Z", 0, 6)]);
}

#[test]
fn tokenizes_should_process_invalid_unicode_range_as_word2() {
  run("U+", vec![Token(Word, "U+", 0, 2)]);
}

#[test]
fn tokenizes_should_process_invalid_unicode_range_as_word3() {
  run("U+Z", vec![Token(Word, "U+Z", 0, 3)]);
}

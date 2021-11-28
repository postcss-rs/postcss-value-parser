use postcss_value_parser::node::*;
use postcss_value_parser::parse;
#[cfg(test)]
use pretty_assertions::assert_eq;
use std::borrow::Cow;

#[test]
fn parser_should_correctly_process_empty_input() {
  let expected = vec![];
  assert_eq!(parse(""), expected);
}

#[test]
fn parser_should_process_escaped_parentheses_open() {
  let expected = vec![Word {
    source_index: 0,
    value: Cow::Borrowed("\\("),
  }
  .into()];
  assert_eq!(parse("\\("), expected);
}

#[test]
fn parser_should_process_escaped_parentheses_close() {
  let expected = vec![Word {
    source_index: 0,
    value: Cow::Borrowed("\\)"),
  }
  .into()];
  assert_eq!(parse("\\)"), expected);
}

#[test]
fn parser_should_process_escaped_parentheses_both() {
  let expected = vec![Word {
    source_index: 0,
    value: Cow::Borrowed("\\(\\)"),
  }
  .into()];
  assert_eq!(parse("\\(\\)"), expected);
}

#[test]
fn parser_should_process_escaped_parentheses_both2() {
  let expected = vec![
    Word {
      source_index: 0,
      value: Cow::Borrowed("\\("),
    }
    .into(),
    Space {
      source_index: 2,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 3,
      value: Cow::Borrowed("\\)"),
    }
    .into(),
  ];
  assert_eq!(parse("\\( \\)"), expected);
}

#[test]
fn parser_should_process_unopened_parentheses_as_word() {
  let expected = vec![
    Function {
      source_index: 0,
      value: Cow::Borrowed(""),
      nodes: vec![],
    }
    .into(),
    Space {
      source_index: 2,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 3,
      value: Cow::Borrowed(")wo)rd)"),
    }
    .into(),
  ];
  assert_eq!(parse("() )wo)rd)"), expected);
}

#[test]
fn parser_should_add_before_prop() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed(""),
    nodes: vec![Space {
      source_index: 1,
      value: Cow::Borrowed(" "),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("( )"), expected);
}

#[test]
fn parser_should_add_before_and_after_prop() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed(""),
    nodes: vec![
      Space {
        source_index: 1,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 2,
        value: Cow::Borrowed("|"),
      }
      .into(),
      Space {
        source_index: 3,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("( | )"), expected);
}

#[test]
fn parser_should_add_value_prop() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("name"),
    nodes: vec![],
  }
  .into()];
  assert_eq!(parse("name()"), expected);
}

#[test]
fn parser_should_process_nested_functions() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed(""),
    nodes: vec![Function {
      source_index: 1,
      value: Cow::Borrowed(""),
      nodes: vec![Function {
        source_index: 2,
        value: Cow::Borrowed(""),
        nodes: vec![],
      }
      .into()],
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("((()))"), expected);
}

#[test]
fn parser_should_process_advanced_nested_functions() {
  let expected = vec![
    Function {
      source_index: 0,
      value: Cow::Borrowed(""),
      nodes: vec![
        Space {
          source_index: 1,
          value: Cow::Borrowed(" "),
        }
        .into(),
        Function {
          source_index: 2,
          value: Cow::Borrowed("calc"),
          nodes: vec![
            Function {
              source_index: 7,
              value: Cow::Borrowed(""),
              nodes: vec![Space {
                source_index: 8,
                value: Cow::Borrowed(" "),
              }
              .into()],
            }
            .into(),
            Space {
              source_index: 10,
              value: Cow::Borrowed(" "),
            }
            .into(),
          ],
        }
        .into(),
      ],
    }
    .into(),
    Word {
      source_index: 13,
      value: Cow::Borrowed("word"),
    }
    .into(),
  ];
  assert_eq!(parse("( calc(( ) ))word"), expected);
}

#[test]
fn parser_should_process_divider_slash() {
  let expected = vec![Div {
    source_index: 0,
    value: Cow::Borrowed("/"),
  }
  .into()];
  assert_eq!(parse("/"), expected);
}

#[test]
fn parser_should_process_divider_() {
  let expected = vec![Div {
    source_index: 0,
    value: Cow::Borrowed(":"),
  }
  .into()];
  assert_eq!(parse(":"), expected);
}

#[test]
fn parser_should_process_divider_comma() {
  let expected = vec![Div {
    source_index: 0,
    value: Cow::Borrowed(","),
  }
  .into()];
  assert_eq!(parse(","), expected);
}

#[test]
fn parser_should_process_complex_divider() {
  let expected = vec![
    Space {
      source_index: 0,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Div {
      source_index: 1,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 2,
      value: Cow::Borrowed(" "),
    }
    .into(),
  ];
  assert_eq!(parse(" , "), expected);
}

#[test]
fn parser_should_process_divider_in_function() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed(""),
    nodes: vec![
      Space {
        source_index: 1,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Div {
        source_index: 2,
        value: Cow::Borrowed(","),
      }
      .into(),
      Space {
        source_index: 3,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("( , )"), expected);
}

#[test]
fn parser_should_process_two_spaced_divider() {
  let expected = vec![
    Space {
      source_index: 0,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Div {
      source_index: 1,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 2,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Div {
      source_index: 3,
      value: Cow::Borrowed(":"),
    }
    .into(),
    Space {
      source_index: 4,
      value: Cow::Borrowed(" "),
    }
    .into(),
  ];
  assert_eq!(parse(" , : "), expected);
}

#[test]
fn parser_should_process_empty_quoted_strings_() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("\"\""),
  }
  .into()];
  assert_eq!(parse("\"\""), expected);
}

#[test]
fn parser_should_process_empty_quoted_strings_2() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("''"),
  }
  .into()];
  assert_eq!(parse("''"), expected);
}

#[test]
fn parser_should_process_escaped_quotes_2() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("'word\\'word'"),
  }
  .into()];
  assert_eq!(parse("'word\\'word'"), expected);
}

#[test]
fn parser_should_process_escaped_quotes_() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("\"word\\\"word\""),
  }
  .into()];
  assert_eq!(parse("\"word\\\"word\""), expected);
}

#[test]
fn parser_should_process_single_quotes_inside_double_quotes_() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("\"word'word\""),
  }
  .into()];
  assert_eq!(parse("\"word'word\""), expected);
}

#[test]
fn parser_should_process_double_quotes_inside_single_quotes_() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("'word\"word'"),
  }
  .into()];
  assert_eq!(parse("'word\"word'"), expected);
}

#[test]
fn parser_should_process_unclosed_quotes() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("\"word"),
  }
  .into()];
  assert_eq!(parse("\"word"), expected);
}

#[test]
fn parser_should_process_unclosed_quotes_with_ended_backslash() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("\"word\\"),
  }
  .into()];
  assert_eq!(parse("\"word\\"), expected);
}

#[test]
fn parser_should_process_quoted_strings() {
  let expected = vec![String {
    source_index: 0,
    value: Cow::Borrowed("\"string\""),
  }
  .into()];
  assert_eq!(parse("\"string\""), expected);
}

#[test]
fn parser_should_process_quoted_strings_and_words() {
  let expected = vec![
    Word {
      source_index: 0,
      value: Cow::Borrowed("word1"),
    }
    .into(),
    String {
      source_index: 5,
      value: Cow::Borrowed("\"string\""),
    }
    .into(),
    Word {
      source_index: 13,
      value: Cow::Borrowed("word2"),
    }
    .into(),
  ];
  assert_eq!(parse("word1\"string\"word2"), expected);
}

#[test]
fn parser_should_process_quoted_strings_and_spaces() {
  let expected = vec![
    Space {
      source_index: 0,
      value: Cow::Borrowed(" "),
    }
    .into(),
    String {
      source_index: 1,
      value: Cow::Borrowed("\"string\""),
    }
    .into(),
    Space {
      source_index: 9,
      value: Cow::Borrowed(" "),
    }
    .into(),
  ];
  assert_eq!(parse(" \"string\" "), expected);
}

#[test]
fn parser_should_process_escaped_symbols_as_words() {
  let expected = vec![
    Space {
      source_index: 0,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 1,
      value: Cow::Borrowed("\\\"word\\'\\ \\\t"),
    }
    .into(),
    Space {
      source_index: 13,
      value: Cow::Borrowed(" "),
    }
    .into(),
  ];
  assert_eq!(parse(" \\\"word\\'\\ \\\t "), expected);
}

#[test]
fn parser_should_correctly_proceess_font_value() {
  let expected = vec![
    Word {
      source_index: 0,
      value: Cow::Borrowed("bold"),
    }
    .into(),
    Space {
      source_index: 4,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 5,
      value: Cow::Borrowed("italic"),
    }
    .into(),
    Space {
      source_index: 11,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 12,
      value: Cow::Borrowed("12px"),
    }
    .into(),
    Space {
      source_index: 16,
      value: Cow::Borrowed(" \t "),
    }
    .into(),
    Div {
      source_index: 19,
      value: Cow::Borrowed("/"),
    }
    .into(),
    Word {
      source_index: 20,
      value: Cow::Borrowed("3"),
    }
    .into(),
    Space {
      source_index: 21,
      value: Cow::Borrowed(" "),
    }
    .into(),
    String {
      source_index: 22,
      value: Cow::Borrowed("'Open Sans'"),
    }
    .into(),
    Div {
      source_index: 33,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 34,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 35,
      value: Cow::Borrowed("Arial"),
    }
    .into(),
    Div {
      source_index: 40,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 41,
      value: Cow::Borrowed(" "),
    }
    .into(),
    String {
      source_index: 42,
      value: Cow::Borrowed("\"Helvetica Neue\""),
    }
    .into(),
    Div {
      source_index: 58,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 59,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 60,
      value: Cow::Borrowed("sans-serif"),
    }
    .into(),
  ];
  assert_eq!(
    parse("bold italic 12px \t /3 'Open Sans', Arial, \"Helvetica Neue\", sans-serif"),
    expected
  );
}

#[test]
fn parser_should_correctly_proceess_color_value() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("rgba"),
    nodes: vec![
      Space {
        source_index: 5,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 6,
        value: Cow::Borrowed("29"),
      }
      .into(),
      Div {
        source_index: 8,
        value: Cow::Borrowed(","),
      }
      .into(),
      Space {
        source_index: 9,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 10,
        value: Cow::Borrowed("439"),
      }
      .into(),
      Space {
        source_index: 13,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Div {
        source_index: 14,
        value: Cow::Borrowed(","),
      }
      .into(),
      Space {
        source_index: 15,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 16,
        value: Cow::Borrowed("29"),
      }
      .into(),
      Space {
        source_index: 18,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("rgba( 29, 439 , 29 )"), expected);
}

#[test]
fn parser_should_correctly_process_url_function() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![
      Space {
        source_index: 4,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 5,
        value: Cow::Borrowed("/gfx/img/bg.jpg"),
      }
      .into(),
      Space {
        source_index: 20,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("url( /gfx/img/bg.jpg )"), expected);
}

#[test]
fn parser_should_add_unclosed_true_prop_for_url_function() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![
      Space {
        source_index: 4,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 5,
        value: Cow::Borrowed("/gfx/img/bg.jpg"),
      }
      .into(),
      Space {
        source_index: 20,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("url( /gfx/img/bg.jpg "), expected);
}

#[test]
fn parser_should_correctly_process_url_function_with_quoted_first_argument() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![
      Space {
        source_index: 4,
        value: Cow::Borrowed(" "),
      }
      .into(),
      String {
        source_index: 5,
        value: Cow::Borrowed("\"/gfx/img/bg.jpg\""),
      }
      .into(),
      Space {
        source_index: 22,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 23,
        value: Cow::Borrowed("hello"),
      }
      .into(),
      Space {
        source_index: 28,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("url( \"/gfx/img/bg.jpg\" hello )"), expected);
}

#[test]
fn parser_should_correctly_parse_spaces() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("calc"),
    nodes: vec![
      Word {
        source_index: 5,
        value: Cow::Borrowed("1"),
      }
      .into(),
      Space {
        source_index: 6,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 7,
        value: Cow::Borrowed("+"),
      }
      .into(),
      Space {
        source_index: 8,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 9,
        value: Cow::Borrowed("2"),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("calc(1 + 2)"), expected);
}

#[test]
fn parser_should_correctly_parse_subtraction_with_spaces() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("calc"),
    nodes: vec![
      Word {
        source_index: 5,
        value: Cow::Borrowed("1"),
      }
      .into(),
      Space {
        source_index: 6,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 7,
        value: Cow::Borrowed("-"),
      }
      .into(),
      Space {
        source_index: 8,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 9,
        value: Cow::Borrowed("2"),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("calc(1 - 2)"), expected);
}

#[test]
fn parser_should_correctly_parse_multiplication_with_spaces() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("calc"),
    nodes: vec![
      Word {
        source_index: 5,
        value: Cow::Borrowed("1"),
      }
      .into(),
      Space {
        source_index: 6,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 7,
        value: Cow::Borrowed("*"),
      }
      .into(),
      Space {
        source_index: 8,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 9,
        value: Cow::Borrowed("2"),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("calc(1 * 2)"), expected);
}

#[test]
fn parser_should_correctly_parse_division_with_spaces() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("calc"),
    nodes: vec![
      Word {
        source_index: 5,
        value: Cow::Borrowed("1"),
      }
      .into(),
      Space {
        source_index: 6,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 7,
        value: Cow::Borrowed("/"),
      }
      .into(),
      Space {
        source_index: 8,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 9,
        value: Cow::Borrowed("2"),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("calc(1 / 2)"), expected);
}

#[test]
fn parser_should_correctly_parse_multiplication_without_spaces() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("calc"),
    nodes: vec![
      Word {
        source_index: 5,
        value: Cow::Borrowed("1"),
      }
      .into(),
      Word {
        source_index: 6,
        value: Cow::Borrowed("*"),
      }
      .into(),
      Word {
        source_index: 7,
        value: Cow::Borrowed("2"),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("calc(1*2)"), expected);
}

#[test]
fn parser_should_correctly_parse_division_without_spaces() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("calc"),
    nodes: vec![
      Word {
        source_index: 5,
        value: Cow::Borrowed("1"),
      }
      .into(),
      Word {
        source_index: 6,
        value: Cow::Borrowed("/"),
      }
      .into(),
      Word {
        source_index: 7,
        value: Cow::Borrowed("2"),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("calc(1/2)"), expected);
}

#[test]
fn parser_should_correctly_process_nested_calc_functions() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("calc"),
    nodes: vec![
      Function {
        source_index: 5,
        value: Cow::Borrowed(""),
        nodes: vec![
          Function {
            source_index: 6,
            value: Cow::Borrowed(""),
            nodes: vec![
              Word {
                source_index: 7,
                value: Cow::Borrowed("768px"),
              }
              .into(),
              Space {
                source_index: 12,
                value: Cow::Borrowed(" "),
              }
              .into(),
              Word {
                source_index: 13,
                value: Cow::Borrowed("-"),
              }
              .into(),
              Space {
                source_index: 14,
                value: Cow::Borrowed(" "),
              }
              .into(),
              Word {
                source_index: 15,
                value: Cow::Borrowed("100vw"),
              }
              .into(),
            ],
          }
          .into(),
          Space {
            source_index: 21,
            value: Cow::Borrowed(" "),
          }
          .into(),
          Div {
            source_index: 22,
            value: Cow::Borrowed("/"),
          }
          .into(),
          Space {
            source_index: 23,
            value: Cow::Borrowed(" "),
          }
          .into(),
          Word {
            source_index: 24,
            value: Cow::Borrowed("2"),
          }
          .into(),
        ],
      }
      .into(),
      Space {
        source_index: 26,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 27,
        value: Cow::Borrowed("-"),
      }
      .into(),
      Space {
        source_index: 28,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 29,
        value: Cow::Borrowed("15px"),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("calc(((768px - 100vw) / 2) - 15px)"), expected);
}

#[test]
fn parser_should_process_colons_with_params() {
  let expected = vec![
    Function {
      source_index: 0,
      value: Cow::Borrowed(""),
      nodes: vec![
        Word {
          source_index: 1,
          value: Cow::Borrowed("min-width"),
        }
        .into(),
        Div {
          source_index: 10,
          value: Cow::Borrowed(":"),
        }
        .into(),
        Space {
          source_index: 11,
          value: Cow::Borrowed(" "),
        }
        .into(),
        Word {
          source_index: 12,
          value: Cow::Borrowed("700px"),
        }
        .into(),
      ],
    }
    .into(),
    Space {
      source_index: 18,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 19,
      value: Cow::Borrowed("and"),
    }
    .into(),
    Space {
      source_index: 22,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Function {
      source_index: 23,
      value: Cow::Borrowed(""),
      nodes: vec![
        Word {
          source_index: 24,
          value: Cow::Borrowed("orientation"),
        }
        .into(),
        Div {
          source_index: 35,
          value: Cow::Borrowed(":"),
        }
        .into(),
        Space {
          source_index: 36,
          value: Cow::Borrowed(" "),
        }
        .into(),
        Word {
          source_index: 37,
          value: Cow::Borrowed("\\$landscape"),
        }
        .into(),
      ],
    }
    .into(),
  ];
  assert_eq!(
    parse("(min-width: 700px) and (orientation: \\$landscape)"),
    expected
  );
}

#[test]
fn parser_should_escape_parentheses_with_backslash() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![
      Space {
        source_index: 4,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 5,
        value: Cow::Borrowed("http://website.com/assets\\)_test"),
      }
      .into(),
      Space {
        source_index: 37,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("url( http://website.com/assets\\)_test )"), expected);
}

#[test]
fn parser_should_parse_parentheses_correctly() {
  let expected = vec![
    Function {
      source_index: 0,
      value: Cow::Borrowed("fn1"),
      nodes: vec![
        Function {
          source_index: 4,
          value: Cow::Borrowed("fn2"),
          nodes: vec![Word {
            source_index: 8,
            value: Cow::Borrowed("255"),
          }
          .into()],
        }
        .into(),
        Div {
          source_index: 12,
          value: Cow::Borrowed(","),
        }
        .into(),
        Space {
          source_index: 13,
          value: Cow::Borrowed(" "),
        }
        .into(),
        Function {
          source_index: 14,
          value: Cow::Borrowed("fn3"),
          nodes: vec![Word {
            source_index: 18,
            value: Cow::Borrowed(".2"),
          }
          .into()],
        }
        .into(),
      ],
    }
    .into(),
    Div {
      source_index: 22,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 23,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Function {
      source_index: 24,
      value: Cow::Borrowed("fn4"),
      nodes: vec![
        Function {
          source_index: 28,
          value: Cow::Borrowed("fn5"),
          nodes: vec![
            Word {
              source_index: 32,
              value: Cow::Borrowed("255"),
            }
            .into(),
            Div {
              source_index: 35,
              value: Cow::Borrowed(","),
            }
            .into(),
            Word {
              source_index: 36,
              value: Cow::Borrowed(".2"),
            }
            .into(),
          ],
        }
        .into(),
        Div {
          source_index: 39,
          value: Cow::Borrowed(","),
        }
        .into(),
        Space {
          source_index: 40,
          value: Cow::Borrowed(" "),
        }
        .into(),
        Word {
          source_index: 41,
          value: Cow::Borrowed("fn6"),
        }
        .into(),
      ],
    }
    .into(),
  ];
  assert_eq!(
    parse("fn1(fn2(255), fn3(.2)), fn4(fn5(255,.2), fn6)"),
    expected
  );
}

#[test]
fn parser_shouldnt_throw_an_error_on_unclosed_function() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed(""),
    nodes: vec![
      Word {
        source_index: 1,
        value: Cow::Borrowed("0"),
      }
      .into(),
      Space {
        source_index: 2,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 3,
        value: Cow::Borrowed("32"),
      }
      .into(),
      Space {
        source_index: 5,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 6,
        value: Cow::Borrowed("word"),
      }
      .into(),
      Space {
        source_index: 10,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("(0 32 word "), expected);
}

#[test]
fn parser_should_add_unclosed_true_prop_for_every_unclosed_function() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed(""),
    nodes: vec![
      Space {
        source_index: 1,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Function {
        source_index: 2,
        value: Cow::Borrowed(""),
        nodes: vec![
          Space {
            source_index: 3,
            value: Cow::Borrowed(" "),
          }
          .into(),
          Function {
            source_index: 4,
            value: Cow::Borrowed(""),
            nodes: vec![Space {
              source_index: 5,
              value: Cow::Borrowed(" "),
            }
            .into()],
          }
          .into(),
          Space {
            source_index: 7,
            value: Cow::Borrowed(" "),
          }
          .into(),
        ],
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("( ( ( ) "), expected);
}

#[test]
fn parser_shouldnt_throw_an_error_on_unopened_function() {
  let expected = vec![
    Word {
      source_index: 0,
      value: Cow::Borrowed("0"),
    }
    .into(),
    Space {
      source_index: 1,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 2,
      value: Cow::Borrowed("32"),
    }
    .into(),
    Space {
      source_index: 4,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 5,
      value: Cow::Borrowed("word"),
    }
    .into(),
    Space {
      source_index: 9,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 10,
      value: Cow::Borrowed(")"),
    }
    .into(),
    Space {
      source_index: 11,
      value: Cow::Borrowed(" "),
    }
    .into(),
  ];
  assert_eq!(parse("0 32 word ) "), expected);
}

#[test]
fn parser_should_process_escaped_spaces_as_word_in_fonts() {
  let expected = vec![Word {
    source_index: 0,
    value: Cow::Borrowed("Bond\\ 007"),
  }
  .into()];
  assert_eq!(parse("Bond\\ 007"), expected);
}

#[test]
fn parser_should_parse_double_url_and_comma() {
  let expected = vec![
    Function {
      source_index: 0,
      value: Cow::Borrowed("url"),
      nodes: vec![Word {
        source_index: 4,
        value: Cow::Borrowed("foo/bar.jpg"),
      }
      .into()],
    }
    .into(),
    Div {
      source_index: 16,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 17,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Function {
      source_index: 18,
      value: Cow::Borrowed("url"),
      nodes: vec![Word {
        source_index: 22,
        value: Cow::Borrowed("http://website.com/img.jpg"),
      }
      .into()],
    }
    .into(),
  ];
  assert_eq!(
    parse("url(foo/bar.jpg), url(http://website.com/img.jpg)"),
    expected
  );
}

#[test]
fn parser_should_parse_empty_url() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![],
  }
  .into()];
  assert_eq!(parse("url()"), expected);
}

#[test]
fn parser_should_parse_empty_url_with_space() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![Space {
      source_index: 4,
      value: Cow::Borrowed(" "),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("url( )"), expected);
}

#[test]
fn parser_should_parse_empty_url_with_multiple_spaces() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![Space {
      source_index: 4,
      value: Cow::Borrowed("   "),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("url(   )"), expected);
}

#[test]
fn parser_should_parse_empty_url_with_newline_lf() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![Space {
      source_index: 4,
      value: Cow::Borrowed("\n"),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("url(\n)"), expected);
}

#[test]
fn parser_should_parse_empty_url_with_newline_crlf() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![Space {
      source_index: 4,
      value: Cow::Borrowed("\r\n"),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("url(\r\n)"), expected);
}

#[test]
fn parser_should_parse_empty_url_with_multiple_newlines_lf() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![Space {
      source_index: 4,
      value: Cow::Borrowed("\n\n\n"),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("url(\n\n\n)"), expected);
}

#[test]
fn parser_should_parse_empty_url_with_multiple_newlines_crlf() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![Space {
      source_index: 4,
      value: Cow::Borrowed("\r\n\r\n\r\n"),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("url(\r\n\r\n\r\n)"), expected);
}

#[test]
fn parser_should_parse_empty_url_with_whitespace_characters() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![Space {
      source_index: 4,
      value: Cow::Borrowed("  \n \t  \r\n  "),
    }
    .into()],
  }
  .into()];
  assert_eq!(parse("url(  \n \t  \r\n  )"), expected);
}

#[test]
fn parser_should_parse_comments() {
  let expected = vec![
    Comment {
      source_index: 0,
      value: Cow::Borrowed("before"),
    }
    .into(),
    Space {
      source_index: 10,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 11,
      value: Cow::Borrowed("1px"),
    }
    .into(),
    Space {
      source_index: 14,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Comment {
      source_index: 15,
      value: Cow::Borrowed("between"),
    }
    .into(),
    Space {
      source_index: 26,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 27,
      value: Cow::Borrowed("1px"),
    }
    .into(),
    Space {
      source_index: 30,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Comment {
      source_index: 31,
      value: Cow::Borrowed("after"),
    }
    .into(),
  ];
  assert_eq!(parse("/*before*/ 1px /*between*/ 1px /*after*/"), expected);
}

#[test]
fn parser_should_parse_comments_inside_functions() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("rgba"),
    nodes: vec![
      Space {
        source_index: 5,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 6,
        value: Cow::Borrowed("0"),
      }
      .into(),
      Div {
        source_index: 7,
        value: Cow::Borrowed(","),
      }
      .into(),
      Space {
        source_index: 8,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 9,
        value: Cow::Borrowed("55"),
      }
      .into(),
      Div {
        source_index: 11,
        value: Cow::Borrowed("/"),
      }
      .into(),
      Word {
        source_index: 12,
        value: Cow::Borrowed("55"),
      }
      .into(),
      Div {
        source_index: 14,
        value: Cow::Borrowed(","),
      }
      .into(),
      Space {
        source_index: 15,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 16,
        value: Cow::Borrowed("0"),
      }
      .into(),
      Comment {
        source_index: 17,
        value: Cow::Borrowed(",.5"),
      }
      .into(),
      Space {
        source_index: 24,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("rgba( 0, 55/55, 0/*,.5*/ )"), expected);
}

#[test]
fn parser_should_parse_comments_at_the_end_of_url_functions_with_quoted_first_argument() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![
      Space {
        source_index: 4,
        value: Cow::Borrowed(" "),
      }
      .into(),
      String {
        source_index: 5,
        value: Cow::Borrowed("\"/demo/bg.png\""),
      }
      .into(),
      Space {
        source_index: 19,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Comment {
        source_index: 20,
        value: Cow::Borrowed("comment"),
      }
      .into(),
      Space {
        source_index: 31,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("url( \"/demo/bg.png\" /*comment*/ )"), expected);
}

#[test]
fn parser_should_not_parse_comments_at_the_start_of_url_function_with_unquoted_first_argument() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![
      Space {
        source_index: 4,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 5,
        value: Cow::Borrowed("/*comment*/ /demo/bg.png"),
      }
      .into(),
      Space {
        source_index: 29,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("url( /*comment*/ /demo/bg.png )"), expected);
}

#[test]
fn parser_should_not_parse_comments_at_the_end_of_url_function_with_unquoted_first_argument() {
  let expected = vec![Function {
    source_index: 0,
    value: Cow::Borrowed("url"),
    nodes: vec![
      Space {
        source_index: 4,
        value: Cow::Borrowed(" "),
      }
      .into(),
      Word {
        source_index: 5,
        value: Cow::Borrowed("/demo/bg.png /*comment*/"),
      }
      .into(),
      Space {
        source_index: 17,
        value: Cow::Borrowed(" "),
      }
      .into(),
    ],
  }
  .into()];
  assert_eq!(parse("url( /demo/bg.png /*comment*/ )"), expected);
}

#[test]
fn parser_should_parse_unclosed_comments() {
  let expected = vec![
    Comment {
      source_index: 0,
      value: Cow::Borrowed("comment"),
    }
    .into(),
    Space {
      source_index: 11,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 12,
      value: Cow::Borrowed("1px"),
    }
    .into(),
    Space {
      source_index: 15,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Comment {
      source_index: 16,
      value: Cow::Borrowed(" unclosed "),
    }
    .into(),
  ];
  assert_eq!(parse("/*comment*/ 1px /* unclosed "), expected);
}

#[test]
fn parser_should_respect_escape_character() {
  let expected = vec![
    Word {
      source_index: 0,
      value: Cow::Borrowed("Hawaii"),
    }
    .into(),
    Space {
      source_index: 6,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 7,
      value: Cow::Borrowed("\\35"),
    }
    .into(),
    Space {
      source_index: 10,
      value: Cow::Borrowed(" "),
    }
    .into(),
    Word {
      source_index: 11,
      value: Cow::Borrowed("-0"),
    }
    .into(),
  ];
  assert_eq!(parse("Hawaii \\35 -0"), expected);
}

#[test]
fn parser_should_parse_unicode_range_single_codepoint() {
  let expected = vec![UnicodeRange {
    source_index: 0,
    value: Cow::Borrowed("U+26"),
  }
  .into()];
  assert_eq!(parse("U+26"), expected);
}

#[test]
fn parser_should_parse_unicode_range_single_codepoint_2() {
  let expected = vec![UnicodeRange {
    source_index: 0,
    value: Cow::Borrowed("U+0-7F"),
  }
  .into()];
  assert_eq!(parse("U+0-7F"), expected);
}

#[test]
fn parser_should_parse_unicode_range_single_codepoint_3() {
  let expected = vec![UnicodeRange {
    source_index: 0,
    value: Cow::Borrowed("U+0-7f"),
  }
  .into()];
  assert_eq!(parse("U+0-7f"), expected);
}

#[test]
fn parser_should_parse_unicode_range_single_codepoint_lowercase() {
  let expected = vec![UnicodeRange {
    source_index: 0,
    value: Cow::Borrowed("u+26"),
  }
  .into()];
  assert_eq!(parse("u+26"), expected);
}

#[test]
fn parser_should_parse_unicode_range_codepoint_range() {
  let expected = vec![UnicodeRange {
    source_index: 0,
    value: Cow::Borrowed("U+0025-00FF"),
  }
  .into()];
  assert_eq!(parse("U+0025-00FF"), expected);
}

#[test]
fn parser_should_parse_unicode_range_wildcard_range() {
  let expected = vec![UnicodeRange {
    source_index: 0,
    value: Cow::Borrowed("U+4??"),
  }
  .into()];
  assert_eq!(parse("U+4??"), expected);
}

#[test]
fn parser_should_parse_unicode_range_multiple_values() {
  let expected = vec![
    UnicodeRange {
      source_index: 0,
      value: Cow::Borrowed("U+0025-00FF"),
    }
    .into(),
    Div {
      source_index: 11,
      value: Cow::Borrowed(","),
    }
    .into(),
    Space {
      source_index: 12,
      value: Cow::Borrowed(" "),
    }
    .into(),
    UnicodeRange {
      source_index: 13,
      value: Cow::Borrowed("U+4??"),
    }
    .into(),
  ];
  assert_eq!(parse("U+0025-00FF, U+4??"), expected);
}

#[test]
fn parser_should_parse_invalid_unicode_range_as_word() {
  let expected = vec![Word {
    source_index: 0,
    value: Cow::Borrowed("U+4??Z"),
  }
  .into()];
  assert_eq!(parse("U+4??Z"), expected);
}

#[test]
fn parser_should_parse_invalid_unicode_range_as_word_2() {
  let expected = vec![Word {
    source_index: 0,
    value: Cow::Borrowed("U+"),
  }
  .into()];
  assert_eq!(parse("U+"), expected);
}

#[test]
fn parser_should_parse_invalid_unicode_range_as_word_3() {
  let expected = vec![Word {
    source_index: 0,
    value: Cow::Borrowed("U+Z"),
  }
  .into()];
  assert_eq!(parse("U+Z"), expected);
}

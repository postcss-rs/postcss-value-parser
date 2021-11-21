use postcss_rs_value_parser::unit;
use postcss_rs_value_parser::Dimension;

#[test]
fn test_unit() {
  // port from https://github.com/TrySound/postcss-value-parser/blob/f57bf9e5bc400f46a30485e996864f6b5f417fe8/test/unit.js#L4
  // MIT License, © Bogdan Chadkin (trysound@yandex.ru)
  //
  // ```
  // var tests = ...
  // tests
  //   .map(({ fixture, expected }) => ({ fixture, expected: expected ? `Some(Dimension::new("${expected.number}", "${expected.unit}"))` : "None" }))
  //   .map(({ fixture, expected }) => `("${fixture}", ${expected})`)
  //   .join(",\n")
  // ```
  let tests = vec![
    (".23rem", Some(Dimension::new(".23", "rem"))),
    (".2.3rem", Some(Dimension::new(".2", ".3rem"))),
    ("2.", Some(Dimension::new("2", "."))),
    ("+2.", Some(Dimension::new("+2", "."))),
    ("-2.", Some(Dimension::new("-2", "."))),
    ("+-2.", None),
    (".", None),
    (".rem", None),
    ("1e4px", Some(Dimension::new("1e4", "px"))),
    ("1em", Some(Dimension::new("1", "em"))),
    ("1e10", Some(Dimension::new("1e10", ""))),
    ("", None),
    ("e", None),
    ("e1", None),
    ("2rem", Some(Dimension::new("2", "rem"))),
    ("2.000rem", Some(Dimension::new("2.000", "rem"))),
    ("+2rem", Some(Dimension::new("+2", "rem"))),
    ("-2rem", Some(Dimension::new("-2", "rem"))),
    ("1.1rem", Some(Dimension::new("1.1", "rem"))),
    ("+1.1rem", Some(Dimension::new("+1.1", "rem"))),
    ("-1.1rem", Some(Dimension::new("-1.1", "rem"))),
    ("1.1e1rem", Some(Dimension::new("1.1e1", "rem"))),
    ("+1.1e1rem", Some(Dimension::new("+1.1e1", "rem"))),
    ("-1.1e1rem", Some(Dimension::new("-1.1e1", "rem"))),
    ("1.1e+1rem", Some(Dimension::new("1.1e+1", "rem"))),
    ("1.1e-1rem", Some(Dimension::new("1.1e-1", "rem"))),
    ("1.1e1e1rem", Some(Dimension::new("1.1e1", "e1rem"))),
    ("1.1e-1e", Some(Dimension::new("1.1e-1", "e"))),
    ("1.1e-1rem", Some(Dimension::new("1.1e-1", "rem"))),
    ("1.1e--++1e", Some(Dimension::new("1.1", "e--++1e"))),
    ("1.1e--++1rem", Some(Dimension::new("1.1", "e--++1rem"))),
    ("100+px", Some(Dimension::new("100", "+px"))),
    ("100.0.0px", Some(Dimension::new("100.0", ".0px"))),
    ("100e1epx", Some(Dimension::new("100e1", "epx"))),
    ("100e1e1px", Some(Dimension::new("100e1", "e1px"))),
    ("+100.1e+1e+1px", Some(Dimension::new("+100.1e+1", "e+1px"))),
    ("-100.1e-1e-1px", Some(Dimension::new("-100.1e-1", "e-1px"))),
    (".5px", Some(Dimension::new(".5", "px"))),
    ("+.5px", Some(Dimension::new("+.5", "px"))),
    ("-.5px", Some(Dimension::new("-.5", "px"))),
    (".5e1px", Some(Dimension::new(".5e1", "px"))),
    ("-.5e1px", Some(Dimension::new("-.5e1", "px"))),
    ("+.5e1px", Some(Dimension::new("+.5e1", "px"))),
    (".5e1e1px", Some(Dimension::new(".5e1", "e1px"))),
    (".5.5px", Some(Dimension::new(".5", ".5px"))),
    ("1e", Some(Dimension::new("1", "e"))),
    ("1e1", Some(Dimension::new("1e1", ""))),
    ("1ee", Some(Dimension::new("1", "ee"))),
    ("1e+", Some(Dimension::new("1", "e+"))),
    ("1e-", Some(Dimension::new("1", "e-"))),
    ("1e+1", Some(Dimension::new("1e+1", ""))),
    ("1e++1", Some(Dimension::new("1", "e++1"))),
    ("1e--1", Some(Dimension::new("1", "e--1"))),
    ("+10", Some(Dimension::new("+10", ""))),
    ("-10", Some(Dimension::new("-10", ""))),
    (".2px", Some(Dimension::new(".2", "px"))),
    ("-.2px", Some(Dimension::new("-.2", "px"))),
    ("+.2px", Some(Dimension::new("+.2", "px"))),
    (".a", None),
    (".", None),
    ("+", None),
    ("-", None),
    ("-a", None),
    ("+a", None),
    ("+.a", None),
    ("-.a", None),
    ("", None),
  ];

  for (fixture, expected) in tests {
    assert_eq!(unit(fixture), expected);
  }
}

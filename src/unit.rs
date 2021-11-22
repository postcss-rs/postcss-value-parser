/// A CSS dimension, decomposed into its numeric and unit parts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimension<'a> {
  pub number: &'a str,
  pub unit: &'a str,
}

impl<'a> Dimension<'a> {
  pub fn new(number: &'a str, unit: &'a str) -> Self {
    Self { number, unit }
  }
}

/// Decompose a CSS dimension into its numeric and unit part
///
/// Consume a number
/// https://www.w3.org/TR/css-syntax-3/#consume-number
pub fn unit(value: &str) -> Option<Dimension> {
  if !start_a_number(value) {
    return None;
  }

  let mut pos: usize = 0;

  // SAFETY: web allready call `starts with a number`
  unsafe {
    if matches!(value.as_bytes().get_unchecked(pos), b'+' | b'-') {
      pos += 1;
    }
  }

  while matches!(value.as_bytes().get(pos), Some(b'0'..=b'9')) {
    pos += 1;
  }

  // float number
  // the next 2 input code points are U+002E FULL STOP (.) followed by a digit
  if matches!(value[pos..].as_bytes(), [b'.', b'0'..=b'9', ..]) {
    pos += 2;
    while matches!(value.as_bytes().get(pos), Some(b'0'..=b'9')) {
      pos += 1;
    }
  }

  // scientific notation
  // the next 2 or 3 input code points are U+0045 LATIN CAPITAL LETTER E (E) or U+0065 LATIN SMALL LETTER E (e),
  // optionally followed by U+002D HYPHEN-MINUS (-) or U+002B PLUS SIGN (+), followed by a digit
  {
    if matches!(value[pos..].as_bytes(), [b'E' | b'e', b'0'..=b'9', ..]) {
      pos += 2;
    } else if matches!(
      value[pos..].as_bytes(),
      [b'E' | b'e', b'+' | b'-', b'0'..=b'9', ..]
    ) {
      pos += 3;
    }

    while matches!(value.as_bytes().get(pos), Some(b'0'..=b'9')) {
      pos += 1;
    }
  }

  Some(Dimension::new(&value[0..pos], &value[pos..]))
}

/// Check if three code points would start a number
/// https://www.w3.org/TR/css-syntax-3/#starts-with-a-number
#[inline]
fn start_a_number(value: &str) -> bool {
  match value.as_bytes().get(0) {
    Some(b'+' | b'-') => match value.as_bytes().get(1) {
      Some(b'.') => matches!(value.as_bytes().get(2), Some(b'0'..=b'9')),
      Some(b'0'..=b'9') => true,
      Some(_) => false,
      None => false,
    },
    Some(b'.') => matches!(value.as_bytes().get(1), Some(b'0'..=b'9')),
    Some(b'0'..=b'9') => true,
    Some(_) => false,
    None => false,
  }
}

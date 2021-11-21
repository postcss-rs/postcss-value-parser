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
pub fn unit(_value: &str) -> Option<Dimension> {
  unimplemented!();
}

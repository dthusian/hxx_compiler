use std::fmt::{Debug, Display, Formatter};

/// A Vec<T> except it has a Display impl that prints it seperated by a provided static string slice.
pub struct SepVec<T: Display>(pub Vec<T>, pub &'static str);

impl<T: Display> Display for SepVec<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !self.0.is_empty() {
      write!(f, "{}", &self.0[0])?;
    }
    for i in 1..self.0.len() {
      write!(f, "{}{}", self.1, self.0[i])?;
    }
    Ok(())
  }
}

impl<T: Display> Debug for SepVec<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    <Self as Display>::fmt(self, f)
  }
}
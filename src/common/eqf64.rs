use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct EqF64(pub f64);

impl PartialEq<Self> for EqF64 {
  fn eq(&self, other: &Self) -> bool {
    self.0.to_bits() == other.0.to_bits()
  }
}

impl Eq for EqF64 { }

impl PartialOrd<Self> for EqF64 {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.to_bits().partial_cmp(&other.0.to_bits())
  }
}

impl Ord for EqF64 {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.to_bits().cmp(&other.0.to_bits())
  }
}
pub enum HighPassFilter {
  Hz250,
  Hz1000,
  Hz2000,
  Hz4000,
}

impl HighPassFilter {
  pub const fn max_value(&self) -> u16 {
    match self {
      Self::Hz250 => 56492,
      Self::Hz1000 => 61787,
      Self::Hz2000 => 61787,
      Self::Hz4000 => 63164,
    }
  }
}

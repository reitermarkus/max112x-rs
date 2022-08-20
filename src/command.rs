use bitflags::bitflags;

use crate::ConversionSpeed;

bitflags! {
  pub struct Command: u8 {
    const START = 0b10000000;
    const MODE  = 0b01000000;

    // Conversion Mode
    const CAL   = 0b00100000;
    const IMPD  = 0b00010000;
    const RATE3 = 0b00001000;
    const RATE2 = 0b00000100;
    const RATE1 = 0b00000010;
    const RATE0 = 0b00000001;
    const RATE  = Self::RATE3.bits | Self::RATE2.bits | Self::RATE1.bits | Self::RATE0.bits;

    // Register Access Mode
    const RS4   = 0b00100000;
    const RS3   = 0b00010000;
    const RS2   = 0b00001000;
    const RS1   = 0b00000100;
    const RS0   = 0b00000010;
    const RW    = 0b00000001;
    const RS    = Self::RS4.bits | Self::RS3.bits | Self::RS2.bits | Self::RS1.bits | Self::RS0.bits;
  }
}

impl Command {
  const fn new() -> Self {
    Self::START
  }

  pub const fn conversion(rate: ConversionSpeed) -> Self {
    Self::new().union(Self::from_bits_truncate(rate as u8))
  }

  pub const fn register_write(reg: u8) -> Self {
    Self::new().union(Self::MODE).union(Self::from_bits_truncate(reg << 1))
  }

  pub const fn register_read(reg: u8) -> Self {
    Self::new().union(Self::MODE).union(Self::from_bits_truncate(reg << 1)).union(Self::RW)
  }
}

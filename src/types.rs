use core::fmt;

use crate::register::Stat;

/// A 24-bit unsigned integer.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub(crate) struct u24([u8; 3]);

impl fmt::Display for u24 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    u32::from(*self).fmt(f)
  }
}

impl fmt::Debug for u24 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    u32::from(*self).fmt(f)
  }
}

impl u24 {
  /// Create a native endian integer value from its representation as a byte array in big endian.
  pub(crate) const fn from_be_bytes(bytes: [u8; 3]) -> Self {
    Self(bytes)
  }

  /// Return the memory representation of this integer as a byte array in big-endian (network) byte order.
  pub const fn to_be_bytes(&self) -> [u8; 3] {
    self.0
  }

  /// Return the memory representation of this integer as a byte array in little-endian byte order.
  pub const fn to_le_bytes(&self) -> [u8; 3] {
    let [b0, b1, b2] = self.0;
    [b2, b1, b0]
  }
}

impl From<u24> for u32 {
  fn from(n: u24) -> Self {
    let [b2, b1, b0] = n.0;
    Self::from_be_bytes([0, b2, b1, b0])
  }
}

impl From<u24> for i32 {
  fn from(n: u24) -> Self {
    let [b2, b1, b0] = n.0;
    Self::from_be_bytes([0, b2, b1, b0])
  }
}

/// Conversion speed (samples per second).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionRate {
  /// 0.95 with SINC filter, 25 with single-cycle conversion
  Hz0_95  = 0b0000,
  /// 1.9 with SINC filter, 31.25 with single-cycle conversion
  Hz1_9   = 0b0001,
  /// 3.9 with SINC filter, 50 with single-cycle conversion
  Hz3_9   = 0b0010,
  /// 7.8 with SINC filter, 62.5 with single-cycle conversion
  Hz7_8   = 0b0011,
  /// 15.6 with SINC filter, 100 with single-cycle conversion
  Hz15_6  = 0b0100,
  /// 31.25 with SINC filter, 125 with single-cycle conversion
  Hz31_25 = 0b0101,
  /// 62.5 with SINC filter (supports FIR filter), 200 with single-cycle conversion
  Hz62_5  = 0b0110,
  /// 125 with SINC filter (supports FIR filter), 250 with single-cycle conversion
  Hz125   = 0b0111,
  /// 250 with SINC filter (supports FIR filter), 400 with single-cycle conversion
  Hz250   = 0b1000,
  /// 500 with SINC filter (supports FIR filter), 500 with single-cycle conversion
  Hz500   = 0b1001,
  /// 1000 with SINC filter (supports FIR filter), 800 with single-cycle conversion
  Hz1000  = 0b1010,
  /// 2000 with SINC filter (supports FIR filter), 1000 with single-cycle conversion
  Hz2000  = 0b1011,
  /// 4000 with SINC filter (supports FIR filter), 1600 with single-cycle conversion
  Hz4000  = 0b1100,
  /// 8000 with SINC filter (supports FIR filter), 2000 with single-cycle conversion
  Hz8000  = 0b1101,
  /// 16000 with SINC filter, 3200 with single-cycle conversion
  Hz16000 = 0b1110,
  /// 32000 with SINC filter, 6400 with single-cycle conversion
  Hz32000 = 0b1111,
}

/// Range format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
  /// Offset binary.
  OffsetBinary,
  /// Two's complement.
  TwosComplement,
}

/// Clock source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSource {
  /// Use external clock as the system clock.
  External,
  /// Use internal clock as the system clock.
  Internal,
}

/// PGA gain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pga {
  /// × 1
  X1,
  /// × 2
  X2,
  /// × 4
  X4,
  /// × 8
  X8,
  /// × 16
  X16,
  /// × 32
  X32,
  /// × 64
  X64,
  /// × 128
  X128,
}

/// System status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
  pub(crate) status: Stat,
}

impl Status {
  /// Check if a new conversion result is available.
  ///
  /// This is reset by reading the DATA register.
  ///
  /// This is duplicated by the RDYB pin.
  pub const fn data_ready(&self) -> bool {
    self.status.contains(Stat::RDY)
  }

  /// Check if the modulator is busy.
  ///
  /// This indicates that a conversion, self-calibration, or system calibration is in progress.
  pub const fn modulator_busy(&self) -> bool {
    self.status.contains(Stat::MSTAT)
  }

  /// Check if the conversion result has exceeded the maximum or minimum value and the result has been clipped.
  pub const fn data_overrange(&self) -> bool {
    self.status.contains(Stat::DOR)
  }

  /// Check if the system gain calibration was overranged.
  pub const fn system_gain_overrange(&self) -> bool {
    self.status.contains(Stat::DOR)
  }

  /// Get the conversion rate that corresponds to the result in the DATA register or the rate that was used for
  /// calibration coefficient calculation.
  ///
  /// Note: This is always the rate of previous conversion and not the rate of the conversion in progress.
  pub const fn data_rate(&self) -> ConversionRate {
    self.status.rate()
  }

  /// Check if the modulator detects that the analog input voltage exceeds 1.3 × full-scale range.
  pub const fn analog_overrange(&self) -> bool {
    self.status.contains(Stat::AOR)
  }

  /// Check if new result is being written to the DATA register while user is reading from the DATA register.
  pub const fn data_read_error(&self) -> bool {
    self.status.contains(Stat::RDERR)
  }

  /// Get the current ADC state.
  pub const fn state(&self) -> State {
    match self.status.intersection(Stat::PDSTAT).bits() >> 10 {
      0b00 => State::Conversion,
      0b01 => State::PowerDown,
      0b10 => State::Standby,
      _ => unreachable!(),
    }
  }
}

/// ADC state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
  /// ADC is converting.
  Conversion,
  /// ADC is fully powered down.
  PowerDown,
  /// ADC is in standby mode with modulator powered off but subregulator powered on.
  Standby,
}

/// Calibration type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Calibration {
  /// Self-calibration.
  SelfCalibration,
  /// System-level offset calibration.
  SystemOffsetCalibration,
  /// System-level full-scale calibration.
  SystemFullScaleCalibration,
}

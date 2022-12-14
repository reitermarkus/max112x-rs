#![allow(clippy::unusual_byte_groupings)] // FIXME: https://github.com/rust-lang/rust-clippy/issues/9183

use crate::ConversionSpeed;

pub trait ReadReg<R>
where
  Self: Sized,
{
  const ADDR: u8;

  fn from_reg(reg: R) -> Self;
}

pub trait WriteReg<R>: ReadReg<R> {
  fn to_reg(self) -> R;
}

macro_rules! register {
  (@impl_read_reg $Reg:ident : $addr:literal : $RegTy:ty) => {
    impl ReadReg<$RegTy> for $Reg {
      const ADDR: u8 = $addr;

      #[inline]
      fn from_reg(reg: $RegTy) -> Self {
        $Reg::from_bits_truncate(reg)
      }
    }
  };
  (@impl_write_reg $Reg:ident : $addr:literal : $RegTy:ty) => {
    impl WriteReg<$RegTy> for $Reg {
      fn to_reg(self) -> $RegTy {
        self.bits()
      }
    }
  };
  (
    #[doc = $name:expr]
    $vis:vis struct $Reg:ident($RegTy:ty): $addr:literal;
  ) => {
    #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
    $vis struct $Reg(pub $RegTy);

    impl $Reg {
      const fn from_bits_truncate(bits: $RegTy) -> Self {
        Self(bits)
      }
    }

    register!(@impl_read_reg $Reg: $addr: $RegTy);
  };
  (
    #[doc = $name:expr]
    $vis:vis struct $Reg:ident : $addr:literal : $RegTy:ty {
      $(
        $(#[$inner:ident $($args:tt)*])*
        const $const_name:ident = $const_value:expr;
      )*
    }
  ) => {
    ::bitflags::bitflags! {
      #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
      $vis struct $Reg: $RegTy {
        $(
          $(#[$inner $($args)*])*
          const $const_name = $const_value;
        )*
      }
    }

    register!(@impl_read_reg $Reg: $addr: $RegTy);
    register!(@impl_write_reg $Reg: $addr: $RegTy);
  };
}

#[allow(non_camel_case_types)]
pub struct u24([u8; 3]);

impl u24 {
  pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
    Self(bytes)
  }
}

impl From<u24> for u32 {
  fn from(n: u24) -> Self {
    let [b2, b1, b0] = n.0;
    Self::from_be_bytes([0, b2, b1, b0])
  }
}

register! {
  /// STAT
  pub struct Stat: 0x0: u16 {
    const INRESET = 0b10000000_00000000;
    const ERROR   = 0b01000000_00000000;
    const PDSTAT1 = 0b00001000_00000000;
    const PDSTAT0 = 0b00000100_00000000;
    const RDERR   = 0b00000010_00000000;
    const AOR     = 0b00000001_00000000;
    const RATE3   = 0b00000000_10000000;
    const RATE2   = 0b00000000_01000000;
    const RATE1   = 0b00000000_00100000;
    const RATE0   = 0b00000000_00010000;
    const SYSGOR  = 0b00000000_00001000;
    const DOR     = 0b00000000_00000100;
    const MSTAT   = 0b00000000_00000010;
    const RDY     = 0b00000000_00000001;

    const PDSTAT = Self::PDSTAT1.bits() | Self::PDSTAT0.bits();
    const RATE = Self::RATE3.bits() | Self::RATE2.bits() | Self::RATE1.bits() | Self::RATE0.bits();
  }
}

impl Stat {
  pub const fn rate(self) -> ConversionSpeed {
    match self.intersection(Self::RATE).bits() >> 4 {
      0b0000 => ConversionSpeed::Rate0,
      0b0001 => ConversionSpeed::Rate1,
      0b0010 => ConversionSpeed::Rate2,
      0b0011 => ConversionSpeed::Rate3,
      0b0100 => ConversionSpeed::Rate4,
      0b0101 => ConversionSpeed::Rate5,
      0b0110 => ConversionSpeed::Rate6,
      0b0111 => ConversionSpeed::Rate7,
      0b1000 => ConversionSpeed::Rate8,
      0b1001 => ConversionSpeed::Rate9,
      0b1010 => ConversionSpeed::Rate10,
      0b1011 => ConversionSpeed::Rate11,
      0b1100 => ConversionSpeed::Rate12,
      0b1101 => ConversionSpeed::Rate13,
      0b1110 => ConversionSpeed::Rate14,
      0b1111 => ConversionSpeed::Rate15,
      _ => unreachable!(),
    }
  }
}

register! {
  /// CTRL1
  pub struct Ctrl1: 0x1: u8 {
    /// External clock bit.
    ///
    /// - 0 Use internal oscillator as the system clock.
    /// - 1 Use external clock as the system clock.
    const EXTCK  = 0b10000000;
    /// Synchronization bit.
    ///
    /// - 0 Pulse synchronization mode.
    /// - 1 Continuous synchronization mode.
    const SYNC   = 0b01000000;
    /// Power mode bits.
    ///
    /// - 00 Normal power-up state. This is the default state.
    /// - 01 Sleep Mode???Powers down the subregulator and the entire digital circuitry. Upon resumption of power to the digital the PD[1:0] reverts to the default state of ???00???.
    /// - 10 Standby power???Powers down the analog blocks leaving the subregulator powered up.
    /// - 11 Resets all registers to POR state leaving the subregulator powered. The PD[1:0] bits are reset to ???00???. The operation of this state is identical to the RSTB pin.
    const PD1    = 0b00100000;
    const PD0    = 0b00010000;
    /// U/B: Unipolar/bipolar bit.
    ///
    /// - 0 Bipolar input range (??VREF).
    /// - 1 Unipolar input range (0 to VREF). S
    const UB     = 0b00001000;
    /// Bipolar range format bit.
    ///
    /// When reading bipolar data:
    /// - 0 Use two???s complement.
    /// - 1 Use offset binary.
    ///
    /// The data from unipolar range is always formatted in offset binary format.
    const FORMAT = 0b00000100;
    /// Single-cycle control bit.
    ///
    /// - 0 Continuous conversion mode.
    /// - 1 Single-cycle mode. The MAX11214 completes one no-latency conversion and then powers down into a leakage-only state.
    const SCYCLE = 0b00000010;
    /// Continuous single-cycle bit.
    ///
    /// - 0 A single conversion.
    /// - 1 Continuous conversions.
    const CONTSC = 0b00000001;
  }
}

register! {
  /// CTRL2
  pub struct Ctrl2: 0x2: u8 {
    const DGAIN1 = 0b10000000;
    const DGAIN0 = 0b01000000;
    const BUFEN  = 0b00100000;
    const LPMODE = 0b00010000;
    const PGAEN  = 0b00001000;
    const PGAG2  = 0b00000100;
    const PGAG1  = 0b00000010;
    const PGAG0  = 0b00000001;

    const PGAG = Self::PGAG2.bits() | Self::PGAG1.bits() | Self::PGAG0.bits();
  }
}

register! {
  /// CTRL3
  pub struct Ctrl3: 0x3: u8 {
    const ENMSYNC = 0b00100000;
    const MODBITS = 0b00010000;
    const DATA32  = 0b00001000;
    const PHASE   = 0b00000100;
    const FILT1   = 0b00000010;
    const FILT0   = 0b00000001;
  }
}

register! {
  /// CTRL4
  pub struct Ctrl4: 0x4: u8 {
    const DIR3 = 0b01000000;
    const DIR2 = 0b00100000;
    const DIR1 = 0b00010000;
    const DIO3 = 0b00000100;
    const DIO2 = 0b00000010;
    const DIO1 = 0b00000001;
  }
}

register! {
  /// CTRL5
  pub struct Ctrl5: 0x5: u8 {
    const CAL1   = 0b10000000;
    const CAL0   = 0b01000000;
    const NOSYSG = 0b00001000;
    const NOSYSO = 0b00000100;
    const NOSCG  = 0b00000010;
    const NOSCO  = 0b00000001;

    const CAL = Self::CAL1.bits() | Self::CAL0.bits();
  }
}

register! {
  /// 32-bit DATA
  pub struct Data32(u32): 0x6;
}

register! {
  /// 24-bit DATA
  pub struct Data24(u24): 0x6;
}

register! {
  /// SOC_SPI
  pub struct SocSpi(u24): 0x7;
}

register! {
  /// SGC_SPI
  pub struct SgcSpi(u24): 0x8;
}

register! {
  /// SCOC_SPI
  pub struct ScocSpi(u24): 0x9;
}

register! {
  /// SCGC_SPI
  pub struct ScgcSpi(u24): 0xA;
}

register! {
  /// HPF
  pub struct Hpf(u16): 0xB;
}

register! {
  /// SOC_ADC
  pub struct SocAdc(u24): 0x15;
}

register! {
  /// SGC_ADC
  pub struct SgcAdc(u24): 0x16;
}

register! {
  /// SCOC_ADC
  pub struct ScocAdc(u24): 0x17;
}

register! {
  /// SCGC_ADC
  pub struct ScgcAdc(u24): 0x18;
}

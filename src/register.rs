pub trait ReadReg<R>
where
  Self: Sized,
{
  const ADDR: u8;

  fn from_reg(reg: R) -> Self;
}

pub trait WriteReg<R>: ReadReg<R> {

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
  (
    #[doc = $name:expr]
    $vis:vis struct $Reg:ident($RegTy:ty): $addr:literal;
  ) => {
    #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
    $vis struct $Reg($RegTy);

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
        const $const_name:ident = $const_value:expr;
      )*
    }
  ) => {
    ::bitflags::bitflags! {
      #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
      $vis struct $Reg: $RegTy {
        $(
          const $const_name = $const_value;
        )*
      }
    }

    register!(@impl_read_reg $Reg: $addr: $RegTy);
  };
}

#[allow(non_camel_case_types)]
pub struct u24([u8; 3]);

impl u24 {
  pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
    Self(bytes)
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
  }
}

register! {
  /// CTRL1
  pub struct Ctrl1: 0x1: u8 {
    const EXTCK  = 0b10000000;
    const SYNC   = 0b01000000;
    const PD1    = 0b00100000;
    const PD0    = 0b00010000;
    const UB     = 0b00001000;
    const FORMAT = 0b00000100;
    const SCYCLE = 0b00000010;
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
    const CAL1 = 0b10000000;
    const CAL0 = 0b01000000;

    const NOSYSG = 0b00001000;
    const NOSYSO = 0b00000100;
    const NOSCG  = 0b00000010;
    const NOSCO  = 0b00000001;
  }
}

register! {
  /// DATA
  pub struct Data(u32): 0x6;
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

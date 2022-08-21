//! Driver for MAX11214 and similar 24-bit Delta-Sigma ADCs.
//!
//! Implemented according to <https://datasheets.maximintegrated.com/en/ds/MAX11214.pdf>.
#![no_std]
#![deny(missing_docs)]

use core::marker::PhantomData;

use embedded_hal::blocking::{delay::DelayMs, spi::Transfer};

mod command;
use command::Command;
mod error;
pub use error::Error;
mod register;
use register::*;
mod types;
pub use types::*;

/// Marks an ADC in conversion mode.
pub enum Conversion {}

/// Marks an ADC in sleep mode.
pub enum Sleep {}

/// Marks an ADC in standby mode.
pub enum Standby {}

/// A MAX11214 ADC.
pub struct Max11214<SPI, MODE> {
  spi: SPI,
  mode: PhantomData<MODE>,
}

impl<SPI, E, MODE> Max11214<SPI, MODE>
where
  SPI: Transfer<u8, Error = E>
{
  /// Create a new ADC with the given SPI peripheral.
  pub fn new(spi: SPI) -> Max11214<SPI, Standby> {
    Max11214 { spi, mode: PhantomData }
  }

  /// Release the contained SPI peripheral.
  pub fn release(self) -> SPI {
    self.spi
  }

  /// Put the ADC into standby mode.
  pub fn into_standby(mut self) -> Result<Max11214<SPI, Standby>, Error<E>> {
    self.modify_reg_u8(|ctrl1: Ctrl1| {
      ctrl1.union(Ctrl1::PD1).difference(Ctrl1::PD0)
    })?;

    self.write_cmd(Command::power_down())?;
    Ok(Max11214 { spi: self.spi, mode: PhantomData })
  }

  /// Put the ADC into sleep mode.
  pub fn into_sleep(mut self) -> Result<Max11214<SPI, Sleep>, Error<E>> {
    self.modify_reg_u8(|ctrl1: Ctrl1| {
      ctrl1.difference(Ctrl1::PD1).union(Ctrl1::PD0)
    })?;

    self.write_cmd(Command::power_down())?;
    Ok(Max11214 { spi: self.spi, mode: PhantomData })
  }

  /// Start conversion.
  pub fn start_conversion(mut self, rate: ConversionSpeed) -> Result<Max11214<SPI, Conversion>, Error<E>> {
    self.modify_reg_u8(|ctrl1: Ctrl1| {
      ctrl1.difference(Ctrl1::PD1).difference(Ctrl1::PD0)
    })?;

    self.write_cmd(Command::convert(rate))?;
    Ok(Max11214 { spi: self.spi, mode: PhantomData })
  }

  /// Get the system status.
  pub fn status(&mut self) -> Result<Status, Error<E>> {
    let stat = self.read_reg_u16::<Stat>()?;
    Ok(Status { status: stat })
  }

  fn write_cmd(&mut self, cmd: Command) -> Result<(), Error<E>> {
    let mut cmd = [cmd.bits()];
    self.spi.transfer(&mut cmd).map_err(|err| Error::Spi(err))?;
    Ok(())
  }

  fn modify_reg_u8<R>(&mut self, f: impl FnOnce(R) -> R) -> Result<(), Error<E>>
  where
    R: WriteReg<u8> + PartialEq + Copy,
  {
    let reg = self.read_reg_u8::<R>()?;
    let new_reg = f(reg);

    if new_reg != reg {
      self.write_reg_u8(new_reg)?;
    }

    Ok(())
  }

  fn write_reg_u8<R>(&mut self, reg: R) -> Result<R, Error<E>>
  where
    R: WriteReg<u8>,
  {
    let mut buf = [
      Command::register_write(R::ADDR).bits(),
      reg.to_reg(),
    ];

    self.spi.transfer(buf.as_mut()).map_err(|err| Error::Spi(err))?;

    Ok(R::from_reg(buf[1]))
  }


  fn read_reg_u8<R>(&mut self) -> Result<R, Error<E>>
  where
    R: ReadReg<u8>,
  {
    let mut buf = [
      Command::register_read(R::ADDR).bits(),
      0,
    ];

    self.spi.transfer(buf.as_mut()).map_err(|err| Error::Spi(err))?;

    Ok(R::from_reg(buf[1]))
  }

  fn read_reg_u16<R>(&mut self) -> Result<R, Error<E>>
  where
    R: ReadReg<u16>,
  {
    let mut buf = [
      Command::register_read(R::ADDR).bits(),
      0,
      0,
    ];

    self.spi.transfer(buf.as_mut()).map_err(|err| Error::Spi(err))?;

    Ok(R::from_reg(u16::from_be_bytes([buf[1], buf[2]])))
  }

  fn read_reg_u24<R>(&mut self) -> Result<R, Error<E>>
  where
    R: ReadReg<u24>,
  {
    let mut buf = [
      Command::register_read(R::ADDR).bits(),
      0,
      0,
      0,
    ];

    self.spi.transfer(buf.as_mut()).map_err(|err| Error::Spi(err))?;

    Ok(R::from_reg(u24::from_be_bytes([buf[1], buf[2], buf[3]])))
  }

  #[allow(unused)]
  fn read_reg_u32<R>(&mut self) -> Result<R, Error<E>>
  where
    R: ReadReg<u32>,
  {
    let mut buf = [
      Command::register_read(R::ADDR).bits(),
      0,
      0,
      0,
      0,
    ];

    self.spi.transfer(buf.as_mut()).map_err(|err| Error::Spi(err))?;

    Ok(R::from_reg(u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]])))
  }
}


impl<SPI, E> Max11214<SPI, Conversion>
where
  SPI: Transfer<u8, Error = E>
{
  /// Read data.
  pub fn data(&mut self) -> Result<u32, Error<E>> {
    let soc_adc = self.read_reg_u24::<ScgcAdc>()?;
    Ok(soc_adc.0.into())
  }
}

macro_rules! impl_sleep_standby {
  () => {
    /// Set the system clock source.
    pub fn set_clock(&mut self, clock: ClockSource) -> Result<(), Error<E>> {
      self.modify_reg_u8(|ctrl1: Ctrl1| {
        match clock {
          ClockSource::External => ctrl1.union(Ctrl1::EXTCK),
          ClockSource::Internal => ctrl1.difference(Ctrl1::EXTCK),
        }
      })
    }

    /// Set the bipolar range format.
    pub fn set_format(&mut self, format: Format) -> Result<(), Error<E>> {
      self.modify_reg_u8(|ctrl1: Ctrl1| {
        match format {
          Format::OffsetBinary => ctrl1.union(Ctrl1::FORMAT),
          Format::TwosComplement => ctrl1.difference(Ctrl1::FORMAT),
        }
      })
    }

    /// Set the PGA gain.
    pub fn set_pga(&mut self, pga: Option<Pga>) -> Result<(), Error<E>> {
      self.modify_reg_u8(|ctrl2: Ctrl2| {
        if let Some(pga) = pga {
          ctrl2.union(Ctrl2::PGAEN).difference(Ctrl2::PGAG).union(Ctrl2::from_bits_truncate(match pga {
            Pga::X1 => 0b000,
            Pga::X2 => 0b001,
            Pga::X4 => 0b010,
            Pga::X8 => 0b011,
            Pga::X16 => 0b100,
            Pga::X32 => 0b101,
            Pga::X64 => 0b110,
            Pga::X128 => 0b111,
          }))
        } else {
          ctrl2.difference(Ctrl2::PGAEN)
        }
      })
    }

    /// Run a self-calibration.
    pub fn self_calibrate<D: DelayMs<u32>>(&mut self, delay: &mut D, calibration: Calibration) -> Result<(), Error<E>> {
      self.modify_reg_u8(|ctrl1: Ctrl5| {
        match calibration {
          Calibration::SelfCalibration => ctrl1.difference(Ctrl5::CAL),
          Calibration::SystemOffsetCalibration => ctrl1.difference(Ctrl5::CAL1).union(Ctrl5::CAL0),
          Calibration::SystemFullScaleCalibration => ctrl1.union(Ctrl5::CAL1).difference(Ctrl5::CAL0),
        }
      })?;

      self.write_cmd(Command::calibrate())?;

      match calibration {
        Calibration::SelfCalibration => delay.delay_ms(200),
        _ => delay.delay_ms(100),
      }

      Ok(())
    }

    /// Get the system offset calibration value.
    pub fn system_offset_calibration_value(&mut self) -> Result<u32, Error<E>> {
      let soc_adc = self.read_reg_u24::<SocAdc>()?;
      Ok(soc_adc.0.into())
    }

    /// Get the system gain calibration value.
    pub fn system_gain_calibration_value(&mut self) -> Result<u32, Error<E>> {
      let soc_adc = self.read_reg_u24::<SgcAdc>()?;
      Ok(soc_adc.0.into())
    }

    /// Get the system self-calibration offset calibration value.
    pub fn self_calibration_offset_calibration_value(&mut self) -> Result<u32, Error<E>> {
      let soc_adc = self.read_reg_u24::<ScocAdc>()?;
      Ok(soc_adc.0.into())
    }

    /// Get the system self-calibration gain calibration value.
    pub fn self_calibration_gain_calibration_value(&mut self) -> Result<u32, Error<E>> {
      let soc_adc = self.read_reg_u24::<ScgcAdc>()?;
      Ok(soc_adc.0.into())
    }
  }
}

impl<SPI, E> Max11214<SPI, Sleep>
where
  SPI: Transfer<u8, Error = E>
{
  impl_sleep_standby!();
}

impl<SPI, E> Max11214<SPI, Standby>
where
  SPI: Transfer<u8, Error = E>
{
  impl_sleep_standby!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#![no_std]

use embedded_hal::blocking::spi::Transfer;

mod command;
use command::Command;
mod error;
pub use error::Error;
mod register;
use register::*;
mod speed;
pub use speed::ConversionSpeed;

pub struct Max11214<SPI> {
  spi: SPI,
}

impl<SPI, E> Max11214<SPI>
where
  SPI: Transfer<u8, Error = E>
{
  pub fn read_u8<R>(&mut self) -> Result<R, Error<E>>
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

  pub fn read_u16<R>(&mut self) -> Result<R, Error<E>>
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

  pub fn read_u24<R>(&mut self) -> Result<R, Error<E>>
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

  pub fn read_u32<R>(&mut self) -> Result<R, Error<E>>
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

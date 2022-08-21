/// An ADC error.
#[derive(Debug, Clone)]
pub enum Error<SPI> {
  /// SPI error.
  Spi(SPI)
}

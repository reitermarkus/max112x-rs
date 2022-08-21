/// An ADC error.
pub enum Error<SPI> {
  /// SPI error.
  Spi(SPI)
}

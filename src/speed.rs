/// Conversion speed (samples per second).
#[derive(Debug, Clone, Copy)]
pub enum ConversionSpeed {
  /// 0.95 with SINC filter, 25 with single-cycle conversion
  Rate0  = 0b0000,
  /// 1.9 with SINC filter, 31.25 with single-cycle conversion
  Rate1  = 0b0001,
  /// 3.9 with SINC filter, 50 with single-cycle conversion
  Rate2  = 0b0010,
  /// 7.8 with SINC filter, 62.5 with single-cycle conversion
  Rate3  = 0b0011,
  /// 15.6 with SINC filter, 100 with single-cycle conversion
  Rate4  = 0b0100,
  /// 31.25 with SINC filter, 125 with single-cycle conversion
  Rate5  = 0b0101,
  /// 62.5 with SINC filter (supports FIR filter), 200 with single-cycle conversion
  Rate6  = 0b0110,
  /// 125 with SINC filter (supports FIR filter), 250 with single-cycle conversion
  Rate7  = 0b0111,
  /// 250 with SINC filter (supports FIR filter), 400 with single-cycle conversion
  Rate8  = 0b1000,
  /// 500 with SINC filter (supports FIR filter), 500 with single-cycle conversion
  Rate9  = 0b1001,
  /// 1000 with SINC filter (supports FIR filter), 800 with single-cycle conversion
  Rate10 = 0b1010,
  /// 2000 with SINC filter (supports FIR filter), 1000 with single-cycle conversion
  Rate11 = 0b1011,
  /// 4000 with SINC filter (supports FIR filter), 1600 with single-cycle conversion
  Rate12 = 0b1100,
  /// 8000 with SINC filter (supports FIR filter), 2000 with single-cycle conversion
  Rate13 = 0b1101,
  /// 16000 with SINC filter, 3200 with single-cycle conversion
  Rate14 = 0b1110,
  /// 32000 with SINC filter, 6400 with single-cycle conversion
  Rate15 = 0b1111,
}

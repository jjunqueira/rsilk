use std::path::Path;

/// # VarLenHeader
///
/// This part of the silk header. They contain different things
/// like the cli command used to create the file. For some file types the
// variable length header also contains the year/month/day/hour of the file.
pub struct VarLenHeader {
  pub id: u32,
  pub length: u32,
  pub content: Vec<u8>,
}

/// # Silk file header
///
/// Full Description: https://tools.netsa.cert.org/silk/faq.html#file-header
pub struct Header {
  pub magic_number: [u8; 4],
  pub file_flags: u8,
  pub record_format: u8,
  pub file_version: u8,
  pub compression: u8,
  pub silk_version: u32,
  pub record_size: u16,
  pub record_version: u16,
  pub var_len_headers: Vec<VarLenHeader>,
  pub header_length: isize,
  pub file_date_ms: u64,
  pub file_sensor: u32,
}

pub fn parse(_path: &Path) -> Header {
  return Header {
    magic_number: [b'0', b'0', b'0', b'0'],
    file_flags: 0,
    record_format: 0,
    file_version: 0,
    compression: 0,
    silk_version: 0,
    record_size: 0,
    record_version: 0,
    var_len_headers: Vec::new(),
    header_length: 0,
    file_date_ms: 0,
    file_sensor: 0,
  };
}

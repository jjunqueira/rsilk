use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub enum HeaderParseError {
  IOError(io::Error),
  UnableToReadHeader,
}

impl From<io::Error> for HeaderParseError {
  fn from(error: io::Error) -> Self {
    HeaderParseError::IOError(error)
  }
}

/// # VarLenHeader
///
/// This part of the silk header. They contain different things
/// like the cli command used to create the file. For some file types the
// variable length header also contains the year/month/day/hour of the file.
#[derive(Debug)]
pub struct VarLenHeader {
  pub id: u32,
  pub length: u32,
  pub content: Vec<u8>,
}

/// # Silk file header
///
/// Full Description: https://tools.netsa.cert.org/silk/faq.html#file-header
#[derive(Debug)]
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

impl Header {
  pub fn parse(path: &Path) -> Result<Header, HeaderParseError> {
    let mut f = File::open(path)?;
    let mut header_buf: [u8; 16] = [0; 16];
    let read_size = f.read(&mut header_buf)?;
    if read_size != 16 {
      return Err(HeaderParseError::UnableToReadHeader);
    }
    Ok(header_from_bytes(&header_buf))
  }
}

fn header_from_bytes(bytes: &[u8; 16]) -> Header {
  let mut magic_number: [u8; 4] = Default::default();
  let mut silk_version: [u8; 4] = Default::default();
  let mut record_size: [u8; 2] = Default::default();
  let mut record_version: [u8; 2] = Default::default();
  magic_number.copy_from_slice(&bytes[0..4]);
  silk_version.copy_from_slice(&bytes[8..12]);
  record_size.copy_from_slice(&bytes[12..14]);
  record_version.copy_from_slice(&bytes[14..16]);
  Header {
    magic_number: magic_number,
    file_flags: bytes[4],
    record_format: bytes[5],
    file_version: bytes[6],
    compression: bytes[7],
    silk_version: u32::from_be_bytes(silk_version),
    record_size: u16::from_be_bytes(record_size),
    record_version: u16::from_be_bytes(record_version),
    var_len_headers: Vec::new(),
    header_length: 0,
    file_date_ms: 0,
    file_sensor: 0,
  }
}

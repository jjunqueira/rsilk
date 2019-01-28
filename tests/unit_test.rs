#[cfg(test)]
mod tests {

  use rsilk::header::Header;
  use std::path::Path;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_can_parse_header() {
    let path = Path::new("tests/data/FT_RWIPV6-v1-c0-B.dat");
    let parsed_header = Header::parse(path).unwrap();
    assert_eq!(parsed_header.compression, 0);
  }
}

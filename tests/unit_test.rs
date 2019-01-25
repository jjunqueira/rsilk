extern crate rsilk;

#[cfg(test)]
mod tests {

  use rsilk::header;
  use std::path::Path;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_can_parse_header() {
    let path = Path::new("./foo/bar.txt");
    let parsed_header = header::parse(path);
    assert_eq!(parsed_header.compression, 0);
  }
}

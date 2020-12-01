use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_into_vec(path_string: String) -> Vec<String> {
  let path = Path::new(&path_string);
  println!("{}", path.display());
  let file = File::open(path).expect("File not found!");
  let buffer = BufReader::new(file);
  buffer
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}

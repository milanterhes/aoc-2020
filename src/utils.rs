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

pub struct PasswordRule {
  pub min: i32,
  pub max: i32,
  pub character: char,
}

pub fn parse_password(raw_pswd: &str) -> (PasswordRule, &str) {
  let f = raw_pswd.split(":").collect::<Vec<&str>>();
  let password = &f[1];

  let boundaries_with_char = &f[0].trim().split(" ").collect::<Vec<&str>>();
  let boundaries = boundaries_with_char[0].split("-").collect::<Vec<&str>>();
  (
    PasswordRule {
      min: boundaries[0].parse::<i32>().unwrap(),
      max: boundaries[1].parse::<i32>().unwrap(),
      character: boundaries_with_char[1].chars().nth(0).unwrap(),
    },
    password,
  )
}

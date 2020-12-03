use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_into_vec(path_string: String) -> Vec<String> {
  let path = Path::new(&path_string);
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

fn step(pos: &(i32, i32), last_index: i32, right: i32, down: i32) -> (i32, i32) {
  let new_char_idx: i32;
  if pos.0 + right > last_index {
    let remainder = last_index - pos.0;
    new_char_idx = right - remainder - 1;
  } else {
    new_char_idx = pos.0 + right;
  }
  (new_char_idx, pos.1 + down)
}

pub fn calculate_slope(input: &Vec<std::string::String>, right: i32, down: i32) -> i32 {
  let mut counter = 0;
  let biome_width = (&input[0].len() - 1) as i32;
  let mut pos = (0, 0);

  while pos.1 != (input.len()) as i32 {
    let c = input[pos.1 as usize].chars().nth(pos.0 as usize).unwrap();
    if c.to_string() == "#" {
      counter = counter + 1;
    }
    if pos.1 + down >= input.len() as i32 {
      break;
    }
    pos = step(&pos, biome_width, right, down);
  }
  counter
}

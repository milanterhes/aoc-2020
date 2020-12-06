use regex::Regex;
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

pub fn parse_passports(input: Vec<String>) -> Vec<Vec<(String, String)>> {
  let mut passport_strings: Vec<String> = vec![];

  let mut string_holder: String = "".to_string();

  for line in input {
    if line.is_empty() {
      passport_strings.push(string_holder.to_string());
      string_holder = "".to_string();
    } else {
      if string_holder == "" {
        string_holder = line;
      } else {
        string_holder = format!("{} {}", string_holder, line);
      }
    }
  }

  let mut passports: Vec<Vec<(String, String)>> = vec![];

  for line in passport_strings {
    let fields: Vec<String> = line.split(" ").map(|f| f.to_string()).collect();
    let pairs = fields
      .iter()
      .map(|f| {
        let sp = f.split(":").collect::<Vec<&str>>();
        (sp[0].to_string(), sp[1].to_string())
      })
      .collect();
    passports.push(pairs);
  }

  passports
}

pub fn validate_passport(passport: &Vec<(String, String)>) -> bool {
  let mut valid = true;

  let mut has_cid = false;

  for field in passport {
    match field.0.as_str() {
      "byr" => {
        let as_num = field.1.parse::<i32>().unwrap();
        if !(field.1.len() == 4 && as_num >= 1920 && as_num <= 2002) {
          valid = false;
          break;
        }
      }
      "iyr" => {
        let as_num = field.1.parse::<i32>().unwrap();
        if !(field.1.len() == 4 && as_num >= 2010 && as_num <= 2020) {
          valid = false;
          break;
        }
      }
      "eyr" => {
        let as_num = field.1.parse::<i32>().unwrap();
        if !(field.1.len() == 4 && as_num >= 2020 && as_num <= 2030) {
          valid = false;
          break;
        }
      }
      "hgt" => {
        let (value, unit) = field.1.split_at(field.1.len() - 2);
        match unit {
          "cm" => {
            let as_num = value.parse::<i32>().unwrap();
            if !(as_num <= 193 && as_num >= 150) {
              valid = false;
              break;
            }
          }
          "in" => {
            let as_num = value.parse::<i32>().unwrap();
            if !(as_num <= 76 && as_num >= 59) {
              valid = false;
              break;
            }
          }
          _ => {
            valid = false;
            break;
          }
        }
      }
      "hcl" => {
        let re = Regex::new(r"#[0-9a-f]").unwrap();
        if !re.is_match(&field.1) && field.1.len() != 7 {
          valid = false;
          break;
        }
      }
      "ecl" => {
        if !(vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
          .iter()
          .any(|x| x == &field.1))
        {
          valid = false;
          break;
        }
      }
      "pid" => {
        if field.1.len() != 9 {
          valid = false;
          break;
        }
      }
      "cid" => {
        has_cid = true;
      }
      _ => (),
    }
  }

  if !(has_cid && passport.len() == 8 || !has_cid && passport.len() == 7) {
    return false;
  }

  valid
}

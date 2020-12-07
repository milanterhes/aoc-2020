use crate::utils::*;
use std::io::{Error, ErrorKind, Result};

pub fn day1_1() -> Result<i32> {
  let input = read_into_vec(String::from("src/inputs/1.txt"));
  for line1 in &input {
    for line2 in &input {
      let numbers = (line1.parse::<i32>().unwrap(), line2.parse::<i32>().unwrap());
      if numbers.0 + numbers.1 == 2020 {
        let result: i32 = numbers.0 * numbers.1;
        return Ok(result);
      }
    }
  }
  Err(Error::new(ErrorKind::NotFound, "Pair not found!"))
}

// TODO: find something faster than O(n^3)
pub fn day1_2() -> Result<i32> {
  let input = read_into_vec(String::from("src/inputs/1.txt"));
  for line1 in &input {
    for line2 in &input {
      for line3 in &input {
        let numbers = (
          line1.parse::<i32>().unwrap(),
          line2.parse::<i32>().unwrap(),
          line3.parse::<i32>().unwrap(),
        );
        if numbers.0 + numbers.1 + numbers.2 == 2020 {
          let result: i32 = numbers.0 * numbers.1 * numbers.2;
          return Ok(result);
        }
      }
    }
  }
  Err(Error::new(ErrorKind::NotFound, "Pair not found!"))
}

pub fn day2_1() -> i32 {
  let input = read_into_vec(String::from("src/inputs/2.txt"));

  fn validate_password(raw_pswd: &str) -> bool {
    let (rule, password) = parse_password(raw_pswd);
    let mut char_counter = 0;
    for x in password.chars() {
      if x == rule.character {
        char_counter = char_counter + 1;
      }
    }
    char_counter >= rule.min && char_counter <= rule.max
  }

  let mut valid_counter = 0;

  for pwd in &input {
    if validate_password(pwd) {
      valid_counter = valid_counter + 1;
    }
  }
  valid_counter
}

pub fn day2_2() -> i32 {
  let input = read_into_vec(String::from("src/inputs/2.txt"));

  fn char_at(p: &str, idx: usize) -> char {
    match p.chars().nth(idx) {
      Some(x) => x,
      None => " ".chars().nth(0).unwrap(),
    }
  }

  fn validate_password(raw_pswd: &str) -> bool {
    let (rule, password) = parse_password(raw_pswd);

    (char_at(password, rule.min as usize) == rule.character
      && char_at(password, rule.max as usize) != rule.character)
      || (char_at(password, rule.min as usize) != rule.character
        && char_at(password, rule.max as usize) == rule.character)
  }

  let mut valid_counter = 0;

  for pwd in &input {
    if validate_password(pwd) {
      valid_counter = valid_counter + 1;
    }
  }
  valid_counter
}

pub fn day3_1() -> i32 {
  let input = read_into_vec(String::from("src/inputs/3.txt"));

  calculate_slope(&input, 3, 1)
}

pub fn day3_2() -> u128 {
  let input = read_into_vec(String::from("src/inputs/3.txt"));

  let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  let solutions = slopes
    .iter()
    .map(|&slope| calculate_slope(&input, slope.0, slope.1))
    .collect::<Vec<i32>>();

  solutions.iter().fold(1 as u128, |acc, x| acc * *x as u128)
}

pub fn day4_1() -> i32 {
  let input = read_into_vec(String::from("src/inputs/4.txt"));
  let passports = parse_passports(input);
  let mut counter = 0;
  for p in passports {
    let has_cid = p.iter().any(|pair| pair.0 == "cid");
    if has_cid && p.len() == 8 {
      counter = counter + 1;
    } else if !has_cid && p.len() == 7 {
      counter = counter + 1;
    }
  }
  counter
}

pub fn day4_2() -> i32 {
  let input = read_into_vec(String::from("src/inputs/4.txt"));
  let passports = parse_passports(input);
  passports.iter().fold(0, |acc, p| {
    if validate_passport(p) {
      return acc + 1;
    }
    acc
  })
}

pub fn day5_1() -> i32 {
  let input = read_into_vec(String::from("src/inputs/5.txt"));
  let mut highest_id = 0;

  for line in input {
    let (row, column) = line.split_at(line.len() - 3);
    let row_sol = find_number(row, true);
    let column_sol = find_number(column, false);

    let seat_id = row_sol * 8 + column_sol;
    if seat_id > highest_id {
      highest_id = seat_id;
    }
  }

  highest_id
}

pub fn day5_2() -> i32 {
  let input = read_into_vec(String::from("src/inputs/5.txt"));
  let mut id_list: Vec<i32> = vec![];

  for line in input {
    let (row, column) = line.split_at(line.len() - 3);
    let row_sol = find_number(row, true);
    let column_sol = find_number(column, false);

    let seat_id = row_sol * 8 + column_sol;

    id_list.push(seat_id);
  }

  id_list.sort();

  for i in 0..id_list.len() {
    let above_id = ((0 + id_list.len() - 1) / 2) + i;
    let below_id = ((0 + id_list.len() - 1) / 2) - i;
    if (above_id as i32) < id_list.len() as i32 - 2 {
      if (id_list[above_id - 1] - id_list[above_id + 1]).abs() != 2 {
        return id_list[above_id] + 1;
      }
    }

    if (below_id as i32) > 1 {
      if (id_list[below_id - 1] - id_list[below_id + 1]).abs() != 2 {
        return id_list[below_id] - 1;
      }
    }
  }
  panic!("Cannot find your seat id")
}

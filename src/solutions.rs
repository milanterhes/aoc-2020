use crate::utils;
use std::io::{Error, ErrorKind, Result};

pub fn day1_1() -> Result<i32> {
  let input = utils::read_into_vec(String::from("src/inputs/1.txt"));
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

pub fn day1_2() -> Result<i32> {
  let input = utils::read_into_vec(String::from("src/inputs/1.txt"));
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

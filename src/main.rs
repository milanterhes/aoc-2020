mod solutions;
mod utils;

fn main() {
    println!("Hello, AOC 2020!");
    println!("day1_1 {}", solutions::day1_1().unwrap());
    println!("day1_2 {}", solutions::day1_2().unwrap());
    println!("day2_1 {}", solutions::day2_1());
    println!("day2_2 {}", solutions::day2_2());
    println!("day3_1 {}", solutions::day3_1());
    println!("day3_2 {}", solutions::day3_2());
    println!("day4_1 {:?}", solutions::day4_1());
    println!("day4_2 {:?}", solutions::day4_2());
}

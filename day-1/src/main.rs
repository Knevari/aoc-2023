use std::fs;
use std::collections::HashMap;

fn main() {
  let digits = HashMap::from([
    (String::from("one"), 1),
    (String::from("two"), 2),
    (String::from("three"), 3),
    (String::from("four"), 4),
    (String::from("five"), 5),
    (String::from("six"), 6),
    (String::from("seven"), 7),
    (String::from("eight"), 8),
    (String::from("nine"), 9)
  ]);

  let contents = fs::read_to_string("./numbers.txt").unwrap();
  let contents = contents.trim();

  let mut sum = 0;

  for line in contents.lines() {

    let mut left: usize = 0;
    let mut right: usize = line.len() - 1;

    let mut first_digit: u32 = 0;
    let mut second_digit: u32 = 0;

    println!("{}", line);

    'outer_1: loop {
      if left >= line.len() {
        break;
      }

      let c = line.chars().nth(left).unwrap();

      if c.is_numeric() {
        first_digit = c.to_digit(10).unwrap();
        break;
      }

      if c.is_alphabetic() {
        // Verify if it is a substring of any number
        for key in digits.keys() {
          let mut is_valid = true;

          for i in 0..key.len() {
            if line.chars().nth(left + i).unwrap() != key.chars().nth(i).unwrap() {
              is_valid = false;
              break;
            }
          }

          if is_valid {
            first_digit = digits[key];
            break 'outer_1;
          }
        }
      }
      
      left += 1;
    }

    // Iterate over elements from right to left and try to match a digit
    'outer_2: loop {
      let c = line.chars().nth(right).unwrap();
      
      // If it is number, set immediately 
      if c.is_numeric() {
        second_digit = c.to_digit(10).unwrap();
        break;
      }

      if c.is_alphabetic() {
        for key in digits.keys() {
          let mut is_valid = true;

          for i in 0..key.len() {
            let key_index = key.len() - i - 1;
            let (_, overflow) = right.overflowing_sub(i);

            if overflow {
              break;
            }

            if key.chars().nth(key_index).unwrap() != line.chars().nth(right - i).unwrap() {
              is_valid = false;
              break;
            }
          }

          if is_valid {
            second_digit = digits[key];
            break 'outer_2;
          }
        }
      }

      let (_, overflow) = right.overflowing_sub(1);

      if overflow {
        break 'outer_2;
      } else {
        right -= 1;
      }
    }
  
    sum +=  first_digit * 10 + second_digit;
  }

  println!("The sum is equal to: {}", sum);
}
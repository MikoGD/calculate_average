use std::collections::HashMap;
use std::io::{self, prelude::*};

enum MenuOption {
  Enter,
  Average,
  Median,
  Mode,
  Exit,
}

#[derive(Copy, Clone)]
struct FloatNum {
  integral: i32,
  fractional: i32,
}

impl FloatNum {
  fn new(integral: i32, fractional: i32) -> FloatNum {
    FloatNum {
      integral,
      fractional,
    }
  }

  fn to_string(self) -> String {
    format!("{}.{}", self.integral, self.fractional)
  }

  fn to_float(self) -> f32 {
    format!("{}.{}", self.integral, self.fractional)
      .parse()
      .expect("Error not a number")
  }
}

fn main() {
  let mut number_set: Vec<FloatNum> = vec![];
  loop {
    display_menu(&number_set);
    let user_input = get_menu_option();

    match user_input {
      MenuOption::Enter => number_set = get_number_set(),
      MenuOption::Average => get_average(&number_set),
      MenuOption::Median => get_median(&number_set),
      MenuOption::Mode => get_mode(&number_set),
      MenuOption::Exit => {
        println!("\nExiting...");
        break;
      }
    }
  }
}

fn display_menu(number_set: &Vec<FloatNum>) {
  println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
  println!("Welcome To Average Calculator");
  print!("\nCurrent number set [");

  for num in number_set.iter() {
    print!("{}, ", num.to_string());
  }

  println!("]\n");
  println!("1) Enter number set");
  println!("2) Get average");
  println!("3) Get median");
  println!("4) Get mode");
  println!("5) Exit");
  print!("> ");

  io::stdout().flush().ok().expect("Could not flush stdout");
}

fn get_menu_option() -> MenuOption {
  let user_input: MenuOption = loop {
    let mut user_input = String::new();

    io::stdin()
      .read_line(&mut user_input)
      .expect("Failed to read input");
    let user_input: MenuOption = match user_input.trim().parse() {
      Ok(num) => match num {
        1 => MenuOption::Enter,
        2 => MenuOption::Average,
        3 => MenuOption::Median,
        4 => MenuOption::Mode,
        5 => MenuOption::Exit,
        _ => {
          println!("Error: invalid menu option");
          println!("Please enter a valid menu option");

          continue;
        }
      },
      Err(_) => {
        println!("Error: invalid menu option");
        println!("Please enter a valid menu option");

        continue;
      }
    };

    break user_input;
  };

  user_input
}

fn get_number() -> Option<FloatNum> {
  println!("\nEnter a number to add to the set or enter \"Done\" to finish");
  let user_input: Option<FloatNum> = loop {
    let mut user_input = String::new();

    io::stdin()
      .read_line(&mut user_input)
      .expect("Failed to read input");
    let user_input = user_input.trim().to_lowercase();
    if user_input == "done" {
      break None;
    } else if user_input == "" {
      println!("Error invalid input");
      println!("Input must be a number\n");
      continue;
    } else if is_any_number(&user_input) == false {
      println!("Error invalid input");
      println!("Input must be a number\n");
      continue;
    }

    let mut decimal_index: usize = 0;

    for (index, letter) in user_input.chars().enumerate() {
      if letter == '.' {
        decimal_index = index;
      } else if letter.is_digit(10) == false {
        println!("Error invalid input");
        println!("Input must be a number\n");
        continue;
      }
    }

    let integral: String;
    let fractional: String;

    if decimal_index == 0 {
      integral = user_input.to_string();
      fractional = String::from("0");
    } else {
      integral = user_input[0..decimal_index].to_string();
      fractional = user_input[decimal_index + 1..].to_string();
    }

    if integral.parse::<i32>().is_ok() || fractional.parse::<i32>().is_ok() {
      break Some(FloatNum::new(
        integral.parse::<i32>().expect("Not a number"),
        fractional.parse::<i32>().expect("Not a number"),
      ));
    } else {
      println!("Error invalid input");
      println!("Input must be a number\n");
      continue;
    }
  };

  user_input
}

fn is_any_number(user_input: &String) -> bool {
  for character in user_input.chars() {
    if character.is_digit(10) || character == '.' {
      continue;
    } else {
      return false;
    }
  }

  true
}

fn get_number_set() -> Vec<FloatNum> {
  let mut number_set: Vec<FloatNum> = Vec::new();

  loop {
    let number = get_number();

    match number {
      Some(num) => {
        number_set.push(num);
      }
      None => break,
    }
  }

  println!("The number set contains: ");

  for num in number_set.iter() {
    print!("{} ", num.to_string());
  }

  number_set
}

fn get_average(number_set: &Vec<FloatNum>) {
  if number_set.len() as i32 == 0 {
    println!("\nNumber set is empty");
    return;
  }

  let mut total: f32 = 0.0;

  for num in number_set.iter() {
    total += num.to_float();
  }

  let average: f32 = total / number_set.len() as f32;

  println!("\nThe average of your number set is {}", average);
}

fn get_median(number_set: &Vec<FloatNum>) {
  if number_set.len() as i32 == 0 {
    println!("\nNumber set is empty");
    return;
  } else if number_set.len() as i32 == 1 {
    println!(
      "\nThe median of the number set is {}",
      number_set.get(0).unwrap().to_string()
    );

    return;
  }
  let mut number_set_copy = Vec::new();

  for nums in number_set.iter() {
    number_set_copy.push(nums.to_float());
  }

  for nums in number_set_copy.iter() {
    println!("{}", nums);
  }

  number_set_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let median: f32;
  if number_set_copy.len() as i32 % 2 == 0 {
    let lower_half = (number_set_copy.len() as i32 / 2) as i32 - 1;
    let upper_half = lower_half + 1;

    let lower_half = number_set_copy.get(lower_half as usize);
    let upper_half = number_set_copy.get(upper_half as usize);

    median = (lower_half.unwrap() + upper_half.unwrap()) as f32 / 2.0;
  } else {
    let median_index = (number_set_copy.len()) / 2;
    median = *number_set_copy.get(median_index).unwrap();
  }

  println!("\nThe median of the number set is {}", median);
}

fn get_mode(number_set: &Vec<FloatNum>) {
  if number_set.len() as i32 == 0 {
    println!("\nNumber set is empty");
    return;
  }

  let mut curr_numbers = HashMap::new();

  for num in number_set.iter() {
    let count = curr_numbers.entry(num.to_string()).or_insert(0);
    *count += 1;
  }

  let mut mode: f32 = 0.0;
  let mut highest_value = 0;
  for (key, value) in curr_numbers {
    if highest_value < value {
      highest_value = value;
      mode = key.parse().expect("Failed to parse number");
    }
  }

  println!(
    "\nThe mode of the number set is {} appearing {} times",
    mode, highest_value
  );
}

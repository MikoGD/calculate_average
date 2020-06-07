use std::collections::HashMap;
use std::io::{self, prelude::*};

enum MenuOption {
  Enter,
  Average,
  Median,
  Mode,
  Exit,
}

fn main() {
  let mut number_set: Vec<i32> = vec![];
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

fn display_menu(number_set: &Vec<i32>) {
  println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
  println!("Welcome To Average Calculator");
  print!("\nCurrent number set [");

  for num in number_set.iter() {
    print!("{}, ", num);
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

fn get_number() -> Option<i32> {
  println!("\nEnter a number to add to the set or enter \"Done\" to finish");
  let user_input: Option<i32> = loop {
    let mut user_input = String::new();

    io::stdin()
      .read_line(&mut user_input)
      .expect("Failed to read input");
    let user_input = user_input.trim();
    if user_input == "Done" {
      break None;
    }

    match user_input.parse() {
      Ok(num) => break Some(num),
      Err(_) => {
        println!("Error: invalid input");
        println!("Please enter an integer");

        continue;
      }
    };
  };

  user_input
}

fn get_number_set() -> Vec<i32> {
  let mut number_set: Vec<i32> = Vec::new();

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
    print!("{} ", num);
  }

  number_set
}

fn get_average(number_set: &Vec<i32>) {
  if number_set.len() as i32 == 0 {
    println!("\nNumber set is empty");
    return;
  }

  let mut total: f32 = 0.0;

  for num in number_set.iter() {
    total += *num as f32;
  }

  let average: f32 = total / number_set.len() as f32;

  println!("\nThe average of your number set is {}", average);
}

fn get_median(number_set: &Vec<i32>) {
  if number_set.len() as i32 == 0 {
    println!("\nNumber set is empty");
    return;
  } else if number_set.len() as i32 == 1 {
    println!(
      "\nThe median of the number set is {}",
      number_set.get(0).unwrap()
    );

    return;
  }
  let mut number_set_copy = number_set.clone();
  number_set_copy.sort();

  let median: f32;
  if number_set_copy.len() as i32 % 2 == 0 {
    let lower_half = (number_set_copy.len() as i32 / 2) as i32 - 1;
    let upper_half = lower_half + 1;

    let lower_half = number_set_copy.get(lower_half as usize);
    let upper_half = number_set_copy.get(upper_half as usize);

    median = (lower_half.unwrap() + upper_half.unwrap()) as f32 / 2.0;
  } else {
    median = (number_set_copy.len() as f32) / 2.0;
  }

  println!("\nThe median of the number set is {}", median);
}

fn get_mode(number_set: &Vec<i32>) {
  if number_set.len() as i32 == 0 {
    println!("\nNumber set is empty");
    return;
  }

  let mut curr_numbers = HashMap::new();

  for num in number_set.iter() {
    let count = curr_numbers.entry(num).or_insert(0);
    *count += 1;
  }

  let mut mode: i32 = 0;
  let mut highest_value = 0;
  for (key, value) in curr_numbers {
    if highest_value < value {
      highest_value = value;
      mode = *key;
    }
  }

  println!(
    "\nThe mode of the number set is {} appearing {} times",
    mode, highest_value
  );
}

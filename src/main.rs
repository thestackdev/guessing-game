use colored::Colorize;
use rand::{prelude::*, rng};
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut thread_rng = rng();
    let magic_number = thread_rng.random_range(0..=100);

    loop {
        print!("Please enter a number: ");
        let mut input_buffer = String::new();
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read input");

        let parsed_number: u32 = match input_buffer.trim().parse() {
            Ok(n) => n,
            _ => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match magic_number.cmp(&parsed_number) {
            Ordering::Less => print!("{}", "You guessed it high".red()),
            Ordering::Greater => print!("{}", "You guessed it low".red()),
            Ordering::Equal => {
                print!("{}", "You guessed it".green());
                break;
            }
        }
        println!();
    }
}

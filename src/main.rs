#![feature(test, transpose_result)]
extern crate rand;
extern crate test;

#[cfg(test)]
mod tests;

mod format;
pub mod group;
mod item;
pub mod quantifier;

use group::Group;
use std::env;
use std::io::{ stdin, stdout, Write };

fn main() {
    if let Err(e) = process() {
        println!("{}", e);
    }
}

fn process() -> Result<(), String> {
    let mut args = env::args();
    args.next()
        .ok_or("Failed to load arguments. This is a bug!")?;

    let path = args.next().ok_or("Requires path")?;

    if let Some(arg) = args.next() {
        let amount = arg.parse().map_err(|e| format!("Failed to parse amount, do to {}", e))?;

        let group = Group::from_path(path).map_err(|e| format!("{}", e))?;

        group.print(amount);
    } else {
        let group = Group::from_path(path).map_err(|e| format!("{}", e))?;

        let mut input = String::new();
        loop {
            print!("amount: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            {
                let input = input.trim();
                if input.is_empty() {break;}
                match input.parse::<usize>() {
                    Ok(amount) => group.print(amount),
                    Err(e) => println!("Failed to parse amount, do to {}", e),
                }
            }
            input.clear();
        }
    }
    Ok(())
}

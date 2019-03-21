#![feature(test)]
extern crate rand;
extern crate test;

#[cfg(test)]
mod tests;

mod format;
pub mod group;
mod item;
pub mod quantifier;
pub mod quantifier_vec;

use group::Group;
use std::env;

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

    let amount = args
        .next()
        .map_or(Ok(1), |p| p.parse())
        .map_err(|e| format!("Failed to parse amount, do to {}", e))?;

    let group = Group::from_path(path).map_err(|e| format!("{}", e))?;

    for (item, q) in group.generate_formated(amount) {
        println!("{}x{}", q, item);
    }
    
    Ok(())
}

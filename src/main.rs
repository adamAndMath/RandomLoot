#![feature(test)]
extern crate test;
extern crate rand;

#[cfg(test)]
mod tests;

mod item;
mod format;
pub mod quantifier;
pub mod group;

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

    let path = args.next()
        .ok_or("Requires path")?;
    
    let amount = args.next()
        .map_or(Ok(1), |p| p.parse())
        .map_err(|e| format!("Failed to parse amount, do to {}", e))?;

    let group = Group::from_path(path)
        .map_err(|e| format!("{}", e))?;
    
    group.print(amount);
    Ok(())
}

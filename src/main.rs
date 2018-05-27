extern crate rand;

mod item;
mod format;
pub mod generator;
pub mod quantifier;
pub mod group;

use group::Group;
use std::env;

fn main() {
    let mut args = env::args();
    let path = args.next().expect("Requires path");
    let amount = args.next().map_or(1, |p| p.parse().expect("Failed to parse amount"));

    match Group::from_path(path) {
        Ok(group) => group.print(amount),
        Err(e) => println!("{}", e),
    }
}

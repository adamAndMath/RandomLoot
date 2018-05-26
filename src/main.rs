extern crate rand;

mod item;
mod format;
pub mod generator;
pub mod quantifier;
pub mod group;

use group::Group;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1).expect("Requires path");
    let amount = args.get(2).map_or(1, |p| p.parse().unwrap());

    match Group::from_path(path) {
        Ok(group) => group.print(amount),
        Err(e) => println!("{}", e),
    }
}

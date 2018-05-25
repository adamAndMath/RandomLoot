extern crate rand;

mod item;
mod format;
pub mod generator;
pub mod quantifier;
mod group;

use group::Group;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1).expect("Requires path");
    let amount = args.get(2).map_or(1, |p| p.parse().unwrap());

    let group = Group::from_path(path).unwrap();
    group.print(amount);
}

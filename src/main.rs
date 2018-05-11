mod item;
use item::Item;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::env;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    let mut typeConsts: HashMap<String, String> = HashMap::new();
    let item = Item::new();

    println!("{}", item);
    let path = args.pop().expect("Requires path");
    let re = read(path);
    println!("{:?}", re);
}

fn read(path: String) -> io::Result<()> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    let s = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();

    println!("{}", s.get(0).expect("No first line"));

    let mut vars = s.get(0).expect("No first line").split("]:[");
    for v in vars {
        println!("{}", v);
    }
    Ok(())
}
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
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    println!("{}", buffer);
    let mut vars = buffer[1..buffer.len()-2].split("]:[");
    for v in vars {
        println!("{}", v);
    }

    Ok(())
}
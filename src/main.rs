mod item;
mod format;

use item::Item;
use format::Format;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut typeConsts: HashMap<String, String> = HashMap::new();
    let item = Item::new();
    //println!("{}", item);

    let path = args.last().expect("Requires path");
    println!("{}", path);
    let re = load(path.clone());
    println!("{:?}", re);
}

fn load(path: String) -> Result<(), String> {
    println!("{}", path);
    let lines = read_from_file(path);
    let format = Format::from(lines[0].to_string())?;
    let items = parse_items(format, lines)?;

    Ok(())
}

#[inline]
fn read_from_file(path: String) -> Vec<String> {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s);
    s.lines().map(|l| l.to_string())
        .map(|l| {println!("{}", l); l})
        .collect::<Vec<String>>()
}

fn parse_items(format: Format, lines: Vec<String>) -> Result<Vec<(Item,u32)>, String> { //TODO
    let results: Vec<Result<(Item, u32), Box<Error>>> = lines.into_iter().skip(1).map(|s| format.parse(s)).collect();
    let mut errs: Vec<String> = vec!();
    let mut items: Vec<(Item, u32)> = vec!();

    for re in results {
        match re {
            Ok(p) => items.push(p),
            Err(e) => errs.push(e.to_string()),
        }
    }

    if errs.len() != 0 {
        Err(errs.join("\n"))
    } else {
        Ok(items)
    }
}
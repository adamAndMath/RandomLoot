mod item;
mod format;

use item::Item;
use format::Format;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::env;

type Result<T> = std::result::Result<T, String>;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut typeConsts: HashMap<String, String> = HashMap::new();
    let item = Item::new();
    //println!("{}", item);

    let path = args.last().expect("Requires path");
    println!("{}", path);
    match load(path.clone()) {
        Ok(expr) => expr,
        Err(e) => println!("{}", e),
    }
}

fn load(path: String) -> Result<()> {
    println!("{}", path);
    let lines = read_from_file(path)?;
    let format = Format::from(lines[0].to_string())?;
    let items = parse_items(format, lines)?;

    Ok(())
}

#[inline]
fn read_from_file(path: String) -> Result<Vec<String>> {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s).map_err(|e| e.to_string())?;
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

fn parse_items(format: Format, lines: Vec<String>) -> Result<Vec<(Item,u32)>> {
    let indexed_lines = (0..lines.len()).zip(lines);
    let results: Vec<Result<(Item, u32)>> = indexed_lines.skip(1).map(|(i, s)| format.parse(s).map_err(|e| format!("ln {}: {}", i, e))).collect();
    let mut errs: Vec<String> = vec!();
    let mut items: Vec<(Item, u32)> = vec!();

    for re in results {
        match re {
            Ok(p) => items.push(p),
            Err(e) => errs.push(e.to_string()),
        }
    }

    if errs.len() != 0 {
        Err(format!("Failed to parse lines:\n{}", errs.join("\n")))
    } else {
        Ok(items)
    }
}
extern crate rand;

mod item;
mod format;
mod generator;

use item::Item;
use format::Format;
use generator::Generator;
use std::io::prelude::*;
use std::fs::File;
use std::env;

type Result<T> = std::result::Result<T, String>;

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args.last().expect("Requires path");
    let generator = load(path.clone()).unwrap();
    let item = generator.iter().next().unwrap();

    println!("{}", item);
}

fn load(path: String) -> Result<Generator> {
    let lines = read_from_file(path)?;
    let format: Format = parse_format(&lines[0])?;
    let items = parse_items(format, lines)?;
    let generator: Generator = items.into_iter().collect();

    Ok(generator)
}

#[inline]
fn read_from_file(path: String) -> Result<Vec<String>> {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s).map_err(|e| e.to_string())?;
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

fn parse_format(s: &String) -> Result<Format> {
    s.parse().map_err(|e| format!("Failed to parse format: {}", e))
}

fn parse_items(format: Format, lines: Vec<String>) -> Result<Vec<(u32, Item)>> {
    let results: Vec<Result<(u32, Item)>> = lines.into_iter().enumerate().skip(1).map(|(i, s)| format.parse(s).map_err(|e| format!("ln {}: {}", i, e))).collect();
    let mut errs: Vec<String> = vec!();
    let mut items: Vec<(u32, Item)> = vec!();

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
extern crate rand;

mod item;
mod format;
mod generator;
pub mod quantifier;

use item::Item;
use format::Format;
use generator::Generator;
use quantifier::Quantifier;
use std::io::prelude::*;
use std::fs::File;
use std::env;

type Result<T> = std::result::Result<T, String>;

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1).expect("Requires path");
    let amount = args.get(2).map_or(1, |p| p.parse().unwrap());
    let (format, items) = load(path.clone()).unwrap();
    let generator: Generator<usize> = (0..items.len()).map(|i| (items[i].0, i)).collect();
    let quantifier: Quantifier<usize> = generator.iter().take(amount).map(|i| *i).collect();
    let stacks: Vec<_> = quantifier.into_iter().map(|(i, q)| (&items[i].1, q)).collect();

    for (item, q) in stacks {
        println!("{}x{}", q, format.to_string(item));
    }
}

fn load(path: String) -> Result<(Format, Vec<(u32, Item)>)> {
    let mut lines = read_from_file(path)?.into_iter();
    let format_line = lines.next().ok_or("Empty file".to_string())?;
    let format: Format = parse_format(format_line)?;
    let items = parse_items(&format, lines)?;

    Ok((format, items))
}

#[inline]
fn read_from_file(path: String) -> Result<Vec<String>> {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s).map_err(|e| e.to_string())?;
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

fn parse_format(s: String) -> Result<Format> {
    s.parse().map_err(|e| format!("Failed to parse format: {}", e))
}

fn parse_items<I: IntoIterator<Item = String>>(format: &Format, lines: I) -> Result<Vec<(u32, Item)>> {
    let results: Vec<Result<(u32, Item)>> = lines.into_iter().enumerate().skip(1).map(|(i, s)| format.parse(s).map_err(|e| format!("ln {}: {}", i, e))).collect();
    let mut errs: Vec<String> = vec!();
    let mut items: Vec<(u32, Item)> = Vec::with_capacity(results.len());

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

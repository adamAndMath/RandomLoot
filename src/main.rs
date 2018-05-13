mod item;

use item::Item;
use std::io;
use std::io::prelude::*;
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

fn load(path: String) -> io::Result<()> {
    println!("{}", path);
    let lines = read_from_file(path);
    let items = parse_items(lines);

    Ok(())
}

#[inline]
fn read_from_file(path: String) -> Vec<String> {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s);
    s.lines().map(|l| l.to_string())
        .map(|l| {
            println!("{}", l);
            l
        })
        .collect::<Vec<String>>()
}

fn parse_items(mut lines: Vec<String>) -> Result<Vec<Item>, String> {
    let i: Vec<Vec<String>> = lines.into_iter()
        .map(|l| {
            l.split(':')
                .map(|o| o.trim().to_string())
                .collect()
        }).collect();
    let mut i = i.into_iter();
    if let Some(options) = i.next() {
        extract_types(options);
    };
    return Ok(Vec::new());

    fn extract_types(o: Vec<String>) -> Vec<Result<Item, String>> {
        let v = Vec::new();
        o.into_iter()
            .map(|type_defs| {
                let part_types_count = type_defs.split('|')
                    .collect::<Vec<&str>>()
                    .len();
                if type_defs.is_empty() {
                    Err("First line of file is empty")
                } else if type_defs.chars().into_iter().next().expect("") != '[' {
                    Err("Expected '[', found '" + type_defs.tail() + "'");
                } else if type_defs.chars().into_iter().last().expect("") != ']' {
                    Err("Expected ']', found '" + match o.next() {
                        Some(s) => s,
                        None => ""
                    }) + "'";
                } else if (2 <=part_types_count) <= 3 {
                    Err("Expects type declerations with syntax: [name|datatype<|postfix>], found: [" +
                        part_types_count.iter().fold(String::new(), |old, new| old + "|" + new) + ']'
                    )
                }
                // Is safe since type_defs at least contains "[]"
                let part_types = unsafe{ type_defs.slice_unchecked(1, type_defs.len() - 1) }
                    .split("|")
                    .map(|s: &str| s.trim());
            }).collect()
    }
}
use std::path::Path;
use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;
use std::io;

use item::Item;
use format::Format;
use format::FormatErr;
use generator::Generator;
use quantifier::Quantifier;

#[derive(Debug)]
pub enum GroupErr {
    EmptyFile,
    File(io::Error),
    FormatParse(FormatErr),
    ItemParse(Vec<(usize, FormatErr)>),
}

impl From<FormatErr> for GroupErr {
    fn from(e: FormatErr) -> Self {
        GroupErr::FormatParse(e)
    }
}

impl From<Vec<(usize, FormatErr)>> for GroupErr {
    fn from(e: Vec<(usize, FormatErr)>) -> Self {
        GroupErr::ItemParse(e)
    }
}

impl From<io::Error> for GroupErr {
    fn from(e: io::Error) -> Self {
        GroupErr::File(e)
    }
}

#[derive(Debug)]
pub struct Group {
    format: Format,
    items: Vec<(usize, Item)>,
    generator: Generator<usize>,
}

impl Group {
    pub fn new(format: Format, items: Vec<(usize, Item)>) -> Self {
        let generator = (0..items.len()).map(|i| (items[i].0, i)).collect();
        Group { format: format, items: items, generator: generator }
    }

    pub fn build<'a, I: Iterator<Item = (usize, &'a str)>>(header: &str, lines: I) -> Result<Group, GroupErr> {
        let format: Format = header.parse()?;
        let items: Vec<_> = collect_result(lines.map(|(i, s)| format.parse(s).map_err(|e| (i, e))))?;

        Ok(Self::new(format, items))
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Group, GroupErr> {
        let mut s = String::new();
        File::open(path)?.read_to_string(&mut s)?;
        s.parse()
    }

    pub fn generate(&self, amount: usize) -> Vec<(&Item, usize)> {
        let quantifier: Quantifier<usize> = self.generator.iter().take(amount).map(|i| *i).collect();
        quantifier.into_iter().map(|(i, q)| (&self.items[i].1, q)).collect()
    }

    pub fn print(&self, amount: usize) {
        for (item, q) in self.generate(amount) {
            println!("{}x{}", q, self.format.to_string(item));
        }
    }
}

impl FromStr for Group {
    type Err = GroupErr;

    fn from_str(s: &str) -> Result<Self, GroupErr> {
        let mut lines = s.lines().enumerate();
        let header = lines.next().ok_or(GroupErr::EmptyFile)?;
        Group::build(header.1, lines)
    }
}

fn collect_result<R, E, I: Iterator<Item = Result<R,E>>>(iter: I) -> Result<Vec<R>, Vec<E>> {
    let mut ok: Vec<R> = vec!();
    let mut err: Vec<E> = vec!();

    for e in iter {
        match e {
            Ok(r) => ok.push(r),
            Err(e) => err.push(e),
        }
    }

    if err.is_empty() {
        Ok(ok)
    } else {
        Err(err)
    }
}
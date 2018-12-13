pub mod parse;
pub mod var;
pub mod ty;
pub mod unit;

pub mod prelude {
    pub use super::parse::Parser;
    pub type Result<T> = std::result::Result<T, FormatErr>;
    pub type FormatErr = Box<std::error::Error>;
}

pub use self::{
    prelude::*,
    var::Var,
    ty::Type,
    unit::Unit,
};

use item::Item;
use std::str::FromStr;

#[derive(Debug)]
pub struct Format(Vec<Var>);

impl FromStr for Format {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Format> {
        s.split(";").map(Var::from_str).collect::<Result<_>>().map(Format)
    }
}

impl Format {
    pub fn parse(&self, s: &str) -> Result<(usize, Item)> {
        let args: Vec<&str> = s.split(';').map(|s| s.trim()).collect();
        let mut item = Item::new();

        if args.len() != self.0.len() + 1 {
            unimplemented!();
        }

        for (var, arg) in self.0.iter().zip(&args) {
            item.insert(var.name.to_owned(), var.ty.parse(arg)?)
        }

        let rand = args[args.len()-1].parse::<usize>()?;

        Ok((rand, item))
    }

    pub fn to_string(&self, item: &Item) -> String {
        format!("{}, {} lb, {} cp", item.get("name").unwrap(), item.get("weight").unwrap(), item.get("price").unwrap())
    }
}
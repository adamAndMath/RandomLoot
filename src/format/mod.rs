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
use std::fmt::{ self, Display, Formatter };

#[derive(Debug)]
pub struct Format(Vec<Var>);

impl FromStr for Format {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Format> {
        s.split(";").map(Var::from_str).collect::<Result<_>>().map(Format)
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if !self.0.is_empty() {
            write!(f, "{}", self.0[0])?;

            for v in &self.0[1..] {
                write!(f, "; {}", v)?;
            }
        }

        Ok(())
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
        self.0.iter().map(|v|v.ty.to_string(item.get(&v.name).unwrap())).collect::<Vec<String>>().join(", ")
        //format!("{}, {} lb, {} cp", item.get("name").unwrap(), item.get("weight").unwrap(), item.get("price").unwrap())
    }
}
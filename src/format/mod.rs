#[macro_use]
pub mod letn_macro;

pub mod parse;
pub mod ty;
pub mod unit;
pub mod var;

pub use self::{parse::Parser, ty::*, unit::*, var::*};

use item::Item;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Format(Vec<Var>);

impl FromStr for Format {
    type Err = VarError;

    fn from_str(s: &str) -> Result<Format, VarError> {
        s.split(";")
            .map(Var::from_str)
            .collect::<Result<_, _>>()
            .map(Format)
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
    pub fn parse(&self, s: &str) -> Result<(usize, Item), ParseErr> {
        let args: Vec<&str> = s.split(';').map(|s| s.trim()).collect();
        let mut item = Item::new();

        if args.len() != self.0.len() + 1 {
            return Err(ParseErr::ParameterCount(args.len()));
        }

        for (var, arg) in self.0.iter().zip(&args) {
            item.insert(var.name().to_owned(), var.ty().parse(arg)?)
        }

        let rand = args[args.len() - 1]
            .parse::<usize>()
            .map_err(ParseErr::Rand)?;

        Ok((rand, item))
    }

    pub fn iter(&self) -> impl Iterator<Item=&Var> {
        self.0.iter()
    }

    pub fn to_string(&self, item: &Item) -> String {
        self.iter()
            .map(|v| v.ty().to_string(item.get(&v.name()).unwrap()))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

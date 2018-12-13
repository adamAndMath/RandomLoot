use std::str::FromStr;
use super::{
    prelude::*,
    Type,
};

#[derive(Debug)]
pub struct Var {
    pub name: String,
    pub ty: Type,
}

impl Var {
    fn new(name: String, ty: Type) -> Self {
        Var { name, ty }
    }
}

impl FromStr for Var {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Var> {
        let mut iter = s.splitn(2, ":").map(|s|s.trim());
        let name = iter.next().unwrap().to_owned();
        let ty = iter.next().unwrap().parse()?;
        Ok(Var::new(name, ty))
    }
}

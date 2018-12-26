use std::str::FromStr;
use std::fmt::{ self, Display, Formatter };
use super::{
    Type,
    TypeError,
};

#[derive(Debug)]
pub struct Var {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug)]
pub enum VarError {
    MissingName,
    MissingType,
    Type(TypeError),
}

impl Var {
    fn new(name: String, ty: Type) -> Self {
        Var { name, ty }
    }
}

impl FromStr for Var {
    type Err = VarError;

    fn from_str(s: &str) -> Result<Var, VarError> {
        let mut iter = s.splitn(2, ":").map(|s|s.trim());
        let name = iter.next().ok_or(VarError::MissingName)?.to_owned();
        let ty = iter.next().ok_or(VarError::MissingType)?.parse().map_err(VarError::Type)?;
        Ok(Var::new(name, ty))
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

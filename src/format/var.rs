use std::str::FromStr;
use std::fmt::{ self, Display, Formatter };
use super::{
    Type,
    TypeError,
};

#[derive(Debug)]
pub struct Var {
    name: String,
    ty: Type,
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn ty(&self) -> &Type {
        &self.ty
    }
}

impl FromStr for Var {
    type Err = VarError;

    fn from_str(s: &str) -> Result<Var, VarError> {
        letn!(name?(VarError::MissingName), ty?(VarError::MissingType) = s.splitn(2, ":").map(|s|s.trim()));
        Ok(Var::new(name.to_owned(), ty.parse().map_err(VarError::Type)?))
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

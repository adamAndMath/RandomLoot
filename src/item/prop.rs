use std::fmt::{ self, Display, Formatter };

#[derive(Debug)]
pub enum Prop {
    Bool(bool),
    Int(isize),
    Float(f32),
    Str(String),
}

impl Display for Prop {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Prop::Bool(v) => v.fmt(f),
            Prop::Int(v) => v.fmt(f),
            Prop::Float(v) => v.fmt(f),
            Prop::Str(v) => v.fmt(f),
        }
    }
}
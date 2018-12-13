use std::str::FromStr;
use std::fmt::{ self, Display, Formatter };
use item::Prop;
use super::{
    Parser,
    Unit,
    UnitError,
};

#[derive(Debug)]
pub enum Type {
    Bool(String, String),
    Int(Unit<isize>),
    Float(Unit<f32>),
    Str,
}

#[derive(Debug)]
pub enum TypeError {
    BoolLabelAmount(usize),
    ExpectedBrackets,
    UndefinedType(String),
    IntUnit(UnitError<isize>),
    FloatUnit(UnitError<f32>),
}

#[derive(Debug)]
pub enum ParseErr {
    UndefinedLabel(String, Vec<String>),
    Int(<isize as FromStr>::Err),
    Float(<f32 as FromStr>::Err),
    Rand(<usize as FromStr>::Err),
    ParameterCount(usize),
}

impl FromStr for Type {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Type, TypeError> {
        if s.starts_with("bool") {
            let s = s[4..].trim();
            if s.is_empty() {
                Ok(Type::Bool("true".to_owned(), "false".to_owned()))
            } else if s.starts_with("[") && s.ends_with("]") {
                let labels = s[1..s.len()-1].split(",").map(|s|s.trim()).collect::<Vec<_>>();
                if labels.len() != 2 {
                    return Err(TypeError::BoolLabelAmount(labels.len()));
                }
                let yes = labels[0].to_owned();
                let no = labels[1].to_owned();
                Ok(Type::Bool(yes, no))
            } else {
                Err(TypeError::ExpectedBrackets)
            }
        } else if s.starts_with("int") {
            s[3..].trim().parse().map(Type::Int).map_err(TypeError::IntUnit)
        } else if s.starts_with("float") {
            s[5..].trim().parse().map(Type::Float).map_err(TypeError::FloatUnit)
        } else if s.starts_with("str") {
            Ok(Type::Str)
        } else {
            Err(TypeError::UndefinedType(s.to_owned()))
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Type::Bool(yes, no) if yes == "true" && no == "false" => write!(f, "bool"),
            Type::Bool(yes, no) => write!(f, "bool[{}, {}]", yes, no),
            Type::Int(unit) => write!(f, "int{}", unit),
            Type::Float(unit) => write!(f, "float{}", unit),
            Type::Str => write!(f, "str"),
        }
    }
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParseErr::UndefinedLabel(s, labels) => write!(f, "Found \"{}\", but expected {}", s, labels.join(", ")),
            ParseErr::Int(e) => e.fmt(f),
            ParseErr::Float(e) => e.fmt(f),
            ParseErr::Rand(e) => e.fmt(f),
            ParseErr::ParameterCount(n) => write!(f, "Recieved {} parameters", n),
        }
    }
}

impl Parser for Type {
    type Output = Box<Prop>;
    type Err = ParseErr;
    fn parse(&self, s: &str) -> Result<Box<Prop>, ParseErr> {
        match self {
            Type::Bool(yes, no) =>
                if s == yes {
                    Ok(Box::new(true))
                } else if s == no {
                    Ok(Box::new(false))
                } else {
                    Err(ParseErr::UndefinedLabel(s.to_owned(), vec![yes.to_owned(), no.to_owned()]))
                },
            Type::Int(unit) => unit.parse(s).map(|v|Box::new(v) as Box<Prop>).map_err(ParseErr::Int),
            Type::Float(unit) => unit.parse(s).map(|v|Box::new(v) as Box<Prop>).map_err(ParseErr::Float),
            Type::Str => Ok(Box::new(s.to_owned()))
        }
    }
}

impl Type {
    pub fn to_string(&self, p: &Box<Prop>) -> String {
        match self {
            Type::Bool(yes, no) =>
                match p.as_any().downcast_ref::<bool>() {
                    Some(true) => yes.to_owned(),
                    Some(false) => no.to_owned(),
                    None => unreachable!(),
                },
            Type::Int(unit) => unit.to_string(p.as_any().downcast_ref().unwrap()),
            Type::Float(unit) => unit.to_string(p.as_any().downcast_ref().unwrap()),
            Type::Str => p.as_any().downcast_ref::<String>().unwrap().to_owned(),
        }
    }
}

use item::Item;
use item::Prop;
use std::error::Error;
use std::ops::Mul;
use std::str::FromStr;
use std::result;

type Result<T> = result::Result<T, FormatErr>;

pub type FormatErr = Box<Error>;

#[derive(Debug)]
pub struct Format(Vec<Var>);

#[derive(Debug)]
struct Var {
    name: String,
    tp: Type,
}

impl Var {
    fn new(name: String, tp: Type) -> Self {
        Var { name, tp }
    }
}

#[derive(Debug)]
enum Type {
    Bool(String, String),
    Int(Unit<isize>),
    Float(Unit<f32>),
    Str,
}

impl Type {
    fn parse(&self, s: &str) -> Result<Box<Prop>> {
        match self {
            Type::Bool(yes, no) =>
                if s == yes {
                    Ok(Box::new(true))
                } else if s == no {
                    Ok(Box::new(false))
                } else {
                    unimplemented!()
                    //Err(Box::new(format!("\"{}\" is not a bool", s)))
                },
            Type::Int(unit) => unit.parse(s),
            Type::Float(unit) => unit.parse(s),
            Type::Str => Ok(Box::new(s.to_owned()))
        }
    }
}

#[derive(Debug)]
struct Unit<T: Num>(Vec<(String, T)>) where T::Err: Error;

trait Num: 'static + Prop + FromStr + Mul<Output=Self> + Copy where Self::Err: Error {
    fn one() -> Self;
}

impl Num for isize {
    fn one() -> Self { 1 }
}

impl Num for f32 {
    fn one() -> Self { 1f32 }
}

impl<T: Num> Unit<T> where T::Err: Error {
    fn parse(&self, s: &str) -> Result<Box<Prop>> {
        let unit = self.0.iter().filter(|(u,_)|s.ends_with(u)).next();

        let v = if let Some((u,m)) = unit {
            s[..s.len()-u.len()].trim().parse::<T>()?*(*m)
        } else {
            s.parse::<T>()?
        };

        Ok(Box::new(v))
    }
}

impl FromStr for Format {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Format> {
        s.split(";").map(Var::from_str).collect::<Result<_>>().map(Format)
    }
}

impl FromStr for Var {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Var> {
        let mut iter = s.splitn(2, ":").map(|s|s.trim());
        let name = iter.next().unwrap().to_owned();
        let tp = iter.next().unwrap().parse()?;
        Ok(Var::new(name, tp))
    }
}

impl FromStr for Type {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Type> {
        if s.starts_with("bool") {
            let s = s[4..].trim();
            if s.is_empty() {
                Ok(Type::Bool("true".to_owned(), "false".to_owned()))
            } else if s.starts_with("[") && s.ends_with("]") {
                let labels = s[1..s.len()-1].split(",").map(|s|s.trim()).collect::<Vec<_>>();
                if labels.len() != 2 {
                    unimplemented!();
                }
                let yes = labels[0].to_owned();
                let no = labels[1].to_owned();
                Ok(Type::Bool(yes, no))
            } else {
                unimplemented!()
            }
        } else if s.starts_with("int") {
            s[3..].trim().parse().map(Type::Int)
        } else if s.starts_with("float") {
            s[5..].trim().parse().map(Type::Float)
        } else if s.starts_with("str") {
            Ok(Type::Str)
        } else {
            unimplemented!()
        }
    }
}

impl<T: Num> FromStr for Unit<T> where T::Err: Error {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Unit<T>> {
        if s.is_empty() {
            Ok(Unit(vec![]))
        } else if s.starts_with("[") && s.ends_with("]") {
            let s = &s[1..s.len()-1];
            if s.contains(",") {
                s.split(",").map(|s| {
                    let v = s.splitn(2, "=").collect::<Vec<_>>();
                    if v.len() != 2 {
                        unimplemented!("{:?}", v)
                    }
                    let name = v[0].to_owned();
                    let size = v[1].parse::<T>()?;
                    Ok((name, size))
                }).collect::<Result<_>>().map(Unit)
            } else {
                let s = s.trim();
                if s.is_empty() {
                    Ok(Unit(vec![]))
                } else {
                    Ok(Unit(vec![(s.to_owned(), T::one())]))
                }
            }
        } else {
            unimplemented!()
        }
    }
}

impl Format {
    pub fn parse(&self, s: &str) -> Result<(usize, Item)> {
        let args: Vec<&str> = s.split(';').map(|s| s.trim()).collect();
        let mut item = Item::new();

        for (var, arg) in self.0.iter().zip(&args) {
            item.insert(var.name.to_owned(), var.tp.parse(arg)?)
        }

        let rand = args[3].parse::<usize>()?;

        Ok((rand, item))
    }

    pub fn to_string(&self, item: &Item) -> String {
        format!("{}, {} lb, {} cp", item.get("name").unwrap(), item.get("weight").unwrap(), item.get("price").unwrap())
    }
}
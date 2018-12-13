use std::str::FromStr;
use std::error::Error;
use item::Prop;
use std::ops::Mul;
use super::prelude::*;

pub trait Num: 'static + Prop + FromStr + Mul<Output=Self> + Copy where Self::Err: Error {
    fn one() -> Self;
}

impl Num for isize {
    fn one() -> Self { 1 }
}

impl Num for f32 {
    fn one() -> Self { 1f32 }
}

#[derive(Debug)]
pub struct Unit<T: Num>(Vec<(String, T)>) where T::Err: Error;

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

impl<T: Num> Parser for Unit<T> where T::Err: Error {
    type Output = Box<Prop>;
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

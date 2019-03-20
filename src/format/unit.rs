use super::Parser;
use std::fmt::{self, Display, Formatter};
use std::ops::Mul;
use std::str::FromStr;

pub trait Num: Display + FromStr + PartialOrd + PartialEq + Mul<Output = Self> + Copy
{
    fn one() -> Self;
    fn match_unit(&self, unit: &Self) -> Option<Self>;
    fn force_match(&self, unit: &Self) -> Option<Self>;
}

impl Num for isize {
    fn one() -> Self {
        1
    }
    fn match_unit(&self, unit: &Self) -> Option<Self> {
        if self % unit == 0 {
            Some(self / unit)
        } else {
            None
        }
    }
    fn force_match(&self, unit: &Self) -> Option<Self> {
        self.match_unit(unit)
    }
}

impl Num for f32 {
    fn one() -> Self {
        1f32
    }
    fn match_unit(&self, unit: &Self) -> Option<Self> {
        if self.abs() >= *unit {
            Some(self / unit)
        } else {
            None
        }
    }
    fn force_match(&self, unit: &Self) -> Option<Self> {
        Some(self / unit)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Unit<T: Num>(pub Vec<(String, T)>);

#[derive(Debug, PartialEq, Eq)]
pub enum UnitError<T: Num> {
    NotBrackets,
    MissingName,
    MissingValue,
    ParseError(T::Err),
}

impl<T: Num> Default for Unit<T> {
    fn default() -> Self {
        Unit(vec![])
    }
}

impl<T: Num> FromStr for Unit<T> {
    type Err = UnitError<T>;

    fn from_str(s: &str) -> Result<Unit<T>, UnitError<T>> {
        if s.contains(",") {
            s.split(",")
                .map(|s| {
                    letn!(name?(UnitError::MissingName), size?(UnitError::MissingName) = s.splitn(2, "=").map(|s|s.trim()));
                    Ok((name.to_owned(), size.parse::<T>().map_err(UnitError::ParseError)?))
                })
                .collect::<Result<Vec<_>, UnitError<T>>>()
                .map(|mut v| {
                    v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                    Unit(v)
                })
        } else {
            let s = s.trim();
            if s.is_empty() {
                Ok(Unit(vec![]))
            } else {
                Ok(Unit(vec![(s.to_owned(), T::one())]))
            }
        }
    }
}

impl<T: Num> Display for Unit<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0.is_empty() {
            Ok(())
        } else if self.0.len() == 1 && self.0[0].1 == T::one() {
            write!(f, "[{}]", self.0[0].0)
        } else {
            write!(f, "[{}={}", self.0[0].0, self.0[0].1)?;

            for (n, s) in &self.0[1..] {
                write!(f, ", {}={}", n, s)?;
            }

            write!(f, "]")
        }
    }
}

impl<T: Num> Parser for Unit<T> {
    type Output = T;
    type Err = T::Err;
    fn parse(&self, s: &str) -> Result<T, T::Err> {
        let unit = self.0.iter().filter(|(u, _)| s.ends_with(u)).next();

        Ok(if let Some((u, m)) = unit {
            s[..s.len() - u.len()].trim().parse::<T>()? * (*m)
        } else {
            s.parse::<T>()?
        })
    }
}

impl<T: Num> Unit<T> {
    pub fn to_string(&self, v: &T) -> String {
        self.0
            .iter()
            .rev()
            .filter_map(|(n, u)| v.match_unit(u).map(|v| format!("{} {}", v, n)))
            .next()
            .or_else(|| {
                self.0
                    .last()
                    .and_then(|(n, u)| v.force_match(u).map(|v| format!("{} {}", v, n)))
            })
            .unwrap_or_else(|| v.to_string())
    }
}

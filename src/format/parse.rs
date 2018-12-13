use super::Result;

pub trait Parser {
    type Output;
    fn parse(&self, s: &str) -> Result<Self::Output>;
}
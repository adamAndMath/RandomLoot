pub trait Parser {
    type Output;
    type Err;
    fn parse(&self, s: &str) -> Result<Self::Output, Self::Err>;
}
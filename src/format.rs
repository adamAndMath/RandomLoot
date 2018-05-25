use super::item::Item;
use std::error::Error;
use std::str::FromStr;
use std::result;

type Result<T> = result::Result<T, FormatErr>;

#[derive(Debug)]
pub struct Format();

pub type FormatErr = Box<Error>;

impl FromStr for Format {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Format> {
        let vars = s[1..s.len()-2].split("]:[");
        for v in vars {
            println!("{}", v);
        }

        Ok(Format())
    }
}

impl Format {
    pub fn parse(&self, s: String) -> Result<(usize, Item)> {
        let args: Vec<&str> = s.split(':').map(|s| s.trim()).collect();
        let mut item = Item::new();

        let name = args[0].to_owned();
        let weight = args[1].parse::<f32>()?;
        let price = args[2].parse::<i32>()?;
        let rand = args[3].parse::<usize>()?;

        item.insert("name".to_owned(), Box::new(name));
        item.insert("weight".to_owned(), Box::new(weight));
        item.insert("price".to_owned(), Box::new(price));

        Ok((rand, item))
    }

    pub fn to_string(&self, item: &Item) -> String {
        format!("{}, {} lb, {} cp", item.get("name").unwrap(), item.get("weight").unwrap(), item.get("price").unwrap())
    }
}
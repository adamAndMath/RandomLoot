use super::item::Item;
use std::error::Error;

pub struct Format();

impl Format {
    pub fn from(s: String) -> Result<Format, &'static str> {
        let vars = s[1..s.len()-2].split("]:[");
        for v in vars {
            println!("{}", v);
        }

        Ok(Format())
    }

    pub fn parse(&self, s: String) -> Result<(Item, u32), Box<Error>> {
        let args: Vec<&str> = s.split(':').map(|s| s.trim()).collect();
        let mut item = Item::new();

        let name = args[0].to_owned();
        let weight = args[1].parse::<f32>()?;
        let price = args[2].parse::<i32>()?;
        let rand = args[3].parse::<u32>()?;

        item.insert("name".to_owned(), name);
        item.insert("weight".to_owned(), weight);
        item.insert("price".to_owned(), price);

        Ok((item, rand))
    }
}
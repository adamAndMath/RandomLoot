use super::item::Item;
use std::error::Error;
use std::str::FromStr;
use std::result;

type Result<T> = result::Result<T, FormatErr>;

pub struct Format(Vec<PropertyType>);

#[derive(Debug)]
pub enum FormatErr {
    Error(Box<Error>),
    StringError(String)
}

impl FromStr for Format {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<Format> {
        let vars: Vec<&str> = s[1..s.len() - 2].split("]:[").collect();
        let mut head: Vec<PropertyType> = Vec::with_capacity(vars.len());
        for v in vars.into_iter() {
            if let Ok(t) = PropertyType::from_str(v) {
                head.push(t);
            }
        }
        Ok(Format(head))
    }
}

enum PropertyType { //TODO Should probably be implemented with &str and lifetimes.
    Int{name: String},
    UInt{name: String},
    Float{name: String},
    UFloat{name: String},
    Bool{name: String},
    String{name: String},
    File{name: String, path: String},
    Format{name: String, format: String}, //TODO format need to be parsed.
}

impl FromStr for PropertyType {
    type Err = FormatErr;

    fn from_str(s: &str) -> Result<PropertyType> {
        let component: Vec<&str> = s.split('|').collect();
        if let (Some(type_string), Some(name_string), Some(special_string)) = (component.get(0), component.get(1), component.get(2)) {
            match *type_string {
                "file" => Ok(PropertyType::File{name: name_string.to_string(), path: special_string.to_string()}),
                "format" => Ok(PropertyType::Format{name: name_string.to_string(), format: special_string.to_string()}),
                _ => Err(FormatErr::StringError(String::from("Wrong number of arguments, or nonexistent type")))
            }
        } else if let (Some(type_string), Some(name_string)) = (component.get(0), component.get(1)) {
            match *type_string {
                "int" => Ok(PropertyType::Int{name: name_string.to_string()}),
                "uint" => Ok(PropertyType::Int{name: name_string.to_string()}),
                "float" => Ok(PropertyType::Int{name: name_string.to_string()}),
                "ufloat" => Ok(PropertyType::Int{name: name_string.to_string()}),
                "bool" => Ok(PropertyType::Int{name: name_string.to_string()}),
                "string" => Ok(PropertyType::Int{name: name_string.to_string()}),
                _ => Err(FormatErr::StringError(String::from("Wrong number of arguments, or nonexistent type")))
            }
        }
        else {Err(FormatErr::StringError(String::from("Expected format information for file")))}
    }
}

impl Format {
    pub fn parse(&self, s: String) -> Result<(u32, Item)> {
        unimplemented!()
        // let args: Vec<&str> = s.split(':').map(|s| s.trim()).collect();
        // let mut item = Item::new();
        //
        // let name = args[0].to_owned();
        // let weight = args[1].parse::<f32>();
        // let price = args[2].parse::<i32>();
        // let rand = args[3].parse::<u32>();
        //
        // item.insert("name".to_owned(), Box::new(name));
        // item.insert("weight".to_owned(), Box::new(weight));
        // item.insert("price".to_owned(), Box::new(price));
        //
        // Ok((rand, item))
    }

    pub fn to_string(&self, item: &Item) -> String {
        unimplemented!()
        // format!("{}, {} lb, {} cp", item.get("name").unwrap(), item.get("weight").unwrap(), item.get("price").unwrap())
    }
}

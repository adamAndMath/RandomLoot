mod prop;

use std::collections::HashMap;
use std::fmt;

use self::prop::Prop;

#[derive(Debug)]
pub struct Item(HashMap<String, Box<Prop>>);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        for (k,p) in &self.0 {
            writeln!(f, "{}: {}", k, p)?;
        }
        write!(f, "}}")
    }
}

impl Item {
    pub fn new() -> Item {
        Item(HashMap::new())
    }

    pub fn insert(&mut self, k: String, v: Box<Prop>) {
        self.0.insert(k, v);
    }

    pub fn get(&self, k: &str) -> Option<&Box<Prop>> {
        self.0.get(k)
    }
}
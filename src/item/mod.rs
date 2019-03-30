mod prop;

use std::collections::HashMap;
use std::ops::Index;
use std::fmt;

pub use self::prop::Prop;

#[derive(Debug)]
pub struct Item(HashMap<String, Prop>);

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

    pub fn insert(&mut self, k: String, v: Prop) {
        self.0.insert(k, v);
    }

    pub fn get(&self, k: &str) -> Option<&Prop> {
        self.0.get(k)
    }
}

impl<'a> Index<&'a str> for Item {
    type Output = Prop;
    fn index(&self, k: &str) -> &Prop {
        self.get(k).expect("Invalid property")
    }
}
mod prop;
mod insertable;

use std::collections::HashMap;
use std::fmt;

use self::insertable::Insertable;
use self::prop::Prop;

pub struct Item(HashMap<String, Prop>);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (k,p) in &self.0 {
            try!(write!(f, "{}: {}", k, p));
        }

        Ok(())
    }
}

impl Item {
    pub fn new() -> Item {
        Item(HashMap::new())
    }

    pub fn insert<P: Insertable>(&mut self, k: String, v: P) {
        self.0.insert(k, v.wrap());
    }

    pub fn get<P: Insertable>(&mut self, k: &String) -> Option<&P> {
        self.0.get(k).and_then(P::unwrap)
    }
}
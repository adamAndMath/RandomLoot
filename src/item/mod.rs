mod prop;
mod insertable;

use std::collections::HashMap;
use std::fmt;

use self::insertable::Insertable;
use self::prop::Prop;

pub struct Item(HashMap<String, Prop>);
pub struct ItemStack(u64, Item);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (k,p) in &self.0 {
            try!(write!(f, "\n{}: {}", k, p));
        }

        Ok(())
    }
}

impl fmt::Display for ItemStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "amount: {}{}", self.0, self.1)
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
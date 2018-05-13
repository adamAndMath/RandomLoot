use std::collections::BTreeMap;
use item::Item;
use item::ItemStack;
use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Generator {
    tree: BTreeMap<u32, Item>,
    total: u32,
}

pub struct Iter<'a> {
    generator: &'a Generator
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<&'a Item> {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, self.generator.total);
        self.generator.tree.range(i..).next().map(|e| e.1)
    }
}

impl Generator {
    pub fn new(items: Vec<(Item, u32)>) -> Self {
        let mut tree = BTreeMap::new();
        let mut total = 0;

        for (item, rand) in items {
            total += rand;
            tree.insert(total, item);
        }

        Generator { tree: tree, total: total }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter { generator: self }
    }
}
use std::collections::BTreeMap;
use item::Item;
use item::ItemStack;
use rand::Rng;

#[derive(Debug)]
pub struct Generator {
    tree: BTreeMap<u32, Item>,
    total: u32,
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

    pub fn get<R: Rng>(&self, rng: &mut R) -> &Item {
        let i = rng.gen_range(0, self.total);
        self.tree.range(i..).next().unwrap().1
    }

    pub fn gen<R: Rng>(&self, amount: u32, rng: &mut R) -> Vec<ItemStack> {
        unimplemented!();
    }
}
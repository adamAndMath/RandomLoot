use std::collections::BTreeMap;
use std::iter::FromIterator;
use rand;
use rand::ThreadRng;
use rand::Rng;

///Reterns random elements from a collection based on the weight of the element.
#[derive(Debug)]
pub struct Generator<E> {
    tree: BTreeMap<usize, E>,
    total: usize,
}

pub struct Iter<'a, E: 'a> {
    generator: &'a Generator<E>,
    rng: ThreadRng
}

impl<E> FromIterator<(usize, E)> for Generator<E> {
    fn from_iter<I: IntoIterator<Item = (usize, E)>>(iter: I) -> Generator<E> {
        let mut tree = BTreeMap::new();
        let mut total = 0;

        for (rand, item) in iter {
            total += rand;
            tree.insert(total, item);
        }

        Generator { tree: tree, total: total }
    }
}

impl<'a, E> Iterator for Iter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<&'a E> {
        let i = self.rng.gen_range(0, self.generator.total);
        self.generator.get(i)
    }
}

impl<E> Generator<E> {
    ///Gets a sigle random element.
    pub fn next(&self) -> Option<&E> {
        let i = rand::thread_rng().gen_range(0, self.total);
        self.get(i)
    }

    ///Gets the ellement in that owns the range that the weighted index belongs to.
    #[inline]
    pub fn get(&self, i: usize) -> Option<&E> {
        self.tree.range(i..).next().map(|e| e.1)
    }

    ///Returns an infinate iterator.
    pub fn iter<'a>(&'a self) -> Iter<'a, E> {
        Iter { generator: self, rng: rand::thread_rng() }
    }
}
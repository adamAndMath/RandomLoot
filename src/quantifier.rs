use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::hash::Hash;

#[derive(Debug)]
pub struct Quantifier<E: Eq + Hash> {
    map: HashMap<E, u32>
}

impl<E: Eq + Hash> Quantifier<E> {
    fn new() -> Self {
        Quantifier { map: HashMap::new() }
    }

    fn add(&mut self, e: E) {
        let v = self.map.get(&e).map_or(1, |v| *v + 1);
        self.map.insert(e, v);
    }
}

impl<E: Eq + Hash> FromIterator<E> for Quantifier<E> {
    fn from_iter<I: IntoIterator<Item = E>>(iter: I) -> Self {
        let mut quantifier = Quantifier::new();

        for e in iter {
            quantifier.add(e);
        }

        quantifier
    }
}

impl<E: Eq + Hash> IntoIterator for Quantifier<E> {
    type Item = (E, u32);
    type IntoIter = IntoIter<E, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
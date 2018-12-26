use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::hash::Hash;

/// Quantifies data by equality.
#[derive(Debug, Default)]
pub struct Quantifier<E: Eq + Hash> {
    map: HashMap<E, usize>
}

impl<E: Eq + Hash> Quantifier<E> {
    ///Creates an empty quantifier.
    pub fn new() -> Self {
        Quantifier { map: HashMap::new() }
    }

    ///Increases the quantity of e by one.
    #[inline]
    pub fn add(&mut self, e: E) {
        let v = self.map.get(&e).map_or(1, |v| *v + 1);
        self.map.insert(e, v);
    }

    ///Reterns the quantity of e if e was seen.
    #[inline]
    pub fn get(&self, e: &E) -> Option<&usize> {
        self.map.get(e)
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
    type Item = (E, usize);
    type IntoIter = IntoIter<E, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
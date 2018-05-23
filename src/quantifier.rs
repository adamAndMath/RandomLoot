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

#[test]
fn quantifier_obj() {
    let mut q = Quantifier::<&str>::new();
    q.add("John");
    q.add("Rick");
    q.add("Carry");
    q.add("Carry");
    q.add("John");
    q.add("John");

    let q = q;

    assert_eq!(q.get(&"John"), Some(&3));
    assert_eq!(q.get(&"Carry"), Some(&2));
    assert_eq!(q.get(&"Rick"), Some(&1));
    assert_eq!(q.get(&"Not in"), None);
    assert_eq!(q.get(&"Rick"), Some(&1));
    assert_eq!(q.get(&"Carry"), Some(&2));
    assert_eq!(q.get(&"John"), Some(&3));
}

#[test]
fn quantifier_iter() {
    let data = vec![1, 2, 3, 4, 2, 3, 4, 3, 4, 4];
    let quantities = vec![(1, 1), (2, 2), (3, 3), (4, 4)];
    let q: Quantifier<_> = data.into_iter().collect();
    let t: HashMap<_, _> = q.into_iter().collect();
    let e: HashMap<_, _> = quantities.into_iter().collect();

    assert_eq!(t, e);
}
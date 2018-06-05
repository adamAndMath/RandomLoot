use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::FusedIterator;
use rand;
use rand::ThreadRng;
use rand::Rng;

/// Reterns random elements from a collection based on the weight of the element.
#[derive(Debug, Default)]
pub struct Generator<E> {
    tree: BTreeMap<usize, E>,
    total_weight: usize,
}

pub struct Iter<'a, E: 'a> {
    generator: &'a Generator<E>,
    rng: ThreadRng
}

impl<'a, E> Iterator for Iter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<&'a E> {
        if self.generator.is_empty() {
            return None
        }

        let i = self.rng.gen_range(0, self.generator.total_weight);
        self.generator.get(i)
    }
}

impl<'a, E> FusedIterator for Iter<'a, E> {}

impl<E> FromIterator<(usize, E)> for Generator<E> {
    fn from_iter<I: IntoIterator<Item = (usize, E)>>(iter: I) -> Generator<E> {
        let mut gen = Generator::new();
        gen.extend(iter);
        gen
    }
}

impl<E> Extend<(usize, E)> for Generator<E> {
    fn extend<I: IntoIterator<Item = (usize, E)>>(&mut self, iter: I) {
        for (rand, item) in iter {
            self.add(rand, item);
        }
    }
}

pub struct IntoIter<E> {
    state: usize,
    data: <BTreeMap<usize, E> as IntoIterator>::IntoIter,
}

impl<E> IntoIterator for Generator<E> {
    type Item = (usize, E);
    type IntoIter = IntoIter<E>;

    fn into_iter(self) -> IntoIter<E> {
        IntoIter { state: 0, data: self.tree.into_iter() }
    }
}

impl<E> Iterator for IntoIter<E> {
    type Item = (usize, E);
    
    fn next(&mut self) -> Option<(usize, E)> {
        let (t, e) = self.data.next()?;
        let r = t - self.state + 1;
        self.state = t + 1;
        Some((r, e))
    }
}

impl<E> Generator<E> {
    ///Creates an empty Generator
    #[inline]
    pub fn new() -> Self {
        Generator { tree: BTreeMap::new(), total_weight: 0 }
    }

    #[inline]
    pub fn add(&mut self, rand: usize, e: E) {
        if rand != 0 {
            self.total_weight += rand;
            self.tree.insert(self.total_weight - 1, e);
        }
    }

    /// Gets the element in that owns the range that the weighted index belongs to.
    /// Returns none if the index is out of bounds.
    #[inline]
    pub fn get(&self, i: usize) -> Option<&E> {
        self.tree.range(i..).next().map(|e| e.1)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.total_weight == 0
    }

    /// Returns a single random element or none if the generator is empty.
    pub fn next(&self) -> Option<&E> {
        if self.is_empty() {
            return None
        }

        let i = rand::thread_rng().gen_range(0, self.total_weight);
        self.get(i)
    }

    /// Returns an infinite iterator.
    /// The iterator is empty if the generator is empty.
    pub fn generate<'a>(&'a self) -> Iter<'a, E> {
        Iter { generator: self, rng: rand::thread_rng() }
    }
}

#[test]
fn manual_get() {
    let g: Generator<&str> = vec![(10, "1st"), (0, "Nope"), (53, "2nd"), (1, "3rd")].into_iter().collect();

    assert_eq!(g.get(0), Some(&"1st"));
    assert_eq!(g.get(6), Some(&"1st"));
    assert_eq!(g.get(9), Some(&"1st"));
    assert_eq!(g.get(10), Some(&"2nd"));
    assert_eq!(g.get(35), Some(&"2nd"));
    assert_eq!(g.get(62), Some(&"2nd"));
    assert_eq!(g.get(63), Some(&"3rd"));
    assert_eq!(g.get(64), None);
}

#[test]
fn empty_generator() {
    let g: Generator<&str> = vec![].into_iter().collect();
    assert_eq!(g.get(0), None);
    assert_eq!(g.next(), None);
}
use std::iter::FromIterator;

/// Quantifies integers.
#[derive(Debug, Default)]
pub struct Quantifier {
    map: Vec<usize>,
}


impl Quantifier {
    ///Creates an empty quantifier.
    pub fn new() -> Self {
        Quantifier { map: Vec::new() }
    }

    ///Creates a quantifier with reserved space.
    pub fn with_capacity(size: usize) -> Self {
        Quantifier { map: Vec::with_capacity(size) }
    }

    ///Increases the quantity of e by one.
    #[inline]
    pub fn add(&mut self, e: usize) {
        if self.map.len() <= e { self.map.resize(e+1, 0) }
        self.map[e] += 1;
    }

    ///Reterns the quantity of e if e was seen.
    #[inline]
    pub fn get(&self, e: usize) -> Option<&usize> {
        self.map.get(e).filter(|c|**c!=0)
    }
}

impl Extend<usize> for Quantifier {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        for e in iter {
            self.add(e);
        }
    }
}

impl FromIterator<usize> for Quantifier {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let mut quantifier = Quantifier::new();
        quantifier.extend(iter);
        quantifier
    }
}

pub struct IntoIter {
    index: usize,
    iter: ::std::vec::IntoIter<usize>,
}

impl Iterator for IntoIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let mut elm = self.iter.next()?;
        while elm == 0 {
            self.index += 1;
            elm = self.iter.next()?;
        }
        let re = Some((self.index, elm));
        self.index += 1;
        re
    }
}

impl IntoIterator for Quantifier {
    type Item = (usize, usize);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { index: 0, iter: self.map.into_iter() }
    }
}
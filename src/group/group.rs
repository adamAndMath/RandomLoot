use std::path::Path;
use std::fs::read_to_string;
use std::str::FromStr;

use rand::Rng;
use rand::distributions::WeightedIndex;

use item::Item;
use format::Format;
use quantifier_vec::Quantifier;
use super::GroupErr;

#[derive(Debug)]
pub struct Group {
    format: Format,
    items: Vec<(usize, Item)>,
    generator: WeightedIndex<usize>,
}

impl Group {
    pub fn new(format: Format, items: Vec<(usize, Item)>) -> Result<Self, GroupErr> {
        let generator = WeightedIndex::new(items.iter().map(|(i,_)|*i))?;
        Ok(Group { format, items, generator: generator })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Group, GroupErr> {
        read_to_string(path)?.parse()
    }

    pub fn generate(&self, amount: usize) -> Vec<(&Item, usize)> {
        let quantifier = rand::thread_rng().sample_iter(&self.generator).take(amount).collect::<Quantifier>();
        quantifier.into_iter().map(|(i, q)| (&self.items[i].1, q)).collect()
    }

    pub fn print(&self, amount: usize) {
        for (item, q) in self.generate(amount) {
            println!("{}x{}", q, self.format.to_string(item));
        }
    }
}

impl FromStr for Group {
    type Err = GroupErr;

    fn from_str(s: &str) -> Result<Self, GroupErr> {
        let mut lines = s.lines().enumerate();
        let (_, header) = lines.next().ok_or(GroupErr::EmptyFile)?;

        let format = header.parse::<Format>()?;
        let items = collect_result(lines.map(|(i, s)| format.parse(s).map_err(|e| (i, e))))?;
        Group::new(format, items)
    }
}

fn collect_result<R, E, I: Iterator<Item = Result<R,E>>>(iter: I) -> Result<Vec<R>, Vec<E>> {
    let mut ok: Vec<R> = vec!();
    let mut err: Vec<E> = vec!();

    for e in iter {
        match e {
            Ok(r) => ok.push(r),
            Err(e) => err.push(e),
        }
    }

    if err.is_empty() {
        Ok(ok)
    } else {
        Err(err)
    }
}

use test::Bencher;
use std::collections::HashMap;
use quantifier::Quantifier;
use test::black_box;

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
    let q = data.into_iter().collect::<Quantifier<_>>();
    let t = q.into_iter().collect::<HashMap<_,_>>();
    let e = quantities.into_iter().collect::<HashMap<_,_>>();

    assert_eq!(t, e);
}

#[bench]
fn quantifier_1m_no_repeates(b: &mut Bencher) {
    b.iter(||(0..black_box(1000000)).into_iter().collect::<Quantifier<_>>())
}

#[bench]
fn quantifier_1m_only_repeates(b: &mut Bencher) {
    b.iter(||(0..black_box(1000000)).into_iter().map(|_|17420).collect::<Quantifier<_>>())
}

use test::Bencher;
use generator::Generator;

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

#[bench]
fn generate_1m_from_single(b: &mut Bencher) {
    let g = vec![(1, 1531)].into_iter().collect::<Generator<_>>();
    b.iter(|| g.generate().nth(1000000));
}

#[bench]
fn generate_1m_from_single_sized_1k(b: &mut Bencher) {
    let g = vec![(1000, 1531)].into_iter().collect::<Generator<_>>();
    b.iter(|| g.generate().nth(1000000));
}

#[bench]
fn generate_1m_from_1k(b: &mut Bencher) {
    let g = (0..1000).into_iter().map(|i|(1,i)).collect::<Generator<_>>();
    b.iter(|| g.generate().nth(1000000));
}

#[bench]
fn generate_1m_from_1k_self_sized(b: &mut Bencher) {
    let g = (0..1000).into_iter().map(|i|(i,i)).collect::<Generator<_>>();
    b.iter(|| g.generate().nth(1000000));
}

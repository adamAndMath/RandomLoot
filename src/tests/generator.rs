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

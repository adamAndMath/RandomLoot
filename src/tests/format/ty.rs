use format::{Type, Unit};

#[test]
fn bool() {
    assert_eq!("bool".parse(), Ok(Type::Bool("yes".to_owned(), "no".to_owned())))
}

#[test]
fn bool_labeled() {
    assert_eq!("bool[yep, nope]".parse(), Ok(Type::Bool("yep".to_owned(), "nope".to_owned())))
}

#[test]
fn int() {
    assert_eq!("int".parse(), Ok(Type::Int(Unit::default())))
}

#[test]
fn int_postfix() {
    assert_eq!("int[apples]".parse(), Ok(Type::Int(Unit(vec![(1, "apples".to_owned())]))))
}

#[test]
fn float() {
    assert_eq!("float[km]".parse(), Ok(Type::Int(Unit())))
}

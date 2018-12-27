use format::{Type, Unit};

#[test]
fn bool() {
    assert_eq!("bool".parse(), Ok(Type::Bool("true".to_owned(), "false".to_owned())))
}

#[test]
fn bool_labeled() {
    assert_eq!("bool[yes, no]".parse(), Ok(Type::Bool("yes".to_owned(), "no".to_owned())))
}

#[test]
fn int() {
    assert_eq!("int".parse(), Ok(Type::Int(Unit::default())))
}

#[test]
fn int_postfix() {
    assert_eq!("int[apples]".parse(), Ok(Type::Int(Unit(vec![("apples".to_owned(), 1)]))))
}

#[test]
fn int_units() {
    assert_eq!("int[p=1,£=100]".parse(), Ok(Type::Int(Unit(vec![
        ("p".to_owned(), 1),
        ("£".to_owned(), 100),
    ]))))
}

#[test]
fn float() {
    assert_eq!("float".parse(), Ok(Type::Float(Unit::default())))
}

#[test]
fn float_postfix() {
    assert_eq!("float[km]".parse(), Ok(Type::Float(Unit(vec![("km".to_owned(), 1.)]))))
}

#[test]
fn float_units() {
    assert_eq!("float[m=1,km=1000,cm=0.01,mm=0.001]".parse(), Ok(Type::Float(Unit(vec![
        ("mm".to_owned(), 0.001),
        ("cm".to_owned(), 0.01),
        ("m".to_owned(), 1.),
        ("km".to_owned(), 1000.),
    ]))))
}

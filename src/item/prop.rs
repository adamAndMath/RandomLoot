use std::fmt;

pub enum Prop {
    PropBool(bool),
    PropStr(String),
    PropF32(f32),
    PropI32(i32),
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = self;
        match p {
            &Prop::PropBool(b) => write!(f, "{}", b),
            &Prop::PropStr(ref s) => write!(f, "{}", s),
            &Prop::PropF32(v) => write!(f, "{}", v),
            &Prop::PropI32(v) => write!(f, "{}", v),
        }
    }
}
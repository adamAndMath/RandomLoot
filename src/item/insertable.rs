use super::prop::Prop;

pub trait Insertable {
    fn wrap(self) -> Prop;
    fn unwrap(&Prop) -> Option<&Self>;
}

impl Insertable for bool {
    fn wrap(self) -> Prop {
        Prop::PropBool(self)
    }

    fn unwrap(p: &Prop) -> Option<&Self> {
        match p {
            &Prop::PropBool(ref b) => Some(b),
            _ => None,
        }
    }
}

impl Insertable for String {
    fn wrap(self) -> Prop {
        Prop::PropStr(self)
    }

    fn unwrap(p: &Prop) -> Option<&Self> {
        match p {
            &Prop::PropStr(ref s) => Some(s),
            _ => None,
        }
    }
}

impl Insertable for f32 {
    fn wrap(self) -> Prop {
        Prop::PropF32(self)
    }

    fn unwrap(p: &Prop) -> Option<&Self> {
        match p {
            &Prop::PropF32(ref v) => Some(v),
            _ => None,
        }
    }
}

impl Insertable for i32 {
    fn wrap(self) -> Prop {
        Prop::PropI32(self)
    }

    fn unwrap(p: &Prop) -> Option<&Self> {
        match p {
            &Prop::PropI32(ref v) => Some(v),
            _ => None,
        }
    }
}
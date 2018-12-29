use std::fmt;
use std::any::Any;

pub trait Prop: fmt::Display + fmt::Debug + Sync {
    fn as_any(&self) -> &dyn Any;
}

macro_rules! impl_prop {
    ($($T:ty,)*) => (impl_prop!{$($T),*});
    ($($T:ty),*) => {
        $(impl Prop for $T {
            fn as_any(&self) -> &dyn Any {
                self
            }
        })*
    };
}

impl_prop!{
    bool,
    String,
    f32,
    isize,
}
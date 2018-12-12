use std::fmt;

pub trait Prop: fmt::Display + fmt::Debug {}

impl Prop for bool {}

impl Prop for String {}

impl Prop for f32 {}

impl Prop for isize {}
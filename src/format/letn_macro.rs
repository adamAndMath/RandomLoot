macro_rules! letn {
    ($($v:ident),+ =$iter:expr) => {
        let iter = &mut ($iter);
        $(
            let $v = iter.next();
        )+
        drop(iter);
    };
    ($($v:ident?($e:expr)),+ =$iter:expr) => {
        let iter = &mut ($iter);
        $(
            let $v = iter.next().ok_or($e)?;
        )+
        drop(iter);
    };
}

//letn!((a, b, c, d) = iter);

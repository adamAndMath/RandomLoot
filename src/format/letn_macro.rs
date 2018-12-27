macro_rules! letn {
    ($($v:ident),+ =$iter:expr) => {
        let iter = &mut ($iter);
        $(
            let $v = iter.next();
        )+
        drop(iter);
    };
}

//letn!((a, b, c, d) = iter);

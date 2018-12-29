mod ty;
mod unit;
mod var;

#[bench]
fn bench_one_of_each(b: &mut test::Bencher) {
    b.iter(|| "a: bool; b: int; c: float; d: str".parse::<::format::Format>());
}

#[bench]
fn bench_one_line(b: &mut test::Bencher) {
    let format = "a: bool; b: int; c: float; d: str".parse::<::format::Format>().unwrap();
    b.iter(|| format.parse("true; 742; 5.23; House; 10"));
}
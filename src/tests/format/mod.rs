mod ty;
mod unit;
mod var;

#[bench]
fn bench_on_of_each(b: &mut test::Bencher) {
    b.iter(|| "a: bool; b: int; c: float; d: str".parse::<::format::Format>());
}
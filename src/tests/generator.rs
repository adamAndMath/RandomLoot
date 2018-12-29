use rand::distributions::WeightedIndex;
use rand::thread_rng;
use rand::Rng;
use std::iter::repeat;
use test::Bencher;

#[bench]
fn generate_1m_from_single(b: &mut Bencher) {
    let g = WeightedIndex::new(vec![1]).unwrap();
    b.iter(|| thread_rng().sample_iter(&g).take(1000000).collect::<Vec<_>>());
}

#[bench]
fn generate_1m_from_single_sized_1k(b: &mut Bencher) {
    let g = WeightedIndex::new(vec![1000]).unwrap();
    b.iter(|| thread_rng().sample_iter(&g).take(1000000).collect::<Vec<_>>());
}

#[bench]
fn generate_1m_from_1k(b: &mut Bencher) {
    let g = WeightedIndex::new(repeat(1).take(1000)).unwrap();
    b.iter(|| thread_rng().sample_iter(&g).take(1000000).collect::<Vec<_>>());
}

#[bench]
fn generate_1m_from_1k_self_sized(b: &mut Bencher) {
    let g = WeightedIndex::new(0..1000).unwrap();
    b.iter(|| thread_rng().sample_iter(&g).take(1000000).collect::<Vec<_>>());
}

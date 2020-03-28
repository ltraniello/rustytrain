#![feature(test)]

extern crate test;
extern crate rusttype;

use rusttype::Point;
use std::vec::Vec;

use test::Bencher;
use rand::Rng;
use std::mem::replace;


#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| {vec!})
}

#[bench]
fn count_one(b: &mut Bencher) {
    let mut val : u32 = 0;
    let mut map = std::collections::HashMap::new();

    b.iter(|| { map.insert(rng.gen::<u8>() as usize, val); val += 1; })
}

#[bench]
fn count_square_2(b: &mut Bencher) {
    let mut val : u32 = 0;
    let mut map = std::collections::HashMap::new();

    b.iter(|| { map.insert(rng.gen::<u8>() as usize, val); val += 1; })
}

#[bench]
fn count_square_10(b: &mut Bencher) {
    let mut val : u32 = 0;
    let mut map = std::collections::HashMap::new();

    b.iter(|| { map.insert(rng.gen::<u8>() as usize, val); val += 1; })
}

#[bench]
fn count_square_100(b: &mut Bencher) {
    let mut val : u32 = 0;
    let mut map = std::collections::HashMap::new();

    b.iter(|| { map.insert(rng.gen::<u8>() as usize, val); val += 1; })
}


benchmark_group!(benches, empty, count_one, count_square_2, count_square_10, count_square_100);
benchmark_main!(benches);
#![cfg(test)]
#![feature(test)]
extern crate test;

use std::collections::BTreeSet;
use test::{black_box, Bencher};

const N: u32 = 1;

fn btreeset_peek<T: Clone>(set: &BTreeSet<T>) -> T {
    set.iter().next().unwrap().clone()
}

#[bench]
pub fn btreeset_peek_fast(b: &mut Bencher) {
    let s: BTreeSet<_> = (0..N).collect();
    b.iter(|| {
        let set = black_box(&s);
        let elt = btreeset_peek(set);
        black_box(elt);
    })
}

#[bench]
pub fn btreeset_peek_slow(b: &mut Bencher) {
    let s: BTreeSet<_> = (0..N).collect();
    b.iter(|| {
        let set = black_box(&s);
        let elt = set.iter().next().unwrap().clone();
        black_box(elt);
    })
}

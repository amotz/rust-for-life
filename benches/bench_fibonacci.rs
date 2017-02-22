#![feature(test)]

extern crate test;
extern crate rustforlife;

use test::Bencher;
use rustforlife::fibonacci;

#[bench]
fn bench_f25(b: &mut Bencher) {
    b.iter(|| fibonacci::fibonacci(25));
}
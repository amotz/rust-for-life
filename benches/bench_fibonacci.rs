#![feature(test)]

extern crate test;
extern crate rustforlife;

use test::Bencher;
use rustforlife::fibonacci;

#[bench]
fn bench_f25(b: &mut Bencher) {
    b.iter(|| fibonacci::fibonacci(25));
}

#[bench]
fn bench_f25_dp(b: &mut Bencher) {
    b.iter(|| fibonacci::fibonacci_dp(25));
}
extern crate rustforlife;
use rustforlife::fizzbuzz;

#[test]
fn number() {
    assert_eq!("1", fizzbuzz::fizzbuzz(1));
}
#[test]
fn fizz() {
    assert_eq!("fizz", fizzbuzz::fizzbuzz(3));
    assert_eq!("fizz", fizzbuzz::fizzbuzz(6));
}
#[test]
fn buzz() {
    assert_eq!("buzz", fizzbuzz::fizzbuzz(5));
    assert_eq!("buzz", fizzbuzz::fizzbuzz(10));
}
#[test]
fn fizzbuzz() {
    assert_eq!("fizzbuzz", fizzbuzz::fizzbuzz(15));
    assert_eq!("fizzbuzz", fizzbuzz::fizzbuzz(30));
}
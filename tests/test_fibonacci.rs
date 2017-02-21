extern crate rustforlife;
use rustforlife::fibonacci;

#[test]
fn f25() {
    assert_eq!(75025, fibonacci::fibonacci(25));
}
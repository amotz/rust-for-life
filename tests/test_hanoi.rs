extern crate rustforlife;
use rustforlife::hanoi;

#[test]
fn h1() {
    assert_eq!(1, hanoi::hanoi(1));
}

#[test]
fn h2() {
    assert_eq!(3, hanoi::hanoi(2));
}

#[test]
fn h3() {
    assert_eq!(7, hanoi::hanoi(3));
}

#[test]
fn h4() {
    assert_eq!(15, hanoi::hanoi(4));
}

#[test]
fn h5() {
    assert_eq!(31, hanoi::hanoi(5));
}
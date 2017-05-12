pub fn hanoi(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => (hanoi(n-1) + hanoi(1) + hanoi(n-1)),
    }
}
pub fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => (fibonacci(n-2) + fibonacci(n-1)),
    }
}

pub fn fibonacci_dp(n: i32) -> i32 {
    let mut num1 = 0;
    let mut num2 = 1;    
    let mut fib = 0;
    for _ in 0..n-1 {
        fib = num1 + num2;
        num1 = num2;
        num2 = fib;
    }
    fib
}
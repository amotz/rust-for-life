extern crate rustforlife;

use rustforlife::fizzbuzz;
use rustforlife::fibonacci;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse::<i32>().unwrap();
    
    // FizzBuzz
    /*
    for n in 1..num+1 {
        println!("{}", fizzbuzz::fizzbuzz(n));
    }
    */

    // Fibonacci
    println!("{}", fibonacci::fibonacci(num));

}

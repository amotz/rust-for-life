extern crate rustforlife;

use rustforlife::fizzbuzz;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args[1].parse::<i32>().unwrap();
    for n in 1..num+1 {
        println!("{}", fizzbuzz::fizzbuzz(n));
    }
}

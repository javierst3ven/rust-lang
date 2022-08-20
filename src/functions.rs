#![allow(unused)]

use rand::Rng;
use std::io;

pub fn print_range(a: isize, b: isize) -> () {
    for item in a..b+1 {
        println!(":: {}", item)
    }
}

pub fn num_range(a: isize, b: isize) -> isize {
    rand::thread_rng().gen_range(a..=b)
}

pub fn add_two() -> i8 {
    println!("Write two numbers: ");
    let mut first_number = String::new();
    let mut second_number = String::new();
    io::stdin().read_line(&mut first_number);
    io::stdin().read_line(&mut second_number);
    let first_number: isize = first_number.trim().parse().expect("Type a number");
    let second_number: isize = second_number.trim().parse().expect("Type a number");
    print_range(first_number, second_number);
    return 0
}

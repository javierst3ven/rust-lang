#![allow(unused)]

use std::io;
use rand::Rng;

mod functions;

fn main() {
    println!("Hello, world!");
    let random_num: i32 = rand::thread_rng().gen_range(1..=101);
    functions::print_range(1, 10);
    add_two();
}

fn add_two() -> () {
    println!("Write two numbers: ");
    let mut first_number = String::new();
    let mut second_number = String::new();
    io::stdin().read_line(&mut first_number);
    io::stdin().read_line(&mut second_number);
    let first_number: isize = first_number.trim().parse().expect("Type a number");
    let second_number: isize = second_number.trim().parse().expect("Type a number");
    functions::print_range(first_number, second_number);
}

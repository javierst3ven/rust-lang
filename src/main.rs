#![allow(unused)]

use std::io;
use rand::Rng;

mod functions;

fn main() {
    println!("Hello, world!");
    let random_num: i32 = rand::thread_rng().gen_range(1..=101);
    functions::print_range(1, 10);
    functions::add_two();
    println!("Finalizando el programa");
}

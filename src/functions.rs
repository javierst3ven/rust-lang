#![allow(unused)]

use rand::Rng;

pub fn print_range(a: isize, b: isize) -> () {
    for item in a..b {
        println!(":: {}", item)
    }
}

pub fn num_range(a: isize, b: isize) -> isize {
    rand::thread_rng().gen_range(a..b)
}

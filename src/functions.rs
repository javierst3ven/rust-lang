#![allow(unused)]

use rand::Rng;

pub fn print_range(a: isize, b: isize) -> () {
    for item in a..b {
        println!(":: {}", item)
    }
}

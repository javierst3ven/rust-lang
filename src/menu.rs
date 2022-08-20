#![allow(unused)]

use std::io;

pub fn menu() -> () {
    println!("MENU");
    println!("Selecciona una opción: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number);
}

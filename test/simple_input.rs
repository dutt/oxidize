#!/usr/bin/oxidize

use std::io;

fn main() {
    let mut name  = String::new();
    println!("Enter name:", );
    io::stdin().read_line(&mut name);
    println!("name {}", name.trim());
}

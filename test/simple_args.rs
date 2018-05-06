#!/usr/bin/oxidize

use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("args!");
    for arg in args.iter() {
        println!("{}", arg);
    }
}

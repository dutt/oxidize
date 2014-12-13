#!/usr/bin/oxidize

mod test_use_mod;

fn main() {
    let is_cookie = test_use_mod::is_cookie();
    println!("is_cookie? {}", is_cookie);
}
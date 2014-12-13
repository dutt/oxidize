#!/usr/bin/oxidize

#[allow(unused_must_use)]
fn main() {
    for _ in range(0i, 2i) {
        println!("fo {}", std::os::args()[2]);
        std::io::timer::sleep(std::time::duration::Duration::seconds(1))
    }
    /*println!("Enter name:");
    let name = match std::io::stdin().read_line() {
        Ok(text) => text,
        Err(e) => panic!("Failed to read stdin: {}", e),
    };
    println!("Name: {}", name);
    std::io::stdio::stderr().write_line("exit");*/
}
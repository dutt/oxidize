oxidize
=======

Run rust code with a #!/path/oxidize. No need for a cargo project or anything else.

stdout, stderr and stdin are handled as well as argument passing.

example
=======

```rust
#!/usr/bin/oxidize

fn main() {
    for _ in range(0i, 2i) {
        println!("fo {}", std::os::args()[2]);
        std::io::timer::sleep(std::time::duration::Duration::seconds(1))
    }
}```

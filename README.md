oxidize
=======

Run rust code with a #!/path/oxidize. No need for a cargo project or anything else. It's compiled into a cache at ~/.rusted and only re-compiled when needed.

stdout, stderr and stdin are handled as well as argument passing.

example
------

```rust
#!/usr/bin/oxidize

fn main() {
    for _ in range(0i, 2i) {
        println!("fo {}", std::os::args()[2]);
        std::io::timer::sleep(std::time::duration::Duration::seconds(1))
    }
}
```

and then run as usual
```
$ ./myscript foo bahr
fo foo
fo foo
$
```

Contributing
---------

This is my first public rust project; all sorts of suggestions, tips or general helpfullness is appreciated.

Why?
-----
We're writing rust for systems, embedded, games, web so I got curious about using it for small scripts.
Well, that and I was bored.

Thanks
-----
To all the helpful people in #rust

Jared https://jaredly.github.io/2014/11/22/rust-compiling-rust-adventures-with-librustc/

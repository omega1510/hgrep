# hgrep
A simplified version of grep, written in Rust.

## Installation
The rust compiler must be present on your system and the `cargo` executable must be in your `PATH`.
```shell
$ git clone https://github.com/omega1510/hgrep.git
$ cd hgrep
$ cargo build --release
```
This places an executable in `target/release/`.

## Usage
```txt
Usage: hgrep [options] <query> <filename>
        
Options:
    -h, --help           Display this message.
    -i, --ignore-case    Search case insensitively.
```

---

_The framework for this program was created following the [Rust Book](https://doc.rust-lang.org/book/). However, I have made some modifications and improvements. This program is still very well a learning exercise in building CLIs with Rust._
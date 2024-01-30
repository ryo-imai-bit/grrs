# grrs

https://rust-cli.github.io/book/index.html

## Requirement

Rust 1.31.0 (or later) 

# Usage

```bash
# run
$ cargo run -- <PATTERN> <PATH>
# example
$ cargo run -- stdout src/main.rs
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
```

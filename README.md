[![CI](https://github.com/lpenz/lineriver/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/lineriver/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/lineriver/badge.svg?branch=main)](https://coveralls.io/github/lpenz/lineriver?branch=main)
[![crates.io](https://img.shields.io/crates/v/lineriver)](https://crates.io/crates/lineriver)
[![doc.rs](https://docs.rs/lineriver/badge.svg)](https://docs.rs/lineriver)

# lineriver

**lineriver** is a rust crate that provides a non-blocking buffered line
reader for [`Read`] objects.

The [`LineReader`] object is akin to a [`BufReader`] object
that returns only complete lines, but without blocking.
The [`LineRead`] trait, on the other hand, is akin to the
[`BufRead`] trait - it concentrates the public API and allows us
to create agnostic collections of LineReaders with distinct
underlying types.

This crate works very well with the [polling] crate, which allows
us to block waiting on data to be available in any one of multiple
streams (files, sockets, etc.). It's an alternative to using
threads and/or [tokio].

See [`LineReader`] for details.

## Usage

The simplest way to explain how to use `LineReader` is
with a busy-loop example:

```rust
use lineriver::{LineReader, LineRead};

let mut linereader = LineReader::new(reader)?;
while !linereader.eof() {
    linereader.read_available()?;
    let lines = linereader.lines_get();
    for line in lines {
        print!("{}", line);
    }
}
```

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`read_line`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
[polling]: https://docs.rs/polling/latest/polling/index.html
[tokio]: https://tokio.rs/
[github]: https://github.com/lpenz/lineriver
[`tcp_line_echo`]: https://github.com/lpenz/lineriver/blob/main/examples/tcp_line_echo.rs

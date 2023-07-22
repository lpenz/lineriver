[![CI](https://github.com/lpenz/lineriver/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/lineriver/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/lineriver/badge.svg?branch=main)](https://coveralls.io/github/lpenz/lineriver?branch=main)

# lineriver

**linereader** is a rust crate that provides a non-blocking buffered line
reader for [Read] objects.

The [`LineReaderNonBlock`] object is akin to a [BufReader] object
that returns only complete lines, but without blocking. It also
implements the [BufRead] trait, but deviates from it by not
blocking in [`read_line`], and allowing it to be called multiple
times.

This crate works very well with the [polling] crate, which allows
us to block waiting on data to be available in any one of multiple
streams (files, sockets, etc.). It's an alternative to using
threads and/or [tokio].

See [`LineReaderNonBlock`] for details.

## Usage

The simplest way to explain how to use `LineReaderNonBlock` is
with a busy-loop example:

```rust
use lineriver::LineReaderNonBlock;

let mut linereader = LineReaderNonBlock::new(reader)?;
while !linereader.eof() {
    linereader.read_available()?;
    let lines = linereader.lines_get();
    for line in lines {
        print!("{}", line);
    }
}
```

[Read]: https://doc.rust-lang.org/std/io/trait.Read.html
[BufReader]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[BufRead]: https://doc.rust-lang.org/std/io/trait.BufRead.html
[`read_line`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
[polling]: https://docs.rs/polling/latest/polling/index.html
[tokio]: https://tokio.rs/
[github]: https://github.com/lpenz/lineriver
[`tcp_line_echo`]: https://github.com/lpenz/lineriver/blob/main/examples/tcp_line_echo.rs



[Read]: https://doc.rust-lang.org/std/io/trait.Read.html
[BufReader]: https://doc.rust-lang.org/std/io/struct.BufReader.html

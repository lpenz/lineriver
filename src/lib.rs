// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! **linereader** is a rust crate that provides a non-blocking buffered line
//! reader for [Read] objects.
//!
//! The [`LineReaderNonBlock`] object is akin to a [BufReader] object
//! that returns only complete lines, but without blocking. It also
//! implements the [BufRead] trait, but deviates from it by not
//! blocking in [`read_line`], and allowing it to be called multiple
//! times.
//!
//! This crate works very well with the [polling] crate, which allows
//! us to block waiting on data to be available in any one of multiple
//! streams (files, sockets, etc.). It's an alternative to using
//! threads and/or [tokio].
//!
//! See [`LineReaderNonBlock`] for details.
//!
//! # Usage
//!
//! The simplest way to explain how to use `LineReaderNonBlock` is
//! with a busy-loop example:
//!
//! ```
//! # use std::io::Write;
//! # use std::os::unix::net::UnixStream;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #    let (mut writer, reader) = UnixStream::pair()?;
//! #    writer.write_all(b"test\n")?;
//! #    writer.flush()?;
//! #    drop(writer);
//! use lineriver::LineReaderNonBlock;
//!
//! let mut linereader = LineReaderNonBlock::new(reader)?;
//! while !linereader.eof() {
//!     linereader.read_available()?;
//!     let lines = linereader.lines_get();
//!     for line in lines {
//!         print!("{}", line);
//!     }
//! }
//! #    Ok(())
//! # }
//! ```
//!
//! # Examples
//!
//! ## `tcp_line_echo.rs`
//!
//! The following example is a full TCP server that prints all lines
//! received from clients, using the [polling] crate to do so
//! efficiently:
//!
//! ```no_run
#![doc = include_str!("../examples/tcp_line_echo.rs")]
//! ```
//!
//! [Read]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [BufReader]: https://doc.rust-lang.org/std/io/struct.BufReader.html
//! [BufRead]: https://doc.rust-lang.org/std/io/trait.BufRead.html
//! [`read_line`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
//! [polling]: https://docs.rs/polling/latest/polling/index.html
//! [tokio]: https://tokio.rs/
//! [github]: https://github.com/lpenz/lineriver
//! [`tcp_line_echo`]: https://github.com/lpenz/lineriver/blob/main/examples/tcp_line_echo.rs

mod blocking;
pub mod line_reader_nonblock;
pub use self::line_reader_nonblock::*;

// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! **lineriver** is a rust crate that provides a non-blocking buffered line
//! reader for [`Read`] objects.
//!
//! The [`LineReader`] object is akin to a [`BufReader`] object
//! that returns only complete lines, but without blocking.
//! The [`LineRead`] trait, on the other hand, is akin to the
//! [`BufRead`] trait - it concentrates the public API and allows us
//! to create agnostic collections of LineReaders with distinct
//! underlying types.
//!
//! This crate works very well with the [polling] crate, which allows
//! us to block waiting on data to be available in any one of multiple
//! streams (files, sockets, etc.). It's an alternative to using
//! threads and/or [tokio].
//!
//! See [`LineReader`] for details.
//!
//! # Usage
//!
//! The simplest way to explain how to use `LineReader` is
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
//! use lineriver::{LineReader, LineRead};
//!
//! let mut linereader = LineReader::new(reader)?;
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
//! [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
//! [`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
//! [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
//! [`read_line`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
//! [polling]: https://docs.rs/polling/latest/polling/index.html
//! [tokio]: https://tokio.rs/
//! [github]: https://github.com/lpenz/lineriver
//! [`tcp_line_echo`]: https://github.com/lpenz/lineriver/blob/main/examples/tcp_line_echo.rs

mod blocking;

pub mod linereader;
pub use self::linereader::*;

pub mod lineread;
pub use self::lineread::*;

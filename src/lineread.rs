// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! This module has the generic trait [`LineRead`].

use std::io;

/// Generic trait implemented by all `LineReader<R>`.
///
/// This trait can be used to create a collection of LineReaders that
/// use different underlying types, by using trait objects.
pub trait LineRead {
    /// Returns true if we have already reached EOF in the underlying
    /// `Read` object.
    ///
    /// Once this function returns true, `read_once` and
    /// `read_available` stop having any effect, they return
    /// immediately.
    ///
    /// The buffer may have complete lines, so a last call to
    /// `lines_get` is recommended.
    fn eof(&self) -> bool;

    /// Performs a single read operation on the underlying `Read`
    /// object.
    ///
    /// Returns `Ok(true)` if we have already reached EOF,
    /// `Ok(false)` if we have not. That doesn't mean that there is
    /// more data available to read immediately, it just means that
    /// the file descriptor is still open.
    ///
    /// This function can also return an [`std::io::Error`] if one is
    /// found, or if an invalid UTF-8 sequence is read.
    fn read_once(&mut self) -> Result<bool, io::Error>;

    /// Reads all available data into the internal line buffer.
    ///
    /// This method just calls [`Self::read_once`] until it returns `false`.
    fn read_available(&mut self) -> Result<(), io::Error> {
        while self.read_once()? {}
        Ok(())
    }

    /// Returns the internal line buffer.
    ///
    /// This method transfers ownership of the buffer to the caller,
    /// effectively clearing the internal buffer.
    fn lines_get(&mut self) -> Vec<String>;
}

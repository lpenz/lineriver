// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! This module has the generic trait [`LineRead`].

use std::io;
use std::os::fd::AsRawFd;

/// Trait for buffered non-blocking readeres that return only complete
/// lines.
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

    /// Reads all available data into the internal line buffer, or at
    /// least until a complete line is available.
    ///
    /// This method just calls [`Self::read_once`] until it returns
    /// `false` or [`Self::has_lines`] returns `true`.
    fn read_available(&mut self) -> Result<(), io::Error> {
        while self.read_once()? && !self.has_lines() {}
        Ok(())
    }

    /// Returns the internal line buffer.
    ///
    /// This method transfers ownership of the buffer to the caller,
    /// effectively clearing the internal buffer.
    fn lines_get(&mut self) -> Vec<String>;

    /// Returns `true` if there are complete lines in the internal buffer.
    ///
    /// If this returns `true`, [`Self::lines_get`] won't return an
    /// empty vector.
    fn has_lines(&mut self) -> bool;
}

/// Trait for buffered non-blocking readeres that return only complete
/// lines and is backed by an entity that has a file descriptor.
///
/// This trait can be used to create a collection of LineReaders that
/// use different underlying types, by using trait objects.
pub trait LineReadFd: LineRead + AsRawFd {}

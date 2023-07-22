// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

//! This module has the main type of this crate: [`LineReaderNonBlock`]

use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::{mem, str};

use crate::blocking;

const BUFFER_SIZE: usize = 8192;

/// Buffered non-blocking reader that returns only complete lines.
#[derive(Debug)]
pub struct LineReaderNonBlock<R: AsRawFd + Read> {
    reader: R,
    at_eof: bool,
    buf: Vec<u8>,
    used: usize,
    lines: Vec<String>,
}

fn u8array_to_string(buf: &[u8]) -> Result<String, io::Error> {
    match str::from_utf8(buf) {
        Ok(line) => Ok(line.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

impl<R: AsRawFd + Read> LineReaderNonBlock<R> {
    /// Creates a new LineReaderNonBlock.
    pub fn new(reader: R) -> Result<Self, io::Error> {
        let fd = reader.as_raw_fd();
        blocking::disable(fd)?;
        Ok(Self {
            reader,
            at_eof: false,
            buf: Default::default(),
            used: 0,
            lines: Default::default(),
        })
    }

    /// Returns true if we have already reached EOF in the underlying
    /// `Read` object.
    ///
    /// Once this function returns true, `read_once` and
    /// `read_available` stop having any effect, they return
    /// immediately.
    ///
    /// The buffer may have complete lines, so a last call to
    /// `lines_get` is recommended.
    pub fn eof(&self) -> bool {
        self.at_eof
    }

    fn eval_buf(&mut self, mut pos: usize) -> Result<(), io::Error> {
        loop {
            if let Some(inewline) = memchr::memchr(b'\n', &self.buf[pos..self.used]) {
                // Found a newline.
                let mut line = self.buf.split_off(pos + inewline + 1);
                self.used -= pos + inewline + 1;
                // They are swapped at the moment, unswap:
                mem::swap(&mut self.buf, &mut line);
                // Convert line to string and append to self.lines:
                self.lines.push(u8array_to_string(&line)?);
                pos = 0;
            } else {
                // No newline read.
                return Ok(());
            }
        }
    }

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
    pub fn read_once(&mut self) -> Result<bool, io::Error> {
        if self.at_eof {
            return Ok(false);
        }
        if self.buf.len() < self.used + BUFFER_SIZE {
            self.buf.resize(self.used + BUFFER_SIZE, 0);
        }
        let oldused = self.used;
        let buf = self.buf.as_mut_slice();
        let r = self.reader.read(&mut buf[self.used..]);
        match r {
            Ok(0) => {
                if self.used > 0 {
                    let mut lastline = mem::take(&mut self.buf);
                    lastline.truncate(self.used);
                    self.lines.push(u8array_to_string(&lastline)?);
                    self.used = 0;
                }
                self.at_eof = true;
            }
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                // No data availble, just let the function return
            }
            Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                // Interrupted, just let the function return
            }
            Ok(len) => {
                self.used += len;
                // Look for newlines from "oldused" forward:
                self.eval_buf(oldused)?;
            }
            Err(err) => {
                return Err(err);
            }
        }
        Ok(true)
    }

    /// Reads all available data into the internal line buffer.
    ///
    /// This method just calls [`Self::read_once`] until it returns `false`.
    pub fn read_available(&mut self) -> Result<(), io::Error> {
        while self.read_once()? {}
        Ok(())
    }

    /// Returns the internal line buffer.
    ///
    /// This method transfers ownership of the buffer to the caller,
    /// effectively clearing the internal buffer.
    pub fn lines_get(&mut self) -> Vec<String> {
        mem::take(&mut self.lines)
    }
}

impl<R: AsRawFd + Read> AsRawFd for LineReaderNonBlock<R> {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        self.reader.as_raw_fd()
    }
}

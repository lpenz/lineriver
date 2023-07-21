// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::{mem, str};

mod blocking;

#[derive(Debug)]
pub struct LineReaderNonBlock<R: AsRawFd + Read> {
    reader: R,
    at_eof: bool,
    buf: Vec<u8>,
    used: usize,
    lines: Vec<String>,
}

impl<R: AsRawFd + Read> LineReaderNonBlock<R> {
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

    pub fn eof(&self) -> bool {
        self.at_eof
    }

    fn u8array_to_string(buf: &[u8]) -> Result<String, io::Error> {
        match str::from_utf8(buf) {
            Ok(line) => Ok(line.to_string()),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
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
                self.lines.push(Self::u8array_to_string(&line)?);
                pos = 0;
            } else {
                // No newline read.
                return Ok(());
            }
        }
    }

    pub fn read_once(&mut self) -> Result<bool, io::Error> {
        if self.at_eof {
            return Ok(false);
        }
        if self.buf.len() < self.used + 1024 {
            self.buf.resize(self.used + 1024, 0);
        }
        let oldused = self.used;
        let buf = self.buf.as_mut_slice();
        let r = self.reader.read(&mut buf[self.used..]);
        match r {
            Ok(0) => {
                if self.used > 0 {
                    let mut lastline = mem::take(&mut self.buf);
                    lastline.truncate(self.used);
                    self.lines.push(Self::u8array_to_string(&lastline)?);
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

    pub fn lines_get(&mut self) -> Vec<String> {
        mem::take(&mut self.lines)
    }
}

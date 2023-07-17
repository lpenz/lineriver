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
    eof: bool,
    buf: Vec<u8>,
    lines: Vec<String>,
}

impl<R: AsRawFd + Read> LineReaderNonBlock<R> {
    pub fn new(reader: R) -> Result<Self, io::Error> {
        let fd = reader.as_raw_fd();
        blocking::disable(fd)?;
        Ok(Self {
            reader,
            eof: false,
            buf: Default::default(),
            lines: Default::default(),
        })
    }

    fn eval_buf(&mut self, mut pos: usize) -> Result<(), io::Error> {
        loop {
            if let Some(inewline) = memchr::memchr(b'\n', &self.buf[pos..]) {
                // Found a newline.
                let mut line = self.buf.split_off(pos + inewline);
                // They are swapped at the moment, unswap:
                mem::swap(&mut self.buf, &mut line);
                // Convert line to string and append to self.lines:
                match str::from_utf8(&line) {
                    Ok(line) => {
                        self.lines.push(line.to_string());
                        pos = 0;
                    }
                    Err(e) => {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, e));
                    }
                }
            } else {
                // No newline read.
                return Ok(());
            }
        }
    }

    pub fn read_once(&mut self) -> Result<bool, io::Error> {
        if self.eof {
            return Ok(false);
        }
        let oldlen = self.buf.len();
        self.buf.reserve(1024);
        let buf = self.buf.as_mut_slice();
        match self.reader.read(&mut buf[oldlen..]) {
            Ok(0) => {
                self.buf.push(b'\n');
                self.eval_buf(0)?;
                self.eof = true;
            }
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                // No data availble, just let the function return
            }
            Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
                // Interrupted, just let the function return
            }
            Ok(_len) => {
                self.eval_buf(oldlen)?;
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

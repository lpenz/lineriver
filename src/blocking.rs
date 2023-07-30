// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use libc::{F_GETFL, F_SETFL, O_NONBLOCK};
use std::fmt::Debug;
use std::io;
use std::os::fd::AsRawFd;

#[tracing::instrument]
fn fcntl(
    fd: std::os::fd::RawFd,
    cmd: libc::c_int,
    arg: libc::c_int,
) -> Result<libc::c_int, io::Error> {
    let result = unsafe { libc::fcntl(fd, cmd, arg) };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(result)
}

#[tracing::instrument]
pub fn disable<R: AsRawFd + Debug>(reader: R) -> Result<(), io::Error> {
    let fd = reader.as_raw_fd();
    let flags = fcntl(fd, F_GETFL, 0)?;
    fcntl(fd, F_SETFL, flags | O_NONBLOCK)?;
    Ok(())
}

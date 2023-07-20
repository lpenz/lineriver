// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::Write;
use std::os::unix::net::UnixStream;

use color_eyre::Result;

use lineriver::*;

fn reader_for(input: &[u8]) -> Result<LineReaderNonBlock<UnixStream>> {
    let (mut wr, rd) = std::os::unix::net::UnixStream::pair()?;
    let reader = LineReaderNonBlock::new(rd)?;
    wr.write_all(input)?;
    wr.flush()?;
    Ok(reader)
}

#[test]
fn test_oneline_newline() -> Result<()> {
    let mut reader = reader_for(b"test\n")?;
    reader.read_once()?;
    assert_eq!(reader.lines_get(), vec!["test\n"]);
    Ok(())
}

#[test]
fn test_oneline_nonewline() -> Result<()> {
    let mut reader = reader_for(b"test")?;
    // First read_once gets the string.
    reader.read_once()?;
    // Second read_once finds eof.
    reader.read_once()?;
    assert_eq!(reader.lines_get(), vec!["test"]);
    Ok(())
}

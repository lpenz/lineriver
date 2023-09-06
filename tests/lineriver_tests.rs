// Copyright (C) 2023 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::io::Write;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::process::Stdio;

use color_eyre::{eyre::eyre, Result};

use ::lineriver::*;

const SPARKLE_HEART: [u8; 4] = [240, 159, 146, 150];
const INVALID_UTF8: [u8; 4] = [0, 159, 146, 150];

#[tracing::instrument(skip(input))]
fn reader_for(input: &[u8]) -> Result<LineReader<UnixStream>> {
    let (mut wr, rd) = std::os::unix::net::UnixStream::pair()?;
    let reader = LineReader::new(rd)?;
    wr.write_all(input)?;
    wr.flush()?;
    Ok(reader)
}

#[test_log::test]
fn test_oneline_newline() -> Result<()> {
    let mut reader = reader_for(b"test\n")?;
    reader.read_once()?;
    assert_eq!(reader.lines_get(), vec!["test\n"]);
    Ok(())
}

#[test_log::test]
fn test_oneline_nonewline() -> Result<()> {
    let mut reader = reader_for(b"test")?;
    // First read_once gets the string.
    reader.read_once()?;
    // Second read_once finds eof.
    reader.read_once()?;
    assert_eq!(reader.lines_get(), vec!["test"]);
    Ok(())
}

#[test_log::test]
fn test_twoline() -> Result<()> {
    let mut reader = reader_for(b"1\n2\n")?;
    // First read_once gets the string.
    reader.read_once()?;
    assert!(!reader.eof());
    // Second read_once finds eof.
    reader.read_once()?;
    assert!(reader.eof());
    assert_eq!(reader.lines_get(), vec!["1\n", "2\n"]);
    Ok(())
}

#[test_log::test]
fn test_threeline() -> Result<()> {
    let mut reader = reader_for(b"1\n\n3\n")?;
    // We only need one read_available to find eof
    reader.read_available()?;
    assert!(reader.has_lines());
    assert_eq!(reader.lines_get(), vec!["1\n", "\n", "3\n"]);
    Ok(())
}

#[test_log::test]
fn test_empty() -> Result<()> {
    let mut reader = reader_for(b"")?;
    reader.read_once()?;
    assert!(reader.lines_get().is_empty());
    Ok(())
}

#[test_log::test]
fn test_empty_line() -> Result<()> {
    let mut reader = reader_for(b"\n")?;
    reader.read_once()?;
    assert_eq!(reader.lines_get(), vec!["\n"]);
    Ok(())
}

#[test_log::test]
fn test_read_past_end() -> Result<()> {
    let mut reader = reader_for(b"")?;
    for _ in 0..10 {
        reader.read_once()?;
    }
    assert!(reader.eof());
    assert!(reader.lines_get().is_empty());
    Ok(())
}

#[test_log::test]
fn test_utf8() -> Result<()> {
    let heart = format!("\n{}\n\n", std::str::from_utf8(&SPARKLE_HEART)?);
    let mut reader = reader_for(heart.as_bytes())?;
    reader.read_once()?;
    assert_eq!(
        reader.lines_get(),
        vec![
            "\n",
            &format!("{}\n", std::str::from_utf8(&SPARKLE_HEART)?),
            "\n"
        ]
    );
    Ok(())
}

#[test_log::test]
fn test_invalid_utf8() -> Result<()> {
    let mut invalid = Vec::from(INVALID_UTF8);
    invalid.push(b'\n');
    let mut reader = reader_for(&invalid)?;
    assert!(match reader.read_once() {
        Ok(_) => false,
        Err(_) => true,
    });
    Ok(())
}

#[test_log::test]
fn test_addlines() -> Result<()> {
    let (mut wr, rd) = std::os::unix::net::UnixStream::pair()?;
    let mut reader = LineReader::new(rd)?;
    reader.read_once()?;
    assert!(reader.lines_get().is_empty());
    wr.write_all(b"1\n2")?;
    assert!(reader.read_once()?);
    assert_eq!(reader.lines_get(), vec!["1\n"]);
    reader.read_once()?;
    assert!(reader.lines_get().is_empty());
    wr.write_all(b"\n3\n4")?;
    reader.read_once()?;
    assert_eq!(reader.lines_get(), vec!["2\n", "3\n"]);
    wr.shutdown(Shutdown::Write)?;
    reader.read_once()?;
    assert_eq!(reader.lines_get(), vec!["4"]);
    assert!(reader.lines_get().is_empty());
    assert!(!reader.read_once()?);
    assert!(reader.eof());
    let _ = format!("{:?}", reader);
    Ok(())
}

#[test_log::test]
fn test_trat_reader() -> Result<()> {
    let array = "abcdefgh".as_bytes();
    let linereader = LineReader::from_nonblocking(array)?;
    let _traitobj = &linereader as &dyn LineRead;
    Ok(())
}

#[test_log::test]
fn test_trat_readerfd() -> Result<()> {
    let mut child = Command::new("true")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdout = LineReader::new(
        child
            .stdout
            .take()
            .ok_or_else(|| eyre!("error taking stdout"))?,
    )?;
    let stderr = LineReader::new(
        child
            .stderr
            .take()
            .ok_or_else(|| eyre!("error taking stderr"))?,
    )?;
    let linereaders = vec![
        &stdout as &dyn LineReadRawAndFd,
        &stderr as &dyn LineReadRawAndFd,
    ];
    let _rawfds1 = linereaders
        .iter()
        .map(|&s| s.as_raw_fd())
        .collect::<Vec<_>>();
    let _fds1 = linereaders.iter().map(|&s| s.as_fd()).collect::<Vec<_>>();
    let linereaders = vec![&stdout as &dyn LineReadRawFd, &stderr as &dyn LineReadRawFd];
    let _rawfds2 = linereaders
        .iter()
        .map(|s| s.as_raw_fd())
        .collect::<Vec<_>>();
    let linereaders = vec![&stdout as &dyn LineReadFd, &stderr as &dyn LineReadFd];
    let _fds2 = linereaders.iter().map(|s| s.as_fd()).collect::<Vec<_>>();
    Ok(())
}

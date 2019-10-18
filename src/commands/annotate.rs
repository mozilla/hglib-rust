// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub files: &'a [&'a str],
    pub revrange: &'a str,
    pub nofollow: bool,
    pub text: bool,
    pub user: bool,
    pub file: bool,
    pub date: bool,
    pub number: bool,
    pub changeset: bool,
    pub line: bool,
    pub verbose: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            files: &[],
            revrange: "",
            nofollow: false,
            text: false,
            user: false,
            file: false,
            date: false,
            number: false,
            changeset: false,
            line: false,
            verbose: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "annotate",
            self.files,
            "-r",
            self.revrange,
            "--no-follow",
            self.nofollow,
            "-a",
            self.text,
            "-u",
            self.user,
            "-f",
            self.file,
            "-d",
            self.date,
            "-n",
            self.number,
            "-c",
            self.changeset,
            "-l",
            self.line,
            "-v",
            self.verbose,
            "-I",
            self.include,
            "-X",
            self.exclude
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub info: &'a str,
    pub content: &'a [u8],
}

pub struct Lines {
    buf: Vec<u8>,
    pos: usize,
}

impl Lines {
    fn new(buf: Vec<u8>) -> Lines {
        Lines { buf, pos: 0 }
    }

    pub fn next_line(&mut self) -> Result<Option<Line>, HglibError> {
        if self.pos >= self.buf.len() {
            return Ok(None);
        }

        let mut info_end = 0;
        let mut info = "";
        for (n, c) in self.buf[self.pos..].iter().enumerate() {
            if *c == b':' {
                if info_end == 0 {
                    if let Some(c) = self.buf.get(self.pos + n + 1) {
                        if *c == b' ' {
                            info_end = self.pos + n;
                            let _info = unsafe { self.buf.get_unchecked(self.pos..info_end) };
                            info = std::str::from_utf8(_info)?;
                        }
                    }
                }
            } else if *c == b'\n' {
                self.pos += n + 1;
                let content = unsafe { self.buf.get_unchecked(info_end + 2..self.pos - 1) };
                return Ok(Some(Line { info, content }));
            }
        }
        Ok(None)
    }
}

impl Client {
    pub fn annotate(&mut self, x: Arg) -> Result<Lines, HglibError> {
        let (data, _) = x.run(self)?;
        Ok(Lines::new(data))
    }
}

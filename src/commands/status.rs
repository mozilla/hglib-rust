// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a [&'a str],
    pub change: &'a str,
    pub all: bool,
    pub modified: bool,
    pub added: bool,
    pub removed: bool,
    pub deleted: bool,
    pub clean: bool,
    pub unknown: bool,
    pub ignored: bool,
    pub copies: bool,
    pub subrepos: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            rev: &[],
            change: "",
            all: false,
            modified: false,
            added: false,
            removed: false,
            deleted: false,
            clean: false,
            unknown: false,
            ignored: false,
            copies: false,
            subrepos: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "status",
            &[""],
            "--rev",
            self.rev,
            "--change",
            self.change,
            "-A",
            self.all,
            "-m",
            self.modified,
            "-a",
            self.added,
            "-r",
            self.removed,
            "-d",
            self.deleted,
            "-c",
            self.clean,
            "-u",
            self.unknown,
            "-i",
            self.ignored,
            "-C",
            self.copies,
            "-S",
            self.subrepos,
            "-I",
            self.include,
            "-X",
            self.exclude,
            "--print0",
            true
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Code {
    Modified,
    Added,
    Removed,
    Clean,
    Missing,
    NotTracked,
    Ignored,
    Origin,
}

#[derive(Debug, PartialEq)]
pub struct Status {
    pub code: Code,
    pub filename: String,
}

impl Client {
    pub fn status(&mut self, x: Arg) -> Result<Vec<Status>, HglibError> {
        if !x.rev.is_empty() && !x.change.is_empty() {
            return Err(HglibError::from("Cannot specify both rev and change"));
        }

        let (data, _) = x.run(self)?;
        let mut res = Vec::new();
        for line in data.split(|c| *c == b'\0').filter(|l| l.len() >= 3) {
            let c = unsafe { line.get_unchecked(0) };
            let code = match *c {
                b'M' => Code::Modified,
                b'A' => Code::Added,
                b'R' => Code::Removed,
                b'C' => Code::Clean,
                b'!' => Code::Missing,
                b'?' => Code::NotTracked,
                b'I' => Code::Ignored,
                b' ' => Code::Origin,
                _ => {
                    return Err(HglibError::from(format!("Invalid code: {}", *c)));
                }
            };
            let filename = unsafe { line.get_unchecked(2..) };
            let filename = String::from_utf8(filename.to_vec())?;
            res.push(Status { code, filename });
        }
        Ok(res)
    }
}

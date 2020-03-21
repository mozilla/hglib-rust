// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a str,
    pub all: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            rev: "",
            all: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "manifest",
            &[""],
            "-r",
            self.rev,
            "--all",
            self.all,
            "--debug",
            true
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct File {
    pub node: String,
    pub perm: String,
    pub symlink: bool,
    pub executable: bool,
    pub filename: String,
}

#[derive(Debug, PartialEq)]
pub enum Manifest {
    All(Vec<String>),
    Info(Vec<File>),
}

impl Client {
    pub fn manifest(&mut self, x: Arg) -> Result<Manifest, HglibError> {
        let (data, _) = x.run(self)?;
        if x.all {
            Ok(Manifest::All(
                data.split(|c| *c == b'\n')
                    .filter(|l| !l.is_empty())
                    .map(|s| String::from_utf8(s.to_vec()).unwrap())
                    .collect(),
            ))
        } else {
            let mut res = Vec::new();
            for line in data.split(|c| *c == b'\n').filter(|l| !l.is_empty()) {
                if line.len() >= 48 {
                    res.push(File {
                        node: String::from_utf8(unsafe { line.get_unchecked(..40).to_vec() })?,
                        perm: String::from_utf8(unsafe { line.get_unchecked(41..44).to_vec() })?,
                        symlink: unsafe { *line.get_unchecked(45) == b'@' },
                        executable: unsafe { *line.get_unchecked(45) == b'*' },
                        filename: String::from_utf8(unsafe { line.get_unchecked(47..).to_vec() })?,
                    });
                } else {
                    return Err(HglibError::from(format!(
                        "Hglib error: invalid length for line: {} <= 48",
                        line.len()
                    )));
                }
            }
            Ok(Manifest::Info(res))
        }
    }
}

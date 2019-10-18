// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub file: &'a [&'a str],
    pub all: bool,
    pub listfiles: bool,
    pub mark: bool,
    pub unmark: bool,
    pub tool: &'a str,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            file: &[],
            all: false,
            listfiles: false,
            mark: false,
            unmark: false,
            tool: "",
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "resolve",
            self.file,
            "-a",
            self.all,
            "-l",
            self.listfiles,
            "-m",
            self.mark,
            "-u",
            self.unmark,
            "-t",
            self.tool,
            "-I",
            self.include,
            "-X",
            self.exclude
        )
    }
}

#[derive(Debug)]
pub enum Kind {
    Resolved,
    Unresolved,
}

#[derive(Debug)]
pub struct Resolve {
    pub kind: Kind,
    pub filename: String,
}

impl Client {
    pub fn resolve(&mut self, x: Arg) -> Result<Option<Vec<Resolve>>, HglibError> {
        let (data, _) = x.run(self)?;
        if x.listfiles {
            let mut res = Vec::new();
            for line in data.split(|c| *c == b'\n') {
                if line.len() >= 3 {
                    let filename = unsafe { line.get_unchecked(2..) };
                    let filename = String::from_utf8(filename.to_vec())?;
                    let c = unsafe { line.get_unchecked(0) };
                    let kind = match c {
                        b'R' => Kind::Resolved,
                        b'U' => Kind::Unresolved,
                        _ => {
                            return Err(HglibError::from("Invalid value"));
                        }
                    };
                    res.push(Resolve { kind, filename });
                }
            }
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub dest: &'a str,
    pub rev: &'a str,
    pub nodecode: bool,
    pub prefix: &'a str,
    pub typ: &'a str,
    pub subrepos: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            dest: "",
            rev: "",
            nodecode: false,
            prefix: "",
            typ: "",
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
            "archive",
            &[self.dest],
            "-r",
            self.rev,
            "--no-decode",
            self.nodecode,
            "-p",
            self.prefix,
            "-t",
            self.typ,
            "-S",
            self.subrepos,
            "-I",
            self.include,
            "-X",
            self.exclude
        )
    }
}

impl Client {
    pub fn archive(&mut self, x: Arg) -> Result<(), HglibError> {
        x.run(self)?;
        Ok(())
    }
}

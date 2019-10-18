// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub files: &'a [&'a str],
    pub rev: &'a [&'a str],
    pub all: bool,
    pub date: &'a str,
    pub nobackup: bool,
    pub dryrun: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            files: &[],
            rev: &[],
            all: false,
            date: "",
            nobackup: false,
            dryrun: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "revert",
            self.files,
            "-r",
            self.rev,
            "-a",
            self.all,
            "-d",
            self.date,
            "--no-backup",
            self.nobackup,
            "-n",
            self.dryrun,
            "-I",
            self.include,
            "-X",
            self.exclude
        )
    }
}

impl Client {
    pub fn revert(&mut self, x: Arg) -> Result<bool, HglibError> {
        HglibError::handle_err(x.run(self))
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub files: &'a [&'a str],
    pub dryrun: bool,
    pub subrepos: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            files: &[],
            dryrun: false,
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
            "add",
            self.files,
            "-n",
            self.dryrun,
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
    pub fn add(&mut self, x: Arg) -> Result<bool, HglibError> {
        let (_, code) = x.run(self)?;
        Ok(code == 0)
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub files: &'a [&'a str],
    pub similarity: Option<u32>,
    pub subrepos: bool,
    pub dryrun: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            files: &[],
            similarity: None,
            subrepos: false,
            dryrun: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        if let Some(x) = self.similarity {
            if x > 100 {
                return Err(HglibError::from(format!("Invalid similarity: {}", x)));
            }
        }
        runcommand!(
            client,
            "addremove",
            self.files,
            "-s",
            self.similarity,
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
    pub fn addremove(&mut self, x: Arg) -> Result<bool, HglibError> {
        HglibError::handle_err(x.run(self))
    }
}

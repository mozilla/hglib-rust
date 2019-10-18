// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub source: &'a [&'a str],
    pub dest: &'a str,
    pub after: bool,
    pub force: bool,
    pub dryrun: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            source: &[],
            dest: "",
            after: false,
            force: false,
            dryrun: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        let mut args = self.source.to_vec();
        args.push(self.dest);
        runcommand!(
            client,
            "copy",
            args,
            "-A",
            self.after,
            "-f",
            self.force,
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
    pub fn copy(&mut self, x: Arg) -> Result<bool, HglibError> {
        HglibError::handle_err(x.run(self))
    }
}

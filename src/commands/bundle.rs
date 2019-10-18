// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub file: &'a str,
    pub destrepo: &'a str,
    pub rev: &'a [&'a str],
    pub branch: &'a [&'a str],
    pub base: &'a [&'a str],
    pub all: bool,
    pub force: bool,
    pub typ: &'a str,
    pub ssh: &'a str,
    pub remotecmd: &'a str,
    pub insecure: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            file: "",
            destrepo: "",
            rev: &[],
            branch: &[],
            base: &[],
            all: false,
            force: false,
            typ: "",
            ssh: "",
            remotecmd: "",
            insecure: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "bundle",
            &[self.file, self.destrepo],
            "-f",
            self.force,
            "-r",
            self.rev,
            "-b",
            self.branch,
            "--base",
            self.base,
            "-a",
            self.all,
            "-t",
            self.typ,
            "-e",
            self.ssh,
            "--remotecmd",
            self.remotecmd,
            "-insecure",
            self.insecure
        )
    }
}

impl Client {
    pub fn bundle(&mut self, x: Arg) -> Result<bool, HglibError> {
        HglibError::handle_err(x.run(self))
    }
}

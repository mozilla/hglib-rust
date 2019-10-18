// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub dest: &'a str,
    pub rev: &'a [&'a str],
    pub force: bool,
    pub bookmark: &'a [&'a str],
    pub branch: &'a [&'a str],
    pub newbranch: &'a str,
    pub ssh: &'a str,
    pub remotecmd: &'a str,
    pub insecure: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            dest: "",
            rev: &[],
            force: false,
            bookmark: &[],
            branch: &[],
            newbranch: "",
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
            "push",
            &[self.dest],
            "-r",
            self.rev,
            "-f",
            self.force,
            "-B",
            self.bookmark,
            "-b",
            self.branch,
            "--new-branch",
            self.newbranch,
            "-e",
            self.ssh,
            "--remotecmd",
            self.remotecmd,
            "--insecure",
            self.insecure
        )
    }
}

impl Client {
    pub fn push(&mut self, x: Arg) -> Result<bool, HglibError> {
        HglibError::handle_err(x.run(self))
    }
}

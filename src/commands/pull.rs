// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub source: &'a str,
    pub rev: &'a [&'a str],
    pub update: bool,
    pub force: bool,
    pub bookmark: &'a [&'a str],
    pub branch: &'a [&'a str],
    pub ssh: &'a str,
    pub remotecmd: &'a str,
    pub insecure: bool,
    pub tool: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            source: "",
            rev: &[],
            update: false,
            force: false,
            bookmark: &[],
            branch: &[],
            ssh: "",
            remotecmd: "",
            insecure: false,
            tool: "",
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "pull",
            &[self.source],
            "-r",
            self.rev,
            "-u",
            self.update,
            "-f",
            self.force,
            "-B",
            self.bookmark,
            "-b",
            self.branch,
            "-e",
            self.ssh,
            "--remotecmd",
            self.remotecmd,
            "--insecure",
            self.insecure,
            "-t",
            self.tool
        )
    }
}

impl Client {
    pub fn pull(&mut self, x: Arg) -> Result<bool, HglibError> {
        HglibError::handle_err(x.run(self))
    }
}

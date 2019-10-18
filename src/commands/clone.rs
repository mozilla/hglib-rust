// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub source: &'a str,
    pub noupdate: bool,
    pub dest: &'a str,
    pub branch: &'a str,
    pub updaterev: &'a str,
    pub revrange: &'a str,
    pub pull: bool,
    pub stream: bool,
    pub ssh: &'a str,
    pub remotecmd: &'a str,
    pub insecure: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            source: ".",
            noupdate: false,
            dest: "",
            branch: "",
            updaterev: "",
            revrange: "",
            pull: false,
            stream: false,
            ssh: "",
            remotecmd: "",
            insecure: false,
        }
    }
}

impl<'a> Arg<'a> {
    pub fn run<T: Runner>(&self, client: &mut T) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "clone",
            &[self.source, self.dest],
            "-U",
            self.noupdate,
            "-b",
            self.branch,
            "-u",
            self.updaterev,
            "-r",
            self.revrange,
            "--pull",
            self.pull,
            "--stream",
            self.stream,
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
    pub fn clone(&mut self, x: Arg) -> Result<(), HglibError> {
        x.run(self)?;
        Ok(())
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub dest: &'a str,
    pub ssh: &'a str,
    pub remotecmd: &'a str,
    pub insecure: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            dest: "",
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
            "init",
            &[self.dest],
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
    pub fn init(&mut self, x: Arg) -> Result<(), HglibError> {
        x.run(self)?;
        Ok(())
    }
}

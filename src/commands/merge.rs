// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a str,
    pub force: bool,
    pub tool: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            rev: "",
            force: false,
            tool: "",
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "merge",
            &[""],
            "-r",
            self.rev,
            "-f",
            self.force,
            "-t",
            self.tool,
            "-y",
            true
        )
    }
}

impl Client {
    pub fn merge(&mut self, x: Arg) -> Result<(), HglibError> {
        x.run(self)?;
        Ok(())
    }
}

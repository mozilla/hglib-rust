// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a str,
    pub merge: bool,
    pub parent: &'a str,
    pub tool: &'a str,
    pub message: &'a str,
    pub logfile: &'a str,
    pub date: &'a str,
    pub user: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            rev: "",
            merge: false,
            parent: "",
            tool: "",
            message: "",
            logfile: "",
            date: "",
            user: "",
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "backout",
            &[self.rev],
            "--merge",
            self.merge,
            "--parent",
            self.parent,
            "-t",
            self.tool,
            "-m",
            self.message,
            "-l",
            self.logfile,
            "-d",
            self.date,
            "-u",
            self.user
        )
    }
}

impl Client {
    pub fn backout(&mut self, x: Arg) -> Result<(), HglibError> {
        x.run(self)?;
        Ok(())
    }
}

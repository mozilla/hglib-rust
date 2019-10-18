// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub names: &'a [&'a str],
    pub rev: &'a str,
    pub message: &'a str,
    pub local: bool,
    pub remove: bool,
    pub date: &'a str,
    pub user: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            names: &[],
            rev: "",
            message: "",
            local: false,
            remove: false,
            date: "",
            user: "",
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "tag",
            self.names,
            "-r",
            self.rev,
            "-m",
            self.message,
            "-l",
            self.local,
            "--remove",
            self.remove,
            "-d",
            self.date,
            "-u",
            self.user
        )
    }
}

impl Client {
    pub fn tag(&mut self, x: Arg) -> Result<(), HglibError> {
        x.run(self)?;
        Ok(())
    }
}

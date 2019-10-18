// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub revs: &'a [&'a str],
    pub output: &'a str,
    pub switchparent: bool,
    pub text: bool,
    pub git: bool,
    pub nodates: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            revs: &[],
            output: "",
            switchparent: false,
            text: false,
            git: false,
            nodates: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "export",
            self.revs,
            "-o",
            self.output,
            "--switch-parent",
            self.switchparent,
            "-a",
            self.text,
            "-g",
            self.git,
            "--nodates",
            self.nodates
        )
    }
}

impl Client {
    pub fn export(&mut self, x: Arg) -> Result<Option<Vec<u8>>, HglibError> {
        Ok(if x.output.is_empty() {
            let (data, _) = x.run(self)?;
            Some(data)
        } else {
            None
        })
    }
}

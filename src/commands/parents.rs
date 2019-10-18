// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use super::common;
use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a str,
    pub file: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self { rev: "", file: "" }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "parents",
            &[self.file],
            "-r",
            self.rev,
            "--template",
            common::CHANGESETS_TEMPLATE
        )
    }
}

impl Client {
    pub fn parents(&mut self, x: Arg) -> Result<Vec<common::Revision>, HglibError> {
        let (data, _) = x.run(self)?;
        if data.is_empty() {
            Ok(Vec::new())
        } else {
            common::parserevs(data)
        }
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use super::common;
use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a [&'a str],
    pub startrev: &'a [&'a str],
    pub topological: bool,
    pub closed: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            rev: &[],
            startrev: &[],
            topological: false,
            closed: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "heads",
            self.rev,
            "-r",
            self.startrev,
            "-t",
            self.topological,
            "-c",
            self.closed,
            "--template",
            common::CHANGESETS_TEMPLATE
        )
    }
}

impl Client {
    pub fn heads(&mut self, x: Arg) -> Result<Vec<common::Revision>, HglibError> {
        match x.run(self) {
            Ok((data, _)) => common::parserevs(data),
            Err(err) => {
                if err.code == 1 {
                    Ok(Vec::new())
                } else {
                    Err(err)
                }
            }
        }
    }
}

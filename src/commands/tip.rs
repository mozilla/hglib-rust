// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use super::common;
use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg {}

impl Default for Arg {
    fn default() -> Self {
        Self {}
    }
}

impl Arg {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "tip",
            &[""],
            "--template",
            common::CHANGESETS_TEMPLATE
        )
    }
}

impl Client {
    pub fn tip(&mut self, x: Arg) -> Result<common::Revision, HglibError> {
        let (data, _) = x.run(self)?;
        let mut rev = common::parserevs(data)?;
        let rev = rev.pop().unwrap();

        Ok(rev)
    }
}

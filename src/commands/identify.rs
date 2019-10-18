// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a str,
    pub source: &'a str,
    pub num: bool,
    pub id: bool,
    pub branch: bool,
    pub tags: bool,
    pub bookmarks: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            rev: "",
            source: "",
            num: false,
            id: false,
            branch: false,
            tags: false,
            bookmarks: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "identify",
            &[self.source],
            "-r",
            self.rev,
            "-n",
            self.num,
            "-i",
            self.id,
            "-b",
            self.branch,
            "-t",
            self.tags,
            "-B",
            self.bookmarks
        )
    }
}

impl Client {
    pub fn identify(&mut self, x: Arg) -> Result<Vec<u8>, HglibError> {
        let (data, _) = x.run(self)?;
        Ok(data)
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub files: &'a [&'a str],
    pub rev: &'a [&'a str],
    pub decode: bool,
    pub output: &'a str,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            files: &[],
            rev: &[],
            decode: false,
            output: "",
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "cat",
            self.files,
            "-r",
            self.rev,
            "-o",
            self.output,
            "-I",
            self.include,
            "-X",
            self.exclude
        )
    }
}

impl Client {
    pub fn cat(&mut self, x: Arg) -> Result<Option<Vec<u8>>, HglibError> {
        let (data, _) = x.run(self)?;
        Ok(if x.output.is_empty() {
            Some(data)
        } else {
            None
        })
    }
}

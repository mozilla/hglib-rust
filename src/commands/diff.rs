// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub files: &'a [&'a str],
    pub revs: &'a [&'a str],
    pub change: &'a str,
    pub text: bool,
    pub git: bool,
    pub nodates: bool,
    pub showfunction: bool,
    pub reverse: bool,
    pub ignoreallspace: bool,
    pub ignorespacechange: bool,
    pub ignoreblanklines: bool,
    pub unified: Option<u32>,
    pub stat: bool,
    pub subrepos: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            files: &[],
            revs: &[],
            change: "",
            text: false,
            git: false,
            nodates: false,
            showfunction: false,
            reverse: false,
            ignoreallspace: false,
            ignorespacechange: false,
            ignoreblanklines: false,
            unified: None,
            stat: false,
            subrepos: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "diff",
            self.files,
            "-r",
            self.revs,
            "-c",
            self.change,
            "-a",
            self.text,
            "-g",
            self.git,
            "--nodates",
            self.nodates,
            "-p",
            self.showfunction,
            "--reverse",
            self.reverse,
            "-w",
            self.ignoreallspace,
            "-b",
            self.ignorespacechange,
            "-B",
            self.ignoreblanklines,
            "-U",
            self.unified,
            "--stat",
            self.stat,
            "-S",
            self.subrepos,
            "-I",
            self.include,
            "-X",
            self.exclude
        )
    }
}

impl Client {
    pub fn diff(&mut self, x: Arg) -> Result<Vec<u8>, HglibError> {
        if !x.change.is_empty() && !x.revs.is_empty() {
            return Err(HglibError::from("Cannot specify both change and rev"));
        }
        let (data, _) = x.run(self)?;
        Ok(data)
    }
}

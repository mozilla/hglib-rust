// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use super::common;
use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub revrange: &'a [&'a str],
    pub files: &'a [&'a str],
    pub follow: bool,
    pub followfirst: bool,
    pub date: &'a str,
    pub copies: bool,
    pub keyword: &'a [&'a str],
    pub removed: bool,
    pub onlymerges: bool,
    pub user: &'a [&'a str],
    pub branch: &'a [&'a str],
    pub prune: &'a [&'a str],
    pub hidden: bool,
    pub limit: Option<u32>,
    pub nomerges: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            revrange: &[],
            files: &[],
            follow: false,
            followfirst: false,
            date: "",
            copies: false,
            keyword: &[],
            removed: false,
            onlymerges: false,
            user: &[],
            branch: &[],
            prune: &[],
            hidden: false,
            limit: None,
            nomerges: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "log",
            self.files,
            "--template",
            common::CHANGESETS_TEMPLATE,
            "-r",
            self.revrange,
            "-f",
            self.follow,
            "--follow-first",
            self.followfirst,
            "-d",
            self.date,
            "-C",
            self.copies,
            "-k",
            self.keyword,
            "--removed",
            self.removed,
            "-m",
            self.onlymerges,
            "-u",
            self.user,
            "-b",
            self.branch,
            "-P",
            self.prune,
            "--hidden",
            self.hidden,
            "-l",
            self.limit,
            "-M",
            self.nomerges,
            "-I",
            self.include,
            "-X",
            self.exclude
        )
    }
}

impl Client {
    pub fn log<'a>(&mut self, x: Arg<'a>) -> Result<Vec<common::Revision>, HglibError> {
        let (data, _) = x.run(self)?;
        common::parserevs(data)
    }
}

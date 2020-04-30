// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use super::common;
use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub revrange: &'a [&'a str],
    pub path: &'a str,
    pub force: bool,
    pub newest: bool,
    pub bookmarks: bool,
    pub branch: &'a str,
    pub limit: Option<u32>,
    pub nomerges: bool,
    pub subrepos: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            revrange: &[],
            path: "",
            force: false,
            newest: false,
            bookmarks: false,
            branch: "",
            limit: None,
            nomerges: false,
            subrepos: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "outgoing",
            &[self.path],
            "--template",
            common::CHANGESETS_TEMPLATE,
            "-r",
            self.revrange,
            "-f",
            self.force,
            "-n",
            self.newest,
            "-B",
            self.bookmarks,
            "-b",
            self.branch,
            "-l",
            self.limit,
            "-M",
            self.nomerges,
            "-S",
            self.subrepos
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Bookmark {
    pub bookmark: String,
    pub revision: String,
}

#[derive(Debug, PartialEq)]
pub enum Outgoing {
    Revisions(Vec<common::Revision>),
    Bookmarks(Vec<Bookmark>),
    Empty,
}

impl Client {
    pub fn outgoing(&mut self, x: Arg) -> Result<Outgoing, HglibError> {
        let data = match x.run(self) {
            Ok(ret) => ret.0,
            Err(e) => {
                if e.code == 1 {
                    return Ok(Outgoing::Empty);
                } else {
                    return Err(e);
                }
            }
        };

        let data = common::eatlines(&data, 2);
        if x.bookmarks {
            let mut res = Vec::new();
            let mut tmp: &[u8] = &[];
            let mut odd = false;
            for chunk in data
                .split(|c| *c == b' ' || *c == b'\n')
                .filter(|&c| !c.is_empty())
            {
                if odd {
                    res.push(Bookmark {
                        bookmark: String::from_utf8(tmp.to_vec())?,
                        revision: String::from_utf8(chunk.to_vec())?,
                    });
                    odd = false;
                } else {
                    tmp = chunk;
                    odd = true;
                }
            }
            Ok(Outgoing::Bookmarks(res))
        } else {
            Ok(Outgoing::Revisions(common::parserevs(data.to_vec())?))
        }
    }
}

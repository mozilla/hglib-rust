// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub rev: &'a str,
    pub clean: bool,
    pub check: bool,
    pub date: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            rev: "",
            clean: false,
            check: false,
            date: "",
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "update",
            &[""],
            "-r",
            self.rev,
            "-C",
            self.clean,
            "-c",
            self.check,
            "-d",
            self.date
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Update {
    pub updated: u32,
    pub merged: u32,
    pub removed: u32,
    pub unresolved: u32,
}

impl Client {
    pub fn update(&mut self, x: Arg) -> Result<Update, HglibError> {
        let data = match x.run(self) {
            Ok((data, _)) => data,
            Err(err) => {
                if err.code == 1 && err.out.is_some() {
                    err.out.unwrap()
                } else {
                    return Err(err);
                }
            }
        };

        let iter = &mut data.iter();
        let n = iter.find(|x| b'0' <= **x && **x <= b'9').unwrap();
        let n = u32::from(n - b'0');

        let updated = iter
            .take_while(|x| **x != b' ')
            .fold(n, |r, x| r * 10 + u32::from(*x - b'0'));
        iter.find(|x| **x == b',').unwrap();
        iter.next();

        let merged = iter
            .take_while(|x| **x != b' ')
            .fold(0, |r, x| r * 10 + u32::from(*x - b'0'));
        iter.find(|x| **x == b',').unwrap();
        iter.next();

        let removed = iter
            .take_while(|x| **x != b' ')
            .fold(0, |r, x| r * 10 + u32::from(*x - b'0'));
        iter.find(|x| **x == b',').unwrap();
        iter.next();

        let unresolved = iter
            .take_while(|x| **x != b' ')
            .fold(0, |r, x| r * 10 + u32::from(*x - b'0'));

        Ok(Update {
            updated,
            merged,
            removed,
            unresolved,
        })
    }
}

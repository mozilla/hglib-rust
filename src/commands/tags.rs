// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

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
        runcommand!(client, "tags", &[""], "-v", true)
    }
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub rev: u64,
    pub node: String,
    pub islocal: bool,
}

impl Client {
    pub fn tags(&mut self, x: Arg) -> Result<Vec<Tag>, HglibError> {
        let (data, _) = x.run(self)?;
        let mut tags = Vec::new();
        for line in data.split(|x| *x == b'\n').filter(|x| !x.is_empty()) {
            let islocal = line.ends_with(b" local");
            let line = if islocal {
                unsafe { line.get_unchecked(..line.len() - 6) }
            } else {
                line
            };
            let mut iter = line.split(|x| *x == b' ').filter(|x| !x.is_empty());
            let name = String::from_utf8(iter.next().unwrap().to_vec())?;
            let rev_node = iter.next().unwrap();
            let iter = &mut rev_node.iter();
            let rev = iter
                .take_while(|x| **x != b':')
                .fold(0, |r, x| r * 10 + u64::from(*x - b'0'));
            let node = iter.as_slice();
            let node = String::from_utf8(node.to_vec())?;
            tags.push(Tag {
                name,
                rev,
                node,
                islocal,
            });
        }
        Ok(tags)
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg {
    pub active: bool,
    pub closed: bool,
}

impl Default for Arg {
    fn default() -> Self {
        Self {
            active: false,
            closed: false,
        }
    }
}

impl Arg {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "branches",
            &[""],
            "-a",
            self.active,
            "-c",
            self.closed
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Branch {
    pub name: String,
    pub rev: u64,
    pub node: String,
}

impl Client {
    pub fn branches(&mut self, x: Arg) -> Result<Vec<Branch>, HglibError> {
        let (data, _) = x.run(self)?;
        let mut branches = Vec::new();
        for line in data.split(|x| *x == b'\n').filter(|x| !x.is_empty()) {
            let mut iter = line.split(|x| *x == b' ').filter(|x| !x.is_empty());
            let name = iter.next().unwrap();
            let name = String::from_utf8(name.to_vec())?;
            let rev_node = iter.next().unwrap();
            let iter = &mut rev_node.iter();
            let rev = iter
                .take_while(|x| **x != b':')
                .fold(0, |r, x| r * 10 + u64::from(*x - b'0'));
            let node = iter.as_slice();
            let node = String::from_utf8(node.to_vec())?;

            branches.push(Branch { name, rev, node });
        }
        Ok(branches)
    }
}

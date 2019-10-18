// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub revs: &'a [&'a str],
    pub secret: bool,
    pub draft: bool,
    pub public: bool,
    pub force: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            revs: &[],
            secret: false,
            draft: false,
            public: false,
            force: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "phase",
            self.revs,
            "--secret",
            self.secret,
            "--draft",
            self.draft,
            "--public",
            self.public,
            "--force",
            self.force
        )
    }
}

#[derive(Debug)]
pub struct Phase {
    pub num: u64,
    pub phase: String,
}

impl Client {
    pub fn phase(&mut self, x: Arg) -> Result<Option<Vec<Phase>>, HglibError> {
        let (data, _) = x.run(self)?;
        if x.draft || x.public || x.secret {
            Ok(None)
        } else {
            let mut phases = Vec::new();
            let mut num: Option<u64> = None;
            for elem in data
                .split(|x| *x == b'\n' || *x == b' ' || *x == b':')
                .filter(|x| !x.is_empty())
            {
                num = if let Some(num) = num {
                    let phase = String::from_utf8(elem.to_vec())?;
                    phases.push(Phase { num, phase });
                    None
                } else {
                    let num = elem.iter().fold(0, |r, x| r * 10 + u64::from(*x - b'0'));
                    Some(num)
                }
            }
            Ok(Some(phases))
        }
    }
}

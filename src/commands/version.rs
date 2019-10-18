// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use regex::bytes::Regex;

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
        runcommand!(client, "version", &[""], "-q", true)
    }
}

#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub micro: Option<u32>,
    pub build_info: Option<String>,
}

impl Client {
    pub fn version(&mut self, x: Arg) -> Result<Version, HglibError> {
        let (data, _) = x.run(self)?;
        let pat = Regex::new(r".*?(\d+)\.(\d+)\.?(\d+)?(\+[0-9a-f-]+)?").unwrap();
        let cap = pat.captures_iter(&data).next().unwrap();

        let micro = if let Some(buf) = cap.get(3) {
            Some(
                buf.as_bytes()
                    .iter()
                    .fold(0, |r, x| r * 10 + u32::from(*x - b'0')),
            )
        } else {
            None
        };

        let build_info = if let Some(buf) = cap.get(4) {
            Some(String::from_utf8(buf.as_bytes().to_vec())?)
        } else {
            None
        };

        Ok(Version {
            major: cap[1].iter().fold(0, |r, x| r * 10 + u32::from(*x - b'0')),
            minor: cap[2].iter().fold(0, |r, x| r * 10 + u32::from(*x - b'0')),
            micro,
            build_info,
        })
    }
}

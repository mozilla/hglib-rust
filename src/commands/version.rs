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

#[derive(Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub micro: Option<u32>,
    pub build_info: Option<String>,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for c in &[self.major.cmp(&other.major), self.minor.cmp(&other.minor)] {
            if c != &std::cmp::Ordering::Equal {
                return *c;
            }
        }

        match (self.micro, other.micro) {
            (None, None) => return std::cmp::Ordering::Equal,
            (Some(x), None) => return x.cmp(&0),
            (None, Some(y)) => return 0.cmp(&y),
            (Some(x), Some(y)) => {
                return x.cmp(&y);
            }
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Version {
    pub fn from_tuple(v: &(u32, u32, Option<u32>)) -> Version {
        return Version {
            major: v.0,
            minor: v.1,
            micro: v.2,
            build_info: None,
        };
    }
}

impl PartialEq<(u32, u32, Option<u32>)> for Version {
    fn eq(&self, other: &(u32, u32, Option<u32>)) -> bool {
        return self == &Version::from_tuple(other);
    }
}

impl PartialOrd<(u32, u32, Option<u32>)> for Version {
    fn partial_cmp(&self, other: &(u32, u32, Option<u32>)) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(&Version::from_tuple(other)));
    }
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

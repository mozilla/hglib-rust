// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::prelude::*;
use chrono::NaiveDateTime;

use crate::client::HglibError;

#[derive(Debug, PartialEq)]
pub struct Revision {
    pub rev: u64,
    pub node: String,
    pub tags: String,
    pub branch: String,
    pub author: String,
    pub desc: String,
    pub date: DateTime<Utc>,
}

pub const CHANGESETS_TEMPLATE: &str =
    "{rev}\\0{node}\\0{tags}\\0{branch}\\0{author}\\0{desc}\\0{date}\\0";

pub fn parserevs(data: Vec<u8>) -> Result<Vec<Revision>, HglibError> {
    let mut count = 0;
    let mut parts: Vec<&[u8]> = vec![&[]; 6];
    let mut res = Vec::new();
    let mut rev: u64 = 0;

    for buf in data.split(|x| *x == b'\0') {
        if count == 0 {
            rev = buf.iter().fold(0, |r, x| r * 10 + u64::from(*x - b'0'));
            count += 1;
        } else if count == 6 {
            count = 0;
            let timestamp = buf
                .iter()
                .take_while(|x| **x != b'.')
                .fold(0, |r, x| r * 10 + i64::from(*x - b'0'));
            res.push(Revision {
                rev,
                node: String::from_utf8(parts[1].to_vec())?,
                tags: String::from_utf8(parts[2].to_vec())?,
                branch: String::from_utf8(parts[3].to_vec())?,
                author: String::from_utf8(parts[4].to_vec())?,
                desc: String::from_utf8(parts[5].to_vec())?,
                date: DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc),
            });
            rev = 0;
        } else {
            parts[count] = &buf;
            count += 1;
        }
    }
    Ok(res)
}

pub fn eatlines(buf: &[u8], n: u32) -> &[u8] {
    let mut iter = buf.iter();
    let mut count = 0;
    while let Some(c) = iter.next() {
        if *c == b'\n' {
            if count == n - 1 {
                return iter.as_slice();
            } else {
                count += 1;
            }
        }
    }
    &[]
}

pub fn handle_err(x: Result<(Vec<u8>, i32), HglibError>) -> Result<bool, HglibError> {
    match x {
        Ok((_, code)) => Ok(code == 0),
        Err(err) => {
            if err.code == 0 {
                Ok(true)
            } else if err.code == 1 {
                Ok(false)
            } else {
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_eatlines() {
        let s = "rust
is
an
amazing
programming
language";
        let s = s.as_bytes();
        let r = eatlines(s, 1);
        assert!(&r[..2] == b"is");

        let r = eatlines(s, 2);
        assert!(&r[..2] == b"an");

        let r = eatlines(s, 4);
        assert!(&r[..2] == b"pr");
    }
}

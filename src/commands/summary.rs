// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg {
    pub remote: bool,
}

impl Default for Arg {
    fn default() -> Self {
        Self { remote: false }
    }
}

impl Arg {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(client, "summary", &[""], "--remote", self.remote)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Revision {
    pub rev: i64,
    pub node: String,
    pub tags: String,
    pub message: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct Remote {
    pub outgoing: u64,
    pub incoming: u64,
    pub outgoing_bookmarks: u64,
    pub incoming_bookmarks: u64,
}

#[derive(Debug, Default, PartialEq)]
pub struct Mq {
    pub applied: u64,
    pub unapplied: u64,
}

#[derive(Debug, Default, PartialEq)]
pub struct Summary {
    pub parent: Vec<Revision>,
    pub branch: String,
    pub commit: bool,
    pub update: u32,
    pub remote: Option<Remote>,
    pub mq: Mq,
    pub others: HashMap<String, String>,
}

impl Client {
    pub fn summary(&mut self, x: Arg) -> Result<Summary, HglibError> {
        let (data, _) = x.run(self)?;

        let mut summary = Summary::default();
        let mut wait_message = false;

        for line in data.split(|x| *x == b'\n').filter(|x| !x.is_empty()) {
            if wait_message {
                let message = String::from_utf8(line[1..].to_vec()).unwrap();
                summary.parent.last_mut().unwrap().message = message;
                wait_message = false;
                continue;
            }

            let i = line.iter().position(|x| *x == b':').unwrap();
            let name = &line[..i];
            let value = &line[i + 2..];

            match name {
                b"parent" => {
                    let iter = &mut value.iter();
                    let first = iter.next().unwrap();
                    let (neg, init) = if *first == b'-' {
                        (true, 0)
                    } else {
                        (false, i64::from(*first - b'0'))
                    };

                    let rev = iter
                        .take_while(|x| **x != b':')
                        .fold(init, |r, x| r * 10 + i64::from(*x - b'0'));
                    let rev = if neg { -rev } else { rev };

                    let node = iter
                        .take_while(|x| **x != b' ')
                        .map(|x| *x as char)
                        .collect::<String>();

                    let mut tags = iter.map(|x| *x as char).collect::<String>();
                    if !tags.is_empty() {
                        let empty = " (empty repository)";
                        if tags.ends_with(empty) {
                            tags.replace_range(tags.len() - empty.len().., "")
                        }
                    }

                    summary.parent.push(Revision {
                        rev,
                        node,
                        tags,
                        message: "".to_string(),
                    });

                    wait_message = rev != -1;
                }
                b"branch" => {
                    summary.branch = String::from_utf8(value.to_vec()).unwrap();
                }
                b"commit" => {
                    let clean = b"(clean)";
                    summary.commit = line.windows(clean.len()).any(|w| w == clean);
                }
                b"update" => {
                    summary.update = if value == b"(current)" {
                        0
                    } else {
                        value
                            .iter()
                            .take_while(|x| **x != b' ')
                            .fold(0, |r, x| r * 10 + u32::from(*x - b'0'))
                    };
                }
                b"remote" => {
                    if !x.remote {
                        continue;
                    }

                    summary.remote = Some(if value == b"(synced)" {
                        Remote::default()
                    } else {
                        let mut rem = Remote::default();
                        let iter = &mut value.iter();
                        loop {
                            let n = iter
                                .take_while(|x| **x != b' ')
                                .fold(0, |r, x| r * 10 + u64::from(*x - b'0'));

                            let typ: Vec<_> = iter.take_while(|x| **x != b',').copied().collect();
                            match typ.as_slice() {
                                b"outgoing" => {
                                    rem.outgoing = n;
                                }
                                b"outgoing bookmarks" => {
                                    rem.outgoing_bookmarks = n;
                                }
                                b"incoming bookmarks" => {
                                    rem.incoming_bookmarks = n;
                                }
                                _ => {
                                    if typ.ends_with(b"incoming") {
                                        rem.incoming = n;
                                    }
                                }
                            }

                            if iter.next().is_none() {
                                break;
                            }
                        }
                        rem
                    });
                }
                b"mq" => {}
                _ => {
                    let name = String::from_utf8(name.to_vec()).unwrap();
                    let value = String::from_utf8(value.to_vec()).unwrap();

                    summary.others.insert(name, value);
                }
            }
        }

        Ok(summary)
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::prelude::*;

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub pattern: &'a str,
    pub files: &'a [&'a str],
    pub all: bool,
    pub text: bool,
    pub follow: bool,
    pub ignorecase: bool,
    pub fileswithmatches: bool,
    pub line: bool,
    pub user: bool,
    pub date: bool,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            pattern: "",
            files: &[],
            all: false,
            text: false,
            follow: false,
            ignorecase: false,
            fileswithmatches: false,
            line: false,
            user: false,
            date: false,
            include: &[],
            exclude: &[],
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        let mut args = self.files.to_vec();
        args.insert(0, self.pattern);
        runcommand!(client,
                    true,
                    "grep", args,
                    "--all", self.all,
                    "-a", self.text,
                    "-f", self.follow,
                    "-i", self.ignorecase,
                    "-l", self.fileswithmatches,
                    "-n", self.line,
                    "-u", self.user,
                    "-d", self.date,
                    "-I", self.include,
                    "-X", self.exclude,
                    "--print0", true)
    }
}

#[derive(Debug)]
pub struct GrepRes {
    pub filename: String,
    pub rev: u64,
    pub line: Option<u32>,
    pub match_status: Option<String>,
    pub user: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub matched: Option<String>,
}

fn get_index(x: &Arg, n: usize, fieldcount: usize) -> usize {
    /*
    (0, 1, 2) => (0, 1, 6);          no options
    (0, 1, 2, 3) => (0, 1, 2, 6)     line
                    (0, 1, 3, 6)     all
                    (0, 1, 4, 6)     user
                    (0, 1, 5, 6)     date

    (0, 1, 2, 3, 4) => (0, 1, 2, 3, 6)     line + all
                       (0, 1, 2, 4, 6)     line + user
                       (0, 1, 2, 5, 6)     line + date
                       (0, 1, 3, 4, 6)     all + user
                       (0, 1, 3, 5, 6)     all + date
                       (0, 1, 4, 5, 6)     user + date

    (0, 1, 2, 3, 4, 5) => (0, 1, 2, 3, 4, 6)     line + all + user
                          (0, 1, 2, 3, 5, 6)     line + all + date
                          (0, 1, 2, 4, 5, 6)     line + user + date
                          (0, 1, 3, 4, 5, 6)     all + user + date

    (0, 1, 2, 3, 4, 5, 6) => (0, 1, 2, 3, 4, 5, 6)     line + all + user + date
     */
    
    let n = n % fieldcount;
    if n == 0 || n == 1 {
        return n;
    }
    if n + 1 == fieldcount {
        return 6;
    }

    if fieldcount == 4 {
        if x.line {
            return 2;
        }
        if x.all {
            return 3;
        }
        if x.user {
            return 4;
        }
        return 5;
    }

    if fieldcount == 5 {
        if x.line {
            return 2;
        }
        if x.all {
            return 3;
        }
        if x.user {
            return 4;
        }
        if x.date {
            return 5;
        }
    }

    
    if x.line {
        if n == 2 {
            return 2;
        }
    }   
    if x.all {
        fieldcount += 1;
    }
    if x.user {
        fieldcount += 1;
    }
    if x.date {
        fieldcount += 1;
    }
    if x.fileswithmatches {
        fieldcount -= 1;
    }
}

impl Client {
    pub fn grep(&mut self, x: Arg) -> Result<Vec<GrepRes>, HglibError> {
        let (data, code) = x.run(self)?;
        let mut res = Vec::new();
        let mut fieldcount: usize = 3;
        if x.user {
            fieldcount += 1;
        }
        if x.date {
            fieldcount += 1;
        }
        if x.line {
            fieldcount += 1;
        }
        if x.all {
            fieldcount += 1;
        }
        if x.fileswithmatches {
            fieldcount -= 1;
        }
        
        for (n, element) in data.split(|x| *x == b'\0').enumerate() {
            let n = n % fieldcount;
            match n {
                0 => {
                    let filename = String::from_utf8(element.to_vec())?;
                    res.push(GrepRes {
                        filename,
                        rev: 0,
                        line: None,
                        match_status: None,
                        user: None,
                        date: None,
                        matched: None,
                    });
                }
                1 => {
                    res.last_mut().unwrap().rev = element.iter().fold(0, |r, x| r * 10 + (*x - b'0') as u64);
                }
                2 => {
                    debug_vec!(element);
                    res.last_mut().unwrap().line = Some(element.iter().fold(0, |r, x| r * 10 + (*x - b'0') as u32));
                }
                3 => {
                    res.last_mut().unwrap().match_status = Some(String::from_utf8(element.to_vec())?);
                }
                4 => {
                    res.last_mut().unwrap().user = Some(String::from_utf8(element.to_vec())?);
                }
                5 => {
                    let sdate = std::str::from_utf8(element)?;
                    res.last_mut().unwrap().date = Some(DateTime::parse_from_rfc2822(sdate)?.with_timezone(&Utc));
                }
                6 => {
                    res.last_mut().unwrap().matched = Some(String::from_utf8(element.to_vec())?);
                }
                _ => {}
            }
        }
        Ok(res)
    }
}

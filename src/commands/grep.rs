// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::version;
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
        runcommand!(
            client,
            "grep",
            args,
            "--all",
            self.all,
            "-a",
            self.text,
            "-f",
            self.follow,
            "-i",
            self.ignorecase,
            "-l",
            self.fileswithmatches,
            "-n",
            self.line,
            "-u",
            self.user,
            "-d",
            self.date,
            "-I",
            self.include,
            "-X",
            self.exclude,
            "--print0",
            true
        )
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(Clone))]
pub struct GrepRes {
    pub filename: String,
    pub rev: Option<u64>,
    pub line: Option<u32>,
    pub match_status: Option<String>,
    pub user: Option<String>,
    //TODO: convert string to datetime
    // pub date: Option<DateTime<Utc>>,
    pub date: Option<String>,
    pub matched: Option<String>,
}

#[derive(Debug, PartialEq)]
enum FieldType {
    Filename,
    Rev,
    Line,
    MatchStatus,
    User,
    Date,
    Matched,
}

impl Client {
    fn get_field_types(&mut self, x: &Arg) -> Result<Vec<FieldType>, HglibError> {
        let mut field_types = vec![FieldType::Filename];

        if x.all || self.version(version::Arg {})? < (5, 2, None) {
            field_types.push(FieldType::Rev);
        }
        if x.line {
            field_types.push(FieldType::Line);
        }
        if x.all {
            field_types.push(FieldType::MatchStatus);
        }
        if x.user {
            field_types.push(FieldType::User);
        }
        if x.date {
            field_types.push(FieldType::Date);
        }
        if !x.fileswithmatches {
            field_types.push(FieldType::Matched);
        }

        return Ok(field_types);
    }

    pub fn grep(&mut self, x: Arg) -> Result<Vec<GrepRes>, HglibError> {
        let mut res = Vec::new();

        let data = match x.run(self) {
            Ok(ret) => ret.0,
            Err(e) => {
                if e.code == 1 {
                    return Ok(res);
                } else {
                    return Err(e);
                }
            }
        };

        let field_types = self.get_field_types(&x)?;
        for (n, element) in data.split(|x| *x == b'\0').enumerate() {
            match field_types[n % field_types.len()] {
                FieldType::Filename => {
                    let filename = String::from_utf8(element.to_vec())?;
                    res.push(GrepRes {
                        filename,
                        rev: None,
                        line: None,
                        match_status: None,
                        user: None,
                        date: None,
                        matched: None,
                    });
                }
                FieldType::Rev => {
                    res.last_mut().unwrap().rev =
                        Some(element.iter().fold(0, |r, x| r * 10 + (*x - b'0') as u64));
                }
                FieldType::Line => {
                    res.last_mut().unwrap().line =
                        Some(element.iter().fold(0, |r, x| r * 10 + (*x - b'0') as u32));
                }
                FieldType::MatchStatus => {
                    res.last_mut().unwrap().match_status =
                        Some(String::from_utf8(element.to_vec())?);
                }
                FieldType::User => {
                    res.last_mut().unwrap().user = Some(String::from_utf8(element.to_vec())?);
                }
                FieldType::Date => {
                    let sdate = std::str::from_utf8(element)?;
                    res.last_mut().unwrap().date = Some(sdate.to_string());
                }
                FieldType::Matched => {
                    res.last_mut().unwrap().matched = Some(String::from_utf8(element.to_vec())?);
                }
            }
        }
        res.pop();
        Ok(res)
    }
}

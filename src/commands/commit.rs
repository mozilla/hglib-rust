// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub message: &'a str,
    pub logfile: &'a str,
    pub addremove: bool,
    pub closebranch: bool,
    pub date: &'a str,
    pub user: &'a str,
    pub include: &'a [&'a str],
    pub exclude: &'a [&'a str],
    pub amend: bool,
    pub subrepos: bool,
    pub secret: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            message: "",
            logfile: "",
            addremove: false,
            closebranch: false,
            date: "",
            user: "",
            include: &[],
            exclude: &[],
            amend: false,
            subrepos: false,
            secret: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "commit",
            &[""],
            "--debug",
            true,
            "-m",
            self.message,
            "-A",
            self.addremove,
            "--close-branch",
            self.closebranch,
            "-d",
            self.date,
            "-u",
            self.user,
            "-l",
            self.logfile,
            "-I",
            self.include,
            "-X",
            self.exclude,
            "--amend",
            self.amend,
            "-S",
            self.subrepos,
            "--secret",
            self.secret
        )
    }
}

#[derive(Debug)]
pub struct Commit {
    pub rev: u64,
    pub node: String,
}

impl Client {
    pub fn commit(&mut self, x: Arg) -> Result<Commit, HglibError> {
        let mut x = x;
        let message = if x.amend && x.message.is_empty() && x.logfile.is_empty() {
            runcommand!(
                self,
                "log",
                &[""],
                "-r",
                ".",
                "-l",
                1 as u32,
                "--template",
                "{desc}"
            )?
            .0
        } else {
            vec![0; 0]
        };
        if !message.is_empty() {
            x.message = std::str::from_utf8(&message)?;
        }

        if x.message.is_empty() && x.logfile.is_empty() && !x.amend {
            return Err(HglibError::from(
                "Must provide at least a message or a logfile",
            ));
        } else if !x.message.is_empty() && !x.logfile.is_empty() {
            return Err(HglibError::from(
                "Cannot specify both a message and a logfile",
            ));
        }

        let (data, _) = x.run(self)?;
        let committed_changeset = b"committed changeset ";
        for line in data.split(|x| *x == b'\n') {
            if line.starts_with(committed_changeset) {
                let rev_node = unsafe { line.get_unchecked(committed_changeset.len()..) };
                let iter = &mut rev_node.iter();
                let rev = iter
                    .take_while(|x| **x != b':')
                    .fold(0, |r, x| r * 10 + u64::from(*x - b'0'));
                let node = iter.as_slice();
                let node = String::from_utf8(node.to_vec())?;
                return Ok(Commit { rev, node });
            }
        }
        let s = std::str::from_utf8(&data).unwrap();
        Err(HglibError::from(format!(
            "Revision and node not found in hg output: {}",
            s
        )))
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Prompt, Runner};
use crate::{runcommand, runcommand_with_prompt, MkArg};

pub struct Arg<'a> {
    pub patches: &'a [&'a str],
    pub strip: Option<u32>,
    pub force: bool,
    pub nocommit: bool,
    pub bypass: bool,
    pub exact: bool,
    pub importbranch: bool,
    pub message: &'a str,
    pub date: &'a str,
    pub user: &'a str,
    pub similarity: Option<u8>,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            patches: &[],
            strip: None,
            force: false,
            nocommit: false,
            bypass: false,
            exact: false,
            importbranch: false,
            message: "",
            date: "",
            user: "",
            similarity: None,
        }
    }
}

struct ImportPrompt<'a> {
    patch: &'a [u8],
    pos: usize,
}

impl<'a> ImportPrompt<'a> {
    fn new(patch: &'a str) -> Self {
        Self {
            patch: patch.as_bytes(),
            pos: 0,
        }
    }
}

impl<'a> Prompt for ImportPrompt<'a> {
    fn call(&mut self, size: usize) -> &[u8] {
        if self.pos < self.patch.len() {
            let end = self.patch.len().min(self.pos + size);
            let buf = &self.patch[self.pos..end];
            self.pos = end;

            buf
        } else {
            &[]
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        if self.patches.len() == 1 && self.patches[0].as_bytes().iter().any(|c| *c == b'\n') {
            let prompt = ImportPrompt::new(self.patches[0]);
            runcommand_with_prompt!(
                client,
                "import",
                prompt,
                &["-"],
                "--strip",
                self.strip,
                "--force",
                self.force,
                "--no-commit",
                self.nocommit,
                "--bypass",
                self.bypass,
                "--exact",
                self.exact,
                "--import-branch",
                self.importbranch,
                "--message",
                self.message,
                "--date",
                self.date,
                "--user",
                self.user,
                "--similarity",
                self.similarity
            )
        } else {
            runcommand!(
                client,
                "import",
                self.patches,
                "--strip",
                self.strip,
                "--force",
                self.force,
                "--no-commit",
                self.nocommit,
                "--bypass",
                self.bypass,
                "--exact",
                self.exact,
                "--import-branch",
                self.importbranch,
                "--message",
                self.message,
                "--date",
                self.date,
                "--user",
                self.user,
                "--similarity",
                self.similarity
            )
        }
    }
}

impl Client {
    pub fn import(&mut self, x: Arg) -> Result<Vec<u8>, HglibError> {
        let (data, _) = x.run(self)?;
        Ok(data)
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub name: &'a str,
    pub clean: bool,
    pub force: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            name: "",
            clean: false,
            force: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "branch",
            &[self.name],
            "-C",
            self.clean,
            "-f",
            self.force
        )
    }
}

impl Client {
    pub fn branch(&mut self, x: Arg) -> Result<String, HglibError> {
        if !x.name.is_empty() && x.clean {
            return Err(HglibError::from("Cannot use both name and clean"));
        }
        let (data, _) = x.run(self)?;
        if !x.name.is_empty() {
            Ok(x.name.to_string())
        } else {
            let pos = if let Some(pos) = data
                .iter()
                .rposition(|x| *x != b' ' && *x != b'\t' && *x != b'\n')
            {
                pos + 1
            } else {
                data.len()
            };
            let data = &data[..pos];

            if !x.clean {
                let o = String::from_utf8(data.to_vec())?;
                Ok(o)
            } else {
                let len = "reset working directory to branch ".len();
                let o = String::from_utf8(data[len..].to_vec())?;
                Ok(o)
            }
        }
    }
}

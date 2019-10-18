// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::runcommand;

pub struct Arg {}

impl Default for Arg {
    fn default() -> Self {
        Self {}
    }
}

impl Arg {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(client, "root", &[""])
    }
}

impl Client {
    pub fn root(&mut self, x: Arg) -> Result<String, HglibError> {
        let (data, _) = x.run(self)?;
        let pos = data
            .iter()
            .rposition(|x| *x != b' ' && *x != b'\n')
            .map_or(data.len(), |p| p + 1);
        let data = &data[..pos];

        Ok(String::from_utf8(data.to_vec())?)
    }
}

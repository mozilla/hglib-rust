// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::client::{Client, HglibError, Runner};
use crate::runcommand;

pub struct Arg<'a> {
    pub name: &'a str,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self { name: "" }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(client, "paths", &[self.name])
    }
}

#[derive(Debug)]
pub enum Paths {
    Map(HashMap<String, String>),
    Value(String),
}

impl Client {
    pub fn paths(&mut self, x: Arg) -> Result<Paths, HglibError> {
        let (data, _) = x.run(self)?;
        if x.name.is_empty() {
            let mut map = HashMap::new();
            for line in data.split(|c| *c == b'\n') {
                if let Some(eq_pos) = line.iter().position(|c| *c == b' ') {
                    if let Some(two) = line.get(eq_pos + 1..eq_pos + 3) {
                        if two == b"= " {
                            map.insert(
                                String::from_utf8(unsafe {
                                    line.get_unchecked(..eq_pos).to_vec()
                                })?,
                                String::from_utf8(unsafe {
                                    line.get_unchecked(eq_pos + 3..).to_vec()
                                })?,
                            );
                        }
                    }
                }
            }
            Ok(Paths::Map(map))
        } else {
            Ok(Paths::Value(String::from_utf8({
                let pos = data
                    .iter()
                    .rposition(|x| *x != b' ' && *x != b'\n')
                    .map_or(data.len(), |p| p + 1);
                data[..pos].to_vec()
            })?))
        }
    }
}

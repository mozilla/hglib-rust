// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::client::{Client, HglibError, Runner};
use crate::{runcommand, MkArg};

pub struct Arg<'a> {
    pub names: &'a [&'a str],
    pub untrusted: bool,
    pub showsource: bool,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Self {
        Self {
            names: &[],
            untrusted: false,
            showsource: false,
        }
    }
}

impl<'a> Arg<'a> {
    fn run(&self, client: &mut Client) -> Result<(Vec<u8>, i32), HglibError> {
        runcommand!(
            client,
            "showconfig",
            self.names,
            "-u",
            self.untrusted,
            "--debug",
            self.showsource
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub source: Option<String>,
    pub section: String,
    pub key: String,
    pub value: String,
}

impl Config {
    fn no_source(line: &[u8]) -> Result<Self, HglibError> {
        let (section, key, value) = splitline(line)?;
        Ok(Self {
            source: None,
            section: section.to_string(),
            key: key.to_string(),
            value: value.to_string(),
        })
    }

    fn with_source(source: &str, line: &[u8]) -> Result<Self, HglibError> {
        let (section, key, value) = splitline(line)?;
        Ok(Self {
            source: Some(source.to_string()),
            section: section.to_string(),
            key: key.to_string(),
            value: value.to_string(),
        })
    }
}

fn splitline(data: &[u8]) -> Result<(&str, &str, &str), HglibError> {
    let data = std::str::from_utf8(data)?;
    let err = || HglibError::from(format!("Invalid line in config: {}", data));
    let data = data.trim_end();
    let mut iter = data.rsplitn(2, '=');
    let value = iter.next().ok_or_else(err)?;
    let section_key = iter.next().ok_or_else(err)?;

    let mut iter = section_key.splitn(2, '.');
    let section = iter.next().ok_or_else(err)?;
    let key = iter.next().ok_or_else(err)?;

    Ok((section, key, value))
}

fn get_skv(line: &[u8]) -> Option<(&[u8], &[u8])> {
    for (n, c) in line.windows(2).enumerate() {
        if c == b": " {
            return Some((&line[..n], &line[n + 2..]));
        }
    }
    None
}

impl Client {
    pub fn config(&mut self, x: Arg) -> Result<Vec<Config>, HglibError> {
        let (data, _) = x.run(self)?;
        let mut conf = Vec::new();
        if x.showsource {
            let mut iter = data.split(|x| *x == b'\n').filter(|x| !x.is_empty());
            let mut line = iter.next();
            while let Some(l) = line {
                if !l.starts_with(b"read config from: ") && !l.starts_with(b"set config by: ") {
                    break;
                }
                line = iter.next();
            }
            while let Some(l) = line {
                if let Some((source, kv)) = get_skv(l) {
                    let source = std::str::from_utf8(source)?;
                    conf.push(Config::with_source(source, kv)?);
                } else {
                    return Err(HglibError::from(format!(
                        "invalid line in config: {}",
                        std::str::from_utf8(l)?
                    )));
                }
                line = iter.next();
            }
        } else {
            for line in data.split(|x| *x == b'\n').filter(|x| !x.is_empty()) {
                conf.push(Config::no_source(line)?);
            }
        }
        Ok(conf)
    }
}

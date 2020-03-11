// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{hg, init, root, Basic, Client, HG};
use std::fs;

mod common;

#[test]
fn test_basic() {
    let c = common::TestClient::new("init_basic", &[]);
    assert!(fs::remove_dir_all(c.get_path(".hg")).is_ok());

    let path = c.get_path("");
    assert!(HG!(init, dest = &path).is_ok());

    let mut client = Client::open(&path, "UTF-8", &[]).unwrap();
    let root = hg!(client, root).unwrap();
    assert!(root.ends_with("init_basic"));
}

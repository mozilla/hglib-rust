// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{commit, hg, remove};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("remove_basic", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    assert!(hg!(c.client, remove, files = &["a"]).is_ok());
}

#[test]
fn test_warnings() {
    let mut c = common::TestClient::new("remove_warnings", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    assert_eq!(hg!(c.client, remove, files = &["a", "b"]).unwrap(), false);
}

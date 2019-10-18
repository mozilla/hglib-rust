// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{bundle, clone, commit, hg};

mod common;

#[test]
fn test_no_changes() {
    let mut c = common::TestClient::new("bundle_no_changes", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    assert!(!hg!(c.client, bundle, file = "bundle", destrepo = ".").unwrap());
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("bundle_basic", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    assert!(hg!(c.client, clone, dest = "other").is_ok());

    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    assert!(hg!(c.client, bundle, file = "bundle", destrepo = "other").unwrap());
}

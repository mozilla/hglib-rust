// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{add, forget, hg};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("forget_basic", &[]);
    c.append("a", &["a"]);
    hg!(c.client, add, files = &["a"]).unwrap();
    assert!(hg!(c.client, forget, files = &["a"]).unwrap());
}

#[test]
fn test_warnings() {
    let mut c = common::TestClient::new("forget_warnings", &[]);
    assert!(!hg!(c.client, forget, files = &["a"]).unwrap());
    c.append("a", &["a"]);
    hg!(c.client, add, files = &["a"]).unwrap();
    assert!(!hg!(c.client, forget, files = &["a", "b"]).unwrap());
}

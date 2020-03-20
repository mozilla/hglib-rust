// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{client, clone, commit, hg, log, push};
mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("push_basic", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    assert!(hg!(c.client, clone, source = ".", dest = "other").is_ok());
    let other = c.get_path("other");
    let mut other = client::Client::open(&other, "UTF-8", &[]).unwrap();

    c.append("a", &["a"]);
    hg!(c.client, commit, message = "second").unwrap();

    assert!(hg!(c.client, push, dest = "other").unwrap());
    assert_eq!(hg!(c.client, log).unwrap(), hg!(other, log).unwrap());
}

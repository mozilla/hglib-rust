// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    client, clone, commit, hg, log, pull, status,
    status::{Code, Status},
};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("pull_basic", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    assert!(hg!(c.client, clone, dest = "other").is_ok());
    let other = c.get_path("other");
    let mut other = client::Client::open(&other, "UTF-8", &[]).unwrap();

    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    assert!(hg!(other, pull).unwrap());
    assert_eq!(hg!(c.client, log).unwrap(), hg!(other, log).unwrap());
}

#[test]
fn test_unresolved() {
    let mut c = common::TestClient::new("pull_unresolved", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    assert!(hg!(c.client, clone, dest = "other").is_ok());
    let other = c.get_path("other");
    let mut other = client::Client::open(&other, "UTF-8", &[]).unwrap();

    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    c.append("other/a", &["b"]);
    assert!(!hg!(other, pull, update = true).unwrap());
    assert!(hg!(other, status).unwrap().contains(&Status {
        code: Code::Modified,
        filename: "a".to_string(),
    }));
}

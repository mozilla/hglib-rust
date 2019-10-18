// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{branch, commit, heads, hg, tip};

mod common;

#[test]
fn test_empty() {
    let mut c = common::TestClient::new("heads_empty", &[]);
    assert!(hg!(c.client, heads).unwrap().is_empty());
}

#[test]
fn test_warnings() {
    let mut c = common::TestClient::new("heads_basic", &[]);
    c.append("a", &["a"]);
    let rev0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    assert_eq!(
        hg!(c.client, heads).unwrap(),
        vec![hg!(c.client, tip).unwrap()]
    );

    hg!(c.client, branch, name = "foo").unwrap();
    c.append("a", &["a"]);
    let _ = hg!(c.client, commit, message = "second", addremove = true).unwrap();

    assert!(
        hg!(c.client, heads, rev = &[&rev0.node], topological = true)
            .unwrap()
            .is_empty()
    );
}

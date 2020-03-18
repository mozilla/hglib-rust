// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{commit, hg, parents};

mod common;

#[test]
fn test_noparents() {
    let mut c = common::TestClient::new("parents_noparents", &[]);
    assert_eq!(hg!(c.client, parents).unwrap().len(), 0);
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("parents_basic", &[]);
    c.append("a", &["a"]);

    let commit = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    assert_eq!(commit.node, hg!(c.client, parents).unwrap()[0].node);
    assert_eq!(
        commit.node,
        hg!(c.client, parents, file = "a").unwrap()[0].node
    );
}

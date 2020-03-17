// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    commit::{self, Commit},
    hg, tag,
    tags::{self, Tag},
    tip, version,
};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("tag_basic", &[]);
    c.append("a", &["a"]);
    let Commit { rev, node } = hg!(c.client, commit, message = "first", addremove = true).unwrap();

    assert!(hg!(c.client, tag, names = &["my tag"]).is_ok());
    assert!(hg!(
        c.client,
        tag,
        names = &["local tag"],
        rev = &rev.to_string(),
        local = true
    )
    .is_ok());

    let version = hg!(c.client, version).unwrap();
    if version.major < 2 {
        c.reopen();
    }

    let expected = vec![
        Tag {
            name: "tip".to_string(),
            rev: 1,
            node: hg![c.client, tip].unwrap().node[..12].to_string(),
            islocal: false,
        },
        Tag {
            name: "my tag".to_string(),
            rev: 0,
            node: node[..12].to_string(),
            islocal: false,
        },
        Tag {
            name: "local tag".to_string(),
            rev: 0,
            node: node[..12].to_string(),
            islocal: true,
        },
    ];

    assert_eq!(hg![c.client, tags].unwrap(), expected);
}

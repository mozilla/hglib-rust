// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{commit, hg, merge, resolve, update};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("resolve_basic", &[]);

    c.append("a", &["a"]);
    c.append("b", &["b"]);
    let node0 = hg!(c.client, commit, message = "first", addremove = true)
        .unwrap()
        .node;

    c.append("a", &["a"]);
    c.append("b", &["b"]);
    let node1 = hg!(c.client, commit, message = "second").unwrap().node;

    assert!(hg!(c.client, update, rev = &node0).is_ok());
    c.append("a", &["b"]);
    c.append("b", &["a"]);
    assert!(hg!(c.client, commit, message = "thrid").is_ok());

    assert!(hg!(c.client, merge, rev = &node1).is_err());
    assert!(hg!(c.client, resolve, all = true).is_err());

    assert_eq!(
        hg!(c.client, resolve, listfiles = true).unwrap(),
        Some(vec![
            resolve::Resolve {
                kind: resolve::Kind::Unresolved,
                filename: "a".to_string(),
            },
            resolve::Resolve {
                kind: resolve::Kind::Unresolved,
                filename: "b".to_string(),
            },
        ])
    );

    assert!(hg!(c.client, resolve, file = &["a"], mark = true).is_ok());
    assert_eq!(
        hg!(c.client, resolve, listfiles = true).unwrap(),
        Some(vec![
            resolve::Resolve {
                kind: resolve::Kind::Resolved,
                filename: "a".to_string(),
            },
            resolve::Resolve {
                kind: resolve::Kind::Unresolved,
                filename: "b".to_string(),
            },
        ])
    );
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    bookmark,
    bookmarks::{self, Bookmark, Bookmarks},
    commit, hg,
};

mod common;

#[test]
fn test_empty() {
    let mut c = common::TestClient::new("bookmarks_empty", &[]);
    assert_eq!(
        hg!(c.client, bookmarks).unwrap(),
        Bookmarks {
            bookmarks: Vec::new(),
            current: None,
        }
    );
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("bookmarks_basic", &[]);
    c.append("a", &["a"]);
    let rev0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    c.append("a", &["a"]);
    let rev1 = hg!(c.client, commit, message = "second").unwrap();

    hg!(
        c.client,
        bookmark,
        name = "zero",
        rev = &rev0.rev.to_string()
    )
    .unwrap();
    assert_eq!(
        hg!(c.client, bookmarks).unwrap(),
        Bookmarks {
            bookmarks: vec![Bookmark {
                name: "zero".to_string(),
                rev: rev0.rev,
                node: rev0.node[..12].to_string(),
            }],
            current: None,
        }
    );

    hg!(
        c.client,
        bookmark,
        name = "one",
        rev = &rev1.rev.to_string()
    )
    .unwrap();
    assert_eq!(
        hg!(c.client, bookmarks).unwrap(),
        Bookmarks {
            bookmarks: vec![
                Bookmark {
                    name: "one".to_string(),
                    rev: rev1.rev,
                    node: rev1.node[..12].to_string(),
                },
                Bookmark {
                    name: "zero".to_string(),
                    rev: rev0.rev,
                    node: rev0.node[..12].to_string(),
                }
            ],
            current: None,
        }
    );
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use std::mem;

use crate::hglib::{bookmark, client, clone, commit, hg, incoming, log, outgoing, tip};

mod common;

macro_rules! match_enum {
    ($l: expr, $r: expr) => {{
        assert_eq!(mem::discriminant(&$l), mem::discriminant(&$r))
    }};
}

#[test]
fn test_empty() {
    let mut c = common::TestClient::new("outgoing_incoming_empty", &[]);

    assert!(hg!(c.client, clone, dest = "other").is_ok());
    let other = c.get_path("other");
    let mut other = client::Client::open(&other, "UTF-8", &[]).unwrap();

    match_enum!(hg!(other, incoming).unwrap(), incoming::Incoming::Empty);

    match_enum!(hg!(other, outgoing).unwrap(), outgoing::Outgoing::Empty);
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("outgoing_incoming_basic", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    assert!(hg!(c.client, clone, dest = "other").is_ok());
    let other = c.get_path("other");
    let mut other = client::Client::open(&other, "UTF-8", &[]).unwrap();

    assert_eq!(hg!(c.client, log).unwrap(), hg!(other, log).unwrap());
    match_enum!(
        hg!(c.client, outgoing, path = "other").unwrap(),
        outgoing::Outgoing::Empty
    );
    match_enum!(hg!(other, incoming).unwrap(), incoming::Incoming::Empty);

    c.append("a", &["a"]);
    let node = hg!(c.client, commit, message = "second").unwrap().node;
    let out = match hg!(c.client, outgoing, path = "other").unwrap() {
        outgoing::Outgoing::Revisions(out) => out,
        _ => panic!("outgoing not return outoing::Outgoing::Revisions type"),
    };

    assert_eq!(out.len(), 1);
    assert_eq!(out[0].node, node);
    // assert_eq!(out[0].node, node);
    match hg!(other, incoming).unwrap() {
        incoming::Incoming::Revisions(ret) => assert_eq!(out[0], ret[0]),
        _ => panic!("incoming not return incoming::Incoming::Revisions type"),
    };
}

#[test]
fn test_bookmarks() {
    let mut c = common::TestClient::new("outgoing_incoming_bookmarks", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    assert!(hg!(c.client, clone, dest = "other").is_ok());
    let other = c.get_path("other");
    let mut other = client::Client::open(&other, "UTF-8", &[]).unwrap();

    assert!(hg!(c.client, bookmark, name = "bm1", rev = "1").is_ok());

    let node = &hg!(c.client, tip).unwrap().node[0..12];

    assert_eq!(
        hg!(other, incoming, bookmarks = true).unwrap(),
        incoming::Incoming::Bookmarks(vec![incoming::Bookmark {
            bookmark: "bm1".to_string(),
            revision: node.to_string(),
        }])
    );

    assert_eq!(
        hg!(c.client, outgoing, path = "other", bookmarks = true).unwrap(),
        outgoing::Outgoing::Bookmarks(vec![outgoing::Bookmark {
            bookmark: "bm1".to_string(),
            revision: node.to_string(),
        }])
    );
}

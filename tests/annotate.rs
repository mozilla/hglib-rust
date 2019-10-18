// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    annotate::{self, Line, Lines},
    commit, hg,
};

mod common;

fn compare(mut lines: Lines, expected: Vec<Line>) {
    let mut iter = expected.into_iter();
    let lines = &mut lines;
    while let Ok(line) = lines.next_line() {
        if line.is_none() {
            assert!(iter.next().is_none());
            break;
        }
        let line = line.unwrap();
        assert_eq!(line, iter.next().unwrap());
    }
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("annotate_basic", &[]);
    c.append("a", &["a\n"]);
    let rev0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    c.append("a", &["b\n"]);
    let rev1 = hg!(c.client, commit, message = "second", addremove = true).unwrap();

    compare(
        hg!(c.client, annotate, files = &["a"]).unwrap(),
        vec![
            Line {
                info: "0",
                content: b"a",
            },
            Line {
                info: "1",
                content: b"b",
            },
        ],
    );

    compare(
        hg!(
            c.client,
            annotate,
            files = &["a"],
            user = true,
            file = true,
            number = true,
            changeset = true,
            line = true,
            verbose = true
        )
        .unwrap(),
        vec![
            Line {
                info: &format!("test 0 {} a:1", &rev0.node[..12]),
                content: b"a",
            },
            Line {
                info: &format!("test 1 {} a:2", &rev1.node[..12]),
                content: b"b",
            },
        ],
    );
}

#[test]
fn test_files() {
    let mut c = common::TestClient::new("annotate_files", &[]);
    c.append("a", &["a\n"]);
    let _ = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    c.append("b", &["b\n"]);
    let _ = hg!(c.client, commit, message = "second", addremove = true).unwrap();

    compare(
        hg!(c.client, annotate, files = &["a", "b"]).unwrap(),
        vec![
            Line {
                info: "0",
                content: b"a",
            },
            Line {
                info: "1",
                content: b"b",
            },
        ],
    );
}

#[test]
fn test_two_colons() {
    let mut c = common::TestClient::new("annotate_two_colons", &[]);
    c.append("a", &["a: b\n"]);
    let _ = hg!(c.client, commit, message = "first", addremove = true).unwrap();

    compare(
        hg!(c.client, annotate, files = &["a"]).unwrap(),
        vec![Line {
            info: "0",
            content: b"a: b",
        }],
    );
}

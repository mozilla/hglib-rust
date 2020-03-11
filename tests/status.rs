// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    add, commit, copy, hg, remove, status,
    status::{Code, Status},
};
use std::fs;

mod common;

#[test]
fn test_empty() {
    let mut c = common::TestClient::new("status_empty", &[]);
    let st = hg!(c.client, status).unwrap();
    assert_eq!(st.len(), 0);
}

#[test]
fn test_one_of_each() {
    let mut c = common::TestClient::new("status_one_of_each", &[]);
    c.append(".hgignore", &["ignored"]);
    c.append("ignored", &["a"]);
    c.append("clean", &["a"]);
    c.append("modified", &["a"]);
    c.append("removed", &["a"]);
    c.append("missing", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    c.append("modified", &["a"]);
    c.append("added", &["a"]);
    assert!(hg!(c.client, add, files = &["added"]).is_ok());

    assert!(fs::remove_file(c.get_path("missing")).is_ok());
    assert!(hg!(c.client, remove, files = &["removed"]).is_ok());
    c.append("untracked", &[]);

    let expected = vec![
        Status {
            code: Code::Modified,
            filename: "modified".to_string(),
        },
        Status {
            code: Code::Added,
            filename: "added".to_string(),
        },
        Status {
            code: Code::Removed,
            filename: "removed".to_string(),
        },
        Status {
            code: Code::Clean,
            filename: ".hgignore".to_string(),
        },
        Status {
            code: Code::Clean,
            filename: "clean".to_string(),
        },
        Status {
            code: Code::Missing,
            filename: "missing".to_string(),
        },
        Status {
            code: Code::NotTracked,
            filename: "untracked".to_string(),
        },
        Status {
            code: Code::Ignored,
            filename: "ignored".to_string(),
        },
    ];

    let status = hg!(c.client, status, all = true).unwrap();
    for e in expected {
        assert!(status.iter().any(|x| *x == e));
    }
}

#[test]
fn test_copy() {
    let mut c = common::TestClient::new("status_copy", &[]);
    c.append("source", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    assert!(hg!(c.client, copy, source = &["source"], dest = "dest").is_ok());
    assert_eq!(
        hg!(c.client, status, copies = true).unwrap(),
        vec![
            Status {
                code: Code::Added,
                filename: "dest".to_string(),
            },
            Status {
                code: Code::Origin,
                filename: "source".to_string(),
            }
        ]
    );
}

#[test]
fn test_copy_origin_space() {
    let mut c = common::TestClient::new("status_copy_origin_space", &[]);
    c.append("s ource", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    assert!(hg!(c.client, copy, source = &["s ource"], dest = "dest").is_ok());
    assert_eq!(
        hg!(c.client, status, copies = true).unwrap(),
        vec![
            Status {
                code: Code::Added,
                filename: "dest".to_string(),
            },
            Status {
                code: Code::Origin,
                filename: "s ource".to_string(),
            }
        ]
    );
}

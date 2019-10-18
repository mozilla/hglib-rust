// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    commit, copy, hg, status,
    status::{Code, Status},
};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("copy_basic", &[]);
    c.append("a", &["a"]);
    hg!(c.client, commit, message = "first", addremove = true).unwrap();
    assert!(hg!(c.client, copy, source = &["a"], dest = "b").unwrap());
    assert_eq!(
        hg!(c.client, status).unwrap(),
        vec![Status {
            code: Code::Added,
            filename: "b".to_string(),
        }]
    );
    c.append("c", &["a"]);
    assert!(hg!(c.client, copy, source = &["a"], dest = "c", after = true).unwrap());
    assert_eq!(
        hg!(c.client, status).unwrap(),
        vec![
            Status {
                code: Code::Added,
                filename: "b".to_string(),
            },
            Status {
                code: Code::Added,
                filename: "c".to_string(),
            }
        ]
    );
}

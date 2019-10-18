// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    branch,
    branches::{self, Branch},
    commit, hg, log,
};

mod common;

#[test]
fn test_empty() {
    let mut c = common::TestClient::new("branches_empty", &[]);
    assert_eq!(hg!(c.client, branches).unwrap(), Vec::new());
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("branches_basic", &[]);
    c.append("a", &["a"]);
    let rev0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    assert!(hg!(c.client, branch, name = "foo").is_ok());
    c.append("a", &["a"]);
    let rev1 = hg!(c.client, commit, message = "second").unwrap();
    let branches = hg!(c.client, branches).unwrap();

    let mut expected = Vec::new();
    for rev in [rev1, rev0].iter() {
        let rev = &hg!(c.client, log, revrange = &[&rev.rev.to_string()]).unwrap()[0];
        expected.push(Branch {
            name: rev.branch.clone(),
            rev: rev.rev,
            node: rev.node[..12].to_string(),
        });
    }

    assert_eq!(branches, expected);
}

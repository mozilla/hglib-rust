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
    let mut c = common::TestClient::new("branch_empty", &[]);
    assert_eq!(hg!(c.client, branch).unwrap(), "default".to_string());
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("branch_basic", &[]);
    assert_eq!(
        hg!(c.client, branch, name = "foo").unwrap(),
        "foo".to_string()
    );

    c.append("a", &["a"]);
    let commit = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    let rev = &hg!(c.client, log, revrange = &[&commit.rev.to_string()]).unwrap()[0];
    assert_eq!(rev.branch, "foo".to_string());

    assert_eq!(
        hg!(c.client, branches).unwrap()[0],
        Branch {
            name: rev.branch.clone(),
            rev: rev.rev,
            node: rev.node[..12].to_string()
        }
    );
}

#[test]
fn test_reset_with_name() {
    let mut c = common::TestClient::new("branch_reset_with_name", &[]);
    assert!(hg!(c.client, branch, name = "foo", clean = true).is_err());
}

#[test]
fn test_reset() {
    let mut c = common::TestClient::new("branch_reset", &[]);
    assert!(hg!(c.client, branch, name = "foo").is_ok());
    assert_eq!(
        hg!(c.client, branch, clean = true).unwrap(),
        "default".to_string()
    );
}

#[test]
fn test_exists() {
    let mut c = common::TestClient::new("branch_exists", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    assert!(hg!(c.client, branch, name = "foo").is_ok());

    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    assert!(hg!(c.client, branch, name = "default").is_err());
}

#[test]
fn test_force() {
    let mut c = common::TestClient::new("branch_force", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    assert!(hg!(c.client, branch, name = "foo").is_ok());

    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    assert!(hg!(c.client, branch, name = "default").is_err());
    assert_eq!(
        hg!(c.client, branch, name = "default", force = true).unwrap(),
        "default".to_string()
    );
}

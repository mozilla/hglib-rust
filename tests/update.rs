// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    commit::{self, Commit},
    hg, parents,
    status::{self, Code, Status},
    update::{self, Update},
    Runner,
};

mod common;

fn set_up(c: &mut common::TestClient) -> (Commit, Commit) {
    c.append("a", &["a"]);
    let commit0 = hg![c.client, commit, message = "first", addremove = true].unwrap();
    c.append("a", &["a"]);
    let commit1 = hg![c.client, commit, message = "second"].unwrap();

    (commit0, commit1)
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("update_basic", &[]);
    let (c0, _) = set_up(&mut c);

    assert_eq!(
        hg![c.client, update, rev = &c0.rev.to_string()].unwrap(),
        Update {
            updated: 1,
            merged: 0,
            removed: 0,
            unresolved: 0,
        }
    );
}

#[test]
fn test_unresolved() {
    let mut c = common::TestClient::new("update_unresolved", &[]);
    let (c0, _) = set_up(&mut c);

    hg![c.client, update, rev = &c0.rev.to_string()].unwrap();
    c.append("a", &["b"]);

    assert_eq!(
        hg![c.client, update].unwrap(),
        Update {
            updated: 0,
            merged: 0,
            removed: 0,
            unresolved: 1,
        }
    );

    assert!(hg![c.client, status].unwrap().contains(&Status {
        code: Code::Modified,
        filename: "a".to_string(),
    }));
}

#[test]
fn test_merge() {
    let mut c = common::TestClient::new("update_merge", &[]);
    set_up(&mut c);

    c.append("a", &["\n\n\n\nb"]);
    let c2 = hg![c.client, commit, message = "third"].unwrap();
    c.append("a", &["b"]);
    hg![c.client, commit, message = "fourth"].unwrap();

    hg![c.client, update, rev = &c2.rev.to_string()].unwrap();

    c.prepend("a", &["a"]);

    assert_eq!(
        hg![c.client, update].unwrap(),
        Update {
            updated: 0,
            merged: 1,
            removed: 0,
            unresolved: 0,
        }
    );

    assert_eq!(
        hg![c.client, status].unwrap(),
        vec![Status {
            code: Code::Modified,
            filename: "a".to_string(),
        }]
    );
}

#[test]
fn test_tip() {
    let mut c = common::TestClient::new("update_tip", &[]);
    let (c0, c1) = set_up(&mut c);

    hg![c.client, update, rev = &c0.rev.to_string()].unwrap();
    let u = hg![c.client, update].unwrap();
    assert_eq!(u.updated, 1);
    assert_eq!(hg![c.client, parents].unwrap()[0].node, c1.node);

    hg![c.client, update, rev = &c0.rev.to_string()].unwrap();
    c.append("a", &["b"]);
    let c2 = hg![c.client, commit, message = "new head"].unwrap();
    hg![c.client, update, rev = &c0.rev.to_string()].unwrap();

    hg![c.client, update].unwrap();

    assert_eq!(hg![c.client, parents].unwrap()[0].node, c2.node);
}

#[test]
fn test_check_clean() {
    let mut c = common::TestClient::new("update_check_clean", &[]);
    set_up(&mut c);

    assert!(hg![c.client, update, clean = true, check = true].is_err());
}

#[test]
fn test_clean() {
    let mut c = common::TestClient::new("update_clean", &[]);
    set_up(&mut c);

    let old = c.read("a");
    c.append("a", &["b"]);
    assert!(hg![c.client, update, check = true].is_err());

    let u = hg![c.client, update, clean = true].unwrap();
    assert_eq!(u.updated, 1);

    assert_eq!(old, c.read("a"));
}

#[test]
fn test_basic_plain() {
    let mut c = common::TestClient::new("update_basic_plain", &[]);
    c.append(".hg/hgrc", &["[defaults]", "update = -v"]);
    c.reopen();

    let (c0, _) = set_up(&mut c);
    assert_eq!(
        hg![c.client, update, rev = &c0.rev.to_string()].unwrap(),
        Update {
            updated: 1,
            merged: 0,
            removed: 0,
            unresolved: 0,
        }
    );
}

#[test]
fn test_disabled_largefiles() {
    let mut c = common::TestClient::new("disabled_largefiles", &[]);
    c.append(".hg/hgrc", &["[extensions]", "largefiles ="]);
    c.reopen();

    let (c0, _) = set_up(&mut c);

    c.append("b", &["a"]);

    c.client.runcommand(&["add", "b", "--large"]).unwrap();

    let c2 = hg![c.client, commit, message = "third"].unwrap();

    hg![c.client, update, rev = &c0.rev.to_string()].unwrap();

    assert_eq!(
        hg![c.client, update, rev = &c2.rev.to_string(), clean = true].unwrap(),
        Update {
            updated: 2,
            merged: 0,
            removed: 0,
            unresolved: 0,
        }
    );
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use chrono::{SecondsFormat, SubsecRound, Utc};

use crate::hglib::{
    branch,
    branches::{self, Branch},
    commit, hg, log, tip,
};

mod common;

#[test]
fn test_user() {
    let mut c = common::TestClient::new("commit_user", &[]);
    c.append("a", &["a"]);
    let rev = hg!(
        c.client,
        commit,
        message = "first",
        addremove = true,
        user = "foo"
    )
    .unwrap();
    let rev = hg!(c.client, log, revrange = &[&rev.node], limit = Some(1)).unwrap();
    let rev = &rev[0];
    assert_eq!(rev.author, "foo");
}

#[test]
fn test_no_user() {
    let mut c = common::TestClient::new("commit_no_user", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", user = "").is_err());
}

#[test]
fn test_close_branch() {
    let mut c = common::TestClient::new("commit_close_branch", &[]);
    c.append("a", &["a"]);
    let rev0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    hg!(c.client, branch, name = "foo").unwrap();
    c.append("a", &["a"]);
    let rev1 = hg!(c.client, commit, message = "second", addremove = true).unwrap();
    let revclose = hg!(
        c.client,
        commit,
        message = "closing foo",
        closebranch = true
    )
    .unwrap();
    let revs = hg!(
        c.client,
        log,
        revrange = &[&rev0.node, &rev1.node, &revclose.node]
    )
    .unwrap();

    assert_eq!(
        hg!(c.client, branches).unwrap(),
        vec![Branch {
            name: revs[0].branch.clone(),
            rev: revs[0].rev,
            node: revs[0].node[..12].to_string(),
        }]
    );

    assert_eq!(
        hg!(c.client, branches, closed = true).unwrap(),
        vec![
            Branch {
                name: revs[2].branch.clone(),
                rev: revs[2].rev,
                node: revs[2].node[..12].to_string(),
            },
            Branch {
                name: revs[0].branch.clone(),
                rev: revs[0].rev,
                node: revs[0].node[..12].to_string(),
            },
        ]
    );
}

#[test]
fn test_message_logfile() {
    let mut c = common::TestClient::new("commit_message_logfile", &[]);
    assert!(hg!(c.client, commit, message = "foo", logfile = "bar").is_err());
    assert!(hg!(c.client, commit).is_err());
}

#[test]
fn test_date() {
    let mut c = common::TestClient::new("commit_date", &[]);
    c.append("a", &["a"]);
    let now = Utc::now().trunc_subsecs(0);
    let _ = hg!(
        c.client,
        commit,
        message = "first",
        addremove = true,
        date = &now.to_rfc3339_opts(SecondsFormat::Secs, false)
    )
    .unwrap();
    assert_eq!(hg!(c.client, tip).unwrap().date, now);
}

#[test]
fn test_amend() {
    let mut c = common::TestClient::new("commit_amend", &[]);
    c.append("a", &["a"]);
    let now = Utc::now().trunc_subsecs(0);
    let rev0 = hg!(
        c.client,
        commit,
        message = "first",
        addremove = true,
        date = &now.to_rfc3339_opts(SecondsFormat::Secs, false)
    )
    .unwrap();
    assert_eq!(hg!(c.client, tip).unwrap().date, now);

    c.append("a", &["a"]);
    let rev1 = hg!(c.client, commit, amend = true).unwrap();
    assert_eq!(hg!(c.client, tip).unwrap().date, now);
    assert_ne!(rev0.node, rev1.node);
    assert_eq!(hg!(c.client, log).unwrap().len(), 1);
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use std::collections::HashMap;

use crate::hglib::{
    bookmark, client, clone, commit, hg, merge, phase,
    summary::{self, Mq, Remote, Revision, Summary},
    update, version,
};

mod common;

fn has_phases(c: &mut common::TestClient) -> bool {
    let version = hg!(c.client, version).unwrap();
    version.major >= 3 && (version.major != 3 || version.minor >= 5)
}

#[test]
fn test_empty() {
    let mut c = common::TestClient::new("summary_empty", &[]);
    assert_eq!(
        hg!(c.client, summary).unwrap(),
        Summary {
            parent: vec![Revision {
                rev: -1,
                node: "000000000000".to_string(),
                tags: "tip".to_string(),
                message: "".to_string()
            }],
            branch: "default".to_string(),
            commit: true,
            update: 0,
            remote: None,
            mq: Mq::default(),
            others: HashMap::default(),
        }
    );
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("summary_basic", &[]);
    c.append("a", &["a"]);
    let c0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();

    assert_eq!(
        hg!(c.client, summary).unwrap(),
        Summary {
            parent: vec![Revision {
                rev: 0,
                node: c0.node[..12].to_string(),
                tags: "tip".to_string(),
                message: "first".to_string()
            }],
            branch: "default".to_string(),
            commit: true,
            update: 0,
            remote: None,
            mq: Mq::default(),
            others: {
                let mut map = HashMap::default();
                if has_phases(&mut c) {
                    map.insert("phases".to_string(), "1 draft".to_string());
                }
                map
            }
        }
    );
}

#[test]
fn test_commit_dirty() {
    let mut c = common::TestClient::new("summary_commit_dirty", &[]);
    c.append("a", &["a"]);
    let c0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    c.append("a", &["a"]);

    assert_eq!(
        hg!(c.client, summary).unwrap(),
        Summary {
            parent: vec![Revision {
                rev: 0,
                node: c0.node[..12].to_string(),
                tags: "tip".to_string(),
                message: "first".to_string()
            }],
            branch: "default".to_string(),
            commit: false,
            update: 0,
            remote: None,
            mq: Mq::default(),
            others: {
                let mut map = HashMap::default();
                if has_phases(&mut c) {
                    map.insert("phases".to_string(), "1 draft".to_string());
                }
                map
            }
        }
    );
}

#[test]
fn test_secret_commit_clean() {
    let mut c = common::TestClient::new("summary_secret_commit_clean", &[]);
    let version = hg!(c.client, version).unwrap();
    if version.major < 2 || (version.major == 2 && version.minor < 1) {
        return;
    }

    c.append("a", &["a"]);
    let c0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    hg![
        c.client,
        phase,
        revs = &[&c0.rev.to_string()],
        secret = true,
        force = true
    ]
    .unwrap();
    let e = hg!(c.client, summary).unwrap();

    assert!(e.commit);
}

#[test]
fn test_update() {
    let mut c = common::TestClient::new("summary_update", &[]);

    c.append("a", &["a"]);
    let c0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    c.append("a", &["a"]);
    hg!(c.client, commit, message = "second").unwrap();
    hg!(c.client, update, rev = "0").unwrap();

    assert_eq!(
        hg!(c.client, summary).unwrap(),
        Summary {
            parent: vec![Revision {
                rev: 0,
                node: c0.node[..12].to_string(),
                tags: "".to_string(),
                message: "first".to_string()
            }],
            branch: "default".to_string(),
            commit: true,
            update: 1,
            remote: None,
            mq: Mq::default(),
            others: {
                let mut map = HashMap::default();
                if has_phases(&mut c) {
                    map.insert("phases".to_string(), "2 draft".to_string());
                }
                map
            }
        }
    );
}

#[test]
fn test_remote() {
    let mut c = common::TestClient::new("summary_remote", &[]);

    c.append("a", &["a"]);
    let c0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();

    assert!(hg!(c.client, clone, dest = "other").is_ok());

    let other = c.get_path("other");
    let mut other = client::Client::open(&other, "UTF-8", &[]).unwrap();

    let mut expected = Summary {
        parent: vec![Revision {
            rev: 0,
            node: c0.node[..12].to_string(),
            tags: "tip".to_string(),
            message: "first".to_string(),
        }],
        branch: "default".to_string(),
        commit: true,
        update: 0,
        remote: Some(Remote {
            outgoing: 0,
            incoming: 0,
            outgoing_bookmarks: 0,
            incoming_bookmarks: 0,
        }),
        mq: Mq::default(),
        others: HashMap::default(),
    };

    assert_eq!(hg!(other, summary, remote = true).unwrap(), expected);

    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "second").is_ok());

    if let Some(r) = expected.remote.as_mut() {
        r.incoming = 1;
    }

    assert_eq!(hg!(other, summary, remote = true).unwrap(), expected);

    assert!(hg!(c.client, bookmark, name = "bm").is_ok());
    if let Some(r) = expected.remote.as_mut() {
        r.incoming_bookmarks = 1;
    }

    assert_eq!(hg!(other, summary, remote = true).unwrap(), expected);

    assert!(hg!(other, bookmark, name = "bmother").is_ok());
    if let Some(r) = expected.remote.as_mut() {
        r.outgoing_bookmarks = 1;
    }

    let version = hg!(c.client, version).unwrap();
    if version.major < 2 {
        expected.parent[0].tags = "tip bmother".to_string();
    } else {
        expected
            .others
            .insert("bookmarks".to_string(), "*bmother".to_string());
    }

    assert_eq!(hg!(other, summary, remote = true).unwrap(), expected);

    c.append("other/a", &["a"]);
    let c1 = hg!(other, commit, message = "second in other").unwrap();

    if let Some(r) = expected.remote.as_mut() {
        r.outgoing = 1;
    }

    expected.parent[0].rev = 1;
    expected.parent[0].node = c1.node[..12].to_string();
    expected.parent[0].message = "second in other".to_string();

    if has_phases(&mut c) {
        expected
            .others
            .insert("phases".to_string(), "1 draft".to_string());
    }

    assert_eq!(hg!(other, summary, remote = true).unwrap(), expected);
}

#[test]
fn test_two_parents() {
    let mut c = common::TestClient::new("summary_two_parents", &[]);

    c.append("a", &["a"]);
    let c0 = hg!(c.client, commit, message = "first", addremove = true).unwrap();

    c.append("a", &["a"]);
    let c1 = hg!(c.client, commit, message = "second").unwrap();

    hg!(c.client, update, rev = &c0.rev.to_string()).unwrap();
    c.append("b", &["a"]);
    let c2 = hg!(c.client, commit, message = "third", addremove = true).unwrap();

    assert!(hg!(c.client, merge, rev = &c1.rev.to_string()).is_ok());

    assert_eq!(
        hg!(c.client, summary).unwrap(),
        Summary {
            parent: vec![
                Revision {
                    rev: 2,
                    node: c2.node[..12].to_string(),
                    tags: "tip".to_string(),
                    message: "third".to_string()
                },
                Revision {
                    rev: 1,
                    node: c1.node[..12].to_string(),
                    tags: "".to_string(),
                    message: "second".to_string()
                }
            ],
            branch: "default".to_string(),
            commit: false,
            update: 0,
            remote: None,
            mq: Mq::default(),
            others: {
                let mut map = HashMap::default();
                if has_phases(&mut c) {
                    map.insert("phases".to_string(), "3 draft".to_string());
                }
                map
            }
        }
    );
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{commit, hg, phase};
mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("phase_basic", &[]);
    c.append("a", &["a"]);
    let commit = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    let phases = hg!(c.client, phase, revs = &[&commit.node])
        .unwrap()
        .unwrap();

    assert_eq!(phases[0].num, 0);
    assert_eq!(phases[0].phase, "draft");

    let phases = hg!(c.client, phase).unwrap().unwrap();
    assert_eq!(phases[0].num, 0);
    assert_eq!(phases[0].phase, "draft");
}

#[test]
fn test_public() {
    let mut c = common::TestClient::new("phase_public", &[]);
    c.append("a", &["a"]);
    let commit = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    assert!(hg!(c.client, phase, revs = &[&commit.node], public = true).is_ok());

    assert_eq!(
        hg!(c.client, phase, revs = &[&commit.node]).unwrap(),
        Some(vec![phase::Phase {
            num: 0,
            phase: "public".to_string(),
        }])
    );
}

#[test]
fn test_secret() {
    let mut c = common::TestClient::new("phase_secret", &[]);
    c.append("a", &["a"]);
    let commit = hg!(c.client, commit, message = "first", addremove = true).unwrap();
    assert!(hg!(c.client, phase, revs = &[&commit.node], public = true).is_ok());

    assert!(hg!(
        c.client,
        phase,
        revs = &[&commit.node],
        secret = true,
        force = true
    )
    .is_ok());

    assert_eq!(
        hg!(c.client, phase, revs = &[&commit.node]).unwrap(),
        Some(vec![phase::Phase {
            num: 0,
            phase: "secret".to_string(),
        }])
    );
}

#[test]
fn test_multiple() {
    let mut c = common::TestClient::new("phase_multiple", &[]);
    c.append("a", &["a"]);
    let commit0 = hg!(c.client, commit, message = "a", addremove = true).unwrap();
    assert!(hg!(c.client, phase, revs = &[&commit0.node], public = true).is_ok());

    c.append("b", &["b"]);
    let commit1 = hg!(c.client, commit, message = "b", addremove = true).unwrap();

    c.append("c", &["c"]);
    let commit2 = hg!(c.client, commit, message = "c", addremove = true).unwrap();

    assert!(hg!(
        c.client,
        phase,
        revs = &[&commit2.node],
        secret = true,
        force = true
    )
    .is_ok());

    assert_eq!(
        hg!(
            c.client,
            phase,
            revs = &[&commit0.node, &commit2.node, &commit1.node]
        )
        .unwrap(),
        Some(vec![
            phase::Phase {
                num: 0,
                phase: "public".to_string()
            },
            phase::Phase {
                num: 2,
                phase: "secret".to_string()
            },
            phase::Phase {
                num: 1,
                phase: "draft".to_string()
            },
        ])
    );
}

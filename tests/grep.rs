// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{commit, grep, hg, version, Client};

mod common;

//Remove rev filed from exptected results for the compatibility.
//Since V5.2 rev field appreard only with 'all' option.
fn remove_rev(client: &mut Client, res: Vec<grep::GrepRes>) -> Vec<grep::GrepRes> {
    if hg!(client, version).unwrap() < (5, 2, None) {
        return res;
    }

    let mut converted: Vec<grep::GrepRes> = vec![];

    for r in res {
        let mut r = r;
        r.rev = None;
        converted.push(r);
    }

    return converted;
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("grep_basic", &[]);
    c.append("a", &["x"]);
    c.append("b", &["xy"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    assert_eq!(hg!(c.client, grep, pattern = "z").unwrap().len(), 0);

    assert_eq!(
        hg!(c.client, grep, pattern = "x").unwrap(),
        remove_rev(
            &mut c.client,
            vec![
                grep::GrepRes {
                    filename: "a".to_string(),
                    rev: Some(0),
                    line: None,
                    match_status: None,
                    user: None,
                    date: None,
                    matched: Some("x".to_string())
                },
                grep::GrepRes {
                    filename: "b".to_string(),
                    rev: Some(0),
                    line: None,
                    match_status: None,
                    user: None,
                    date: None,
                    matched: Some("xy".to_string())
                },
            ]
        ),
    );

    assert_eq!(
        hg!(c.client, grep, pattern = "x", files = &["a"]).unwrap(),
        remove_rev(
            &mut c.client,
            vec![grep::GrepRes {
                filename: "a".to_string(),
                rev: Some(0),
                line: None,
                match_status: None,
                user: None,
                date: None,
                matched: Some("x".to_string())
            }]
        ),
    );

    assert_eq!(
        hg!(c.client, grep, pattern = "y").unwrap(),
        remove_rev(
            &mut c.client,
            vec![grep::GrepRes {
                filename: "b".to_string(),
                rev: Some(0),
                line: None,
                match_status: None,
                user: None,
                date: None,
                matched: Some("xy".to_string())
            },]
        ),
    );
}

#[test]
fn test_options() {
    let mut c = common::TestClient::new("grep_options", &[]);
    c.append("a", &["x\n"]);
    c.append("b", &["xy\n"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    assert_eq!(
        hg!(c.client, grep, pattern = "x", all = true).unwrap(),
        vec![
            grep::GrepRes {
                filename: "a".to_string(),
                rev: Some(0),
                line: None,
                match_status: Some("+".to_string()),
                user: None,
                date: None,
                matched: Some("x".to_string())
            },
            grep::GrepRes {
                filename: "b".to_string(),
                rev: Some(0),
                line: None,
                match_status: Some("+".to_string()),
                user: None,
                date: None,
                matched: Some("xy".to_string())
            },
        ],
    );

    assert_eq!(
        hg!(c.client, grep, pattern = "x", fileswithmatches = true).unwrap(),
        remove_rev(
            &mut c.client,
            vec![
                grep::GrepRes {
                    filename: "a".to_string(),
                    rev: Some(0),
                    line: None,
                    match_status: None,
                    user: None,
                    date: None,
                    matched: None,
                },
                grep::GrepRes {
                    filename: "b".to_string(),
                    rev: Some(0),
                    line: None,
                    match_status: None,
                    user: None,
                    date: None,
                    matched: None,
                }
            ]
        ),
    );

    assert_eq!(
        hg!(c.client, grep, pattern = "x", line = true).unwrap(),
        remove_rev(
            &mut c.client,
            vec![
                grep::GrepRes {
                    filename: "a".to_string(),
                    rev: Some(0),
                    line: Some(1),
                    match_status: None,
                    user: None,
                    date: None,
                    matched: Some("x".to_string()),
                },
                grep::GrepRes {
                    filename: "b".to_string(),
                    rev: Some(0),
                    line: Some(1),
                    match_status: None,
                    user: None,
                    date: None,
                    matched: Some("xy".to_string())
                },
            ]
        ),
    );

    assert_eq!(
        hg!(c.client, grep, pattern = "x", user = true).unwrap(),
        remove_rev(
            &mut c.client,
            vec![
                grep::GrepRes {
                    filename: "a".to_string(),
                    rev: Some(0),
                    line: None,
                    match_status: None,
                    user: Some("test".to_string()),
                    date: None,
                    matched: Some("x".to_string()),
                },
                grep::GrepRes {
                    filename: "b".to_string(),
                    rev: Some(0),
                    line: None,
                    match_status: None,
                    user: Some("test".to_string()),
                    date: None,
                    matched: Some("xy".to_string())
                },
            ]
        ),
    );

    assert_eq!(
        hg!(
            c.client,
            grep,
            pattern = "x",
            all = true,
            line = true,
            user = true,
            fileswithmatches = true
        )
        .unwrap(),
        vec![
            grep::GrepRes {
                filename: "a".to_string(),
                rev: Some(0),
                line: Some(1),
                match_status: Some("+".to_string()),
                user: Some("test".to_string()),
                date: None,
                matched: None,
            },
            grep::GrepRes {
                filename: "b".to_string(),
                rev: Some(0),
                line: Some(1),
                match_status: Some("+".to_string()),
                user: Some("test".to_string()),
                date: None,
                matched: None,
            },
        ],
    );
}

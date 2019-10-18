// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    config::{self, Config},
    hg,
};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("config_basic", &[]);
    c.append(".hg/hgrc", &["[section]", "key=value"]);
    c.reopen();
    let config = hg!(c.client, config).unwrap();
    assert!(config.iter().any(|x| *x
        == Config {
            source: None,
            section: "section".to_string(),
            key: "key".to_string(),
            value: "value".to_string(),
        }));

    assert_eq!(
        hg!(c.client, config, names = &["section"]).unwrap(),
        vec![Config {
            source: None,
            section: "section".to_string(),
            key: "key".to_string(),
            value: "value".to_string(),
        }]
    );

    assert_eq!(
        hg!(c.client, config, names = &["section", "foo"]).unwrap(),
        vec![Config {
            source: None,
            section: "section".to_string(),
            key: "key".to_string(),
            value: "value".to_string(),
        }]
    );

    assert!(hg!(c.client, config, names = &["a.b", "foo"]).is_err());
}

#[test]
fn test_show_source() {
    let mut c = common::TestClient::new("config_show_source", &[]);
    c.append(".hg/hgrc", &["[section]", "key=value"]);
    c.reopen();
    let config = hg!(c.client, config, showsource = true).unwrap();
    assert!(config.iter().any(|x| *x
        == Config {
            source: Some(c.get_path(".hg/hgrc") + ":2"),
            section: "section".to_string(),
            key: "key".to_string(),
            value: "value".to_string(),
        }));
}

#[test]
fn test_arguments() {
    let mut c = common::TestClient::new("config_arguments", &["diff.unified=5", "a.b=foo"]);
    assert_eq!(
        hg!(c.client, config, names = &["a"]).unwrap(),
        vec![Config {
            source: None,
            section: "a".to_string(),
            key: "b".to_string(),
            value: "foo".to_string(),
        }]
    );

    assert!(hg!(c.client, config, names = &["diff"])
        .unwrap()
        .iter()
        .any(|x| *x
            == Config {
                source: None,
                section: "diff".to_string(),
                key: "unified".to_string(),
                value: "5".to_string(),
            }));
}

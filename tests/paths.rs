// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{hg, paths};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("paths_basic", &[]);

    c.append(".hg/hgrc", &["[paths]", "foo = bar"]);
    c.reopen();

    let paths = hg!(c.client, paths).unwrap();
    let path_map = match paths {
        paths::Paths::Map(m) => m,
        _ => panic!("paths return {:?} expected paths::Paths::Map type", paths),
    };

    assert_eq!(path_map.len(), 1);
    assert_eq!(path_map["foo"], c.get_path("bar"));

    let paths = hg!(c.client, paths, name = "foo").unwrap();
    match paths {
        paths::Paths::Value(v) => assert_eq!(v, c.get_path("bar")),
        _ => panic!("paths return {:?} expected paths::Paths::Value type", paths),
    };
}

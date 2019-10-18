// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{client, clone, commit, hg, log, Basic, HG};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("clone_basic", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());
    assert!(HG!(clone, source = ".", dest = "cloned").is_ok());

    let cloned = c.get_path("cloned");
    let mut cloned = client::Client::open(&cloned, "UTF-8", &[]).unwrap();
    assert_eq!(hg!(c.client, log).unwrap(), hg!(cloned, log).unwrap());
}

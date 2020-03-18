// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{add, hg, r#move};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("move_basic", &[]);
    c.append("a", &["a"]);
    assert!(hg!(c.client, add, files = &["a"]).is_ok());
    assert!(hg!(c.client, r#move, source = "a", dest = "b").is_ok());
}

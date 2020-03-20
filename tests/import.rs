// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{cat, hg, import, update};

mod common;

const PATCH: &str = r#"
# HG changeset patch";
# User test
# Date 0 0
# Node ID c103a3dec114d882c98382d684d8af798d09d857
# Parent  0000000000000000000000000000000000000000
1

diff -r 000000000000 -r c103a3dec114 a
--- /dev/null	Thu Jan 01 00:00:00 1970 +0000
+++ b/a	Thu Jan 01 00:00:00 1970 +0000
@@ -0,0 +1,1 @@
+1
"#;

#[test]
fn test_basic_cstringio() {
    let mut c = common::TestClient::new("import_basic_cstringio", &[]);
    hg!(c.client, import, patches = &[PATCH]).unwrap();
    let a = hg!(c.client, cat, files = &["a"]).unwrap().unwrap();
    assert_eq!(String::from_utf8(a).unwrap(), "1\n");
}

#[test]
fn test_basic_file() {
    let mut c = common::TestClient::new("import_basic_file", &[]);
    c.write("patch", PATCH);

    assert!(hg!(c.client, import, patches = &["patch"], nocommit = true).is_ok());
    assert_eq!(c.read("a"), "1\n");

    assert!(hg!(c.client, update, clean = true).is_ok());
    c.rm("a");

    assert!(hg!(c.client, import, patches = &["patch"]).is_ok());
    let a = hg!(c.client, cat, files = &["a"]).unwrap().unwrap();
    assert_eq!(String::from_utf8(a).unwrap(), "1\n");
}

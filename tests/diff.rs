// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{add, commit, diff, hg};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("diff_basic", &[]);
    c.append(".hg/hgrc", &["[diff]", "git = 0"]);
    c.reopen();

    c.append("a", &["a\n"]);
    hg!(c.client, add, files = &["a"]).unwrap();
    let diff1 = "diff -r 000000000000 a
--- /dev/null
+++ b/a
@@ -0,0 +1,1 @@
+a
";
    let diff = hg!(c.client, diff, nodates = true).unwrap();
    let diff = String::from_utf8(diff).unwrap();
    assert_eq!(diff, diff1);

    let diff = hg!(c.client, diff, files = &["a"], nodates = true).unwrap();
    let diff = String::from_utf8(diff).unwrap();
    assert!(diff == diff1);

    let rev0 = hg!(c.client, commit, message = "first").unwrap();
    let diff2 = "diff -r 000000000000 -r ".to_string()
        + &rev0.node[..12]
        + " a
--- /dev/null
+++ b/a
@@ -0,0 +1,1 @@
+a
";
    let diff = hg!(
        c.client,
        diff,
        change = &rev0.rev.to_string(),
        nodates = true
    )
    .unwrap();
    let diff = String::from_utf8(diff).unwrap();
    assert_eq!(diff2, diff);

    c.append("a", &["a\n"]);
    let rev1 = hg!(c.client, commit, message = "second").unwrap();

    let diff3 = "diff -r ".to_string()
        + &rev0.node[..12]
        + " a
--- a/a
+++ b/a
@@ -1,1 +1,2 @@
 a
+a
";

    let diff = hg!(
        c.client,
        diff,
        revs = &[&rev0.rev.to_string()],
        nodates = true
    )
    .unwrap();
    let diff = String::from_utf8(diff).unwrap();
    assert_eq!(diff3, diff);

    let diff4 = "diff -r ".to_string()
        + &rev0.node[..12]
        + " -r "
        + &rev1.node[..12]
        + " a
--- a/a
+++ b/a
@@ -1,1 +1,2 @@
 a
+a
";

    let diff = hg!(
        c.client,
        diff,
        revs = &[&rev0.rev.to_string(), &rev1.rev.to_string()],
        nodates = true
    )
    .unwrap();
    let diff = String::from_utf8(diff).unwrap();
    assert_eq!(diff4, diff);
}

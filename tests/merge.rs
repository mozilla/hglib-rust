// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{commit, diff, hg, merge, update};
use std::str;

mod common;

fn set_up(c: &mut common::TestClient) -> (String, String) {
    c.append("a", &["a"]);
    let first_commit = hg!(c.client, commit, message = "first", addremove = true).unwrap();

    c.append("a", &["a"]);
    let second_commit = hg!(c.client, commit, message = "change").unwrap();

    (first_commit.node, second_commit.node)
}

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("merge_basic", &[]);
    let (node0, node1) = set_up(&mut c);

    assert!(hg!(c.client, update, rev = node0.as_str()).is_ok());
    c.append("b", &["a"]);
    let commit2 = hg!(c.client, commit, message = "new file", addremove = true).unwrap();
    assert!(hg!(c.client, merge, rev = &node1).is_ok());
    let commit = hg!(c.client, commit, message = "merge").unwrap();

    let expected = format!(
        r#"diff -r {node2} -r {node} a
--- a/a
+++ b/a
@@ -1,1 +1,1 @@
-a
\ No newline at end of file
+aa
\ No newline at end of file
"#,
        node2 = &commit2.node[0..12],
        node = &commit.node[0..12]
    );

    let diff_bytes = hg!(
        c.client,
        diff,
        change = commit.node.as_str(),
        nodates = true
    )
    .unwrap();
    let diff = str::from_utf8(&diff_bytes).unwrap();

    assert_eq!(diff, expected)
}

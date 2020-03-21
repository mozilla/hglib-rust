// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

mod common;

#[test]
fn test_encoding() {
    let c = common::TestClient::new("encoding", &[]);
    assert_eq!(c.client.encoding(), "UTF-8");
}

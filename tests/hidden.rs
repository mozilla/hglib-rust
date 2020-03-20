// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{add, commit, hg, log, update, version, Runner};

mod common;

fn set_up(c: &mut common::TestClient) {
    c.append(
        ".hg/obs.py",
        &[
            "import mercurial.obsolete",
            "mercurial.obsolete._enabled = True",
            "mercurial.obsolete.isenabled = lambda r, opt: True",
        ],
    );
    c.append(".hg/hgrc", &["", "[extensions]", "obs=.hg/obs.py"]);
    c.reopen();
}

#[test]
fn test_debugobsolete_failure() {
    let mut c = common::TestClient::new("hidden_debugobsolete_failure", &[]);
    c.append("gna1", &["g"]);
    assert!(hg!(c.client, add, files = &["gna1"]).is_ok());
    let rev0 = hg!(c.client, commit, message = "gna1").unwrap();
    let node0 = rev0.node;

    assert!(c.client.runcommand(&["debugobsolete", &node0], None).is_err());
}

#[test]
fn test_debugobsolete_success() {
    let mut c = common::TestClient::new("hidden_debugobsolete_success", &[]);
    set_up(&mut c);
    c.append("gna1", &["ga"]);
    assert!(hg!(c.client, add, files = &["gna1"]).is_ok());
    let rev0 = hg!(c.client, commit, message = "gna1").unwrap();
    let node0 = rev0.node;

    c.client.runcommand(&["debugobsolete", &node0], None).unwrap();
}

#[test]
fn test_obsolete_in() {
    let mut c = common::TestClient::new("hidden_obsolete_in", &[]);
    let version = hg!(c.client, version).unwrap();
    if version.major < 2 || (version.major == 2 && version.minor < 9) {
        return;
    }

    set_up(&mut c);

    c.append("gna1", &["ga"]);
    assert!(hg!(c.client, add, files = &["gna1"]).is_ok());
    let rev0 = hg!(c.client, commit, message = "gna1").unwrap();
    let node0 = rev0.node;

    c.append("gna2", &["gaaa"]);
    assert!(hg!(c.client, add, files = &["gna2"]).is_ok());
    let rev1 = hg!(c.client, commit, message = "gna2").unwrap();
    let node1 = rev1.node;
    c.client.runcommand(&["debugobsolete", &node1], None).unwrap();
    assert!(hg!(c.client, update, rev = &node0).is_ok());

    assert!(hg!(c.client, log, revrange = &[&node1]).is_err());
    assert!(hg!(c.client, log, revrange = &[&node0]).is_ok());
}

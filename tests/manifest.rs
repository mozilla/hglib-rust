// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::{
    commit, hg,
    manifest::{self, File, Manifest},
};

mod common;

#[test]
fn test_basic() {
    let mut c = common::TestClient::new("manifest_basic", &[]);
    c.append("a", &["a"]);

    let mut files = vec!["a"];
    let mut manifest = vec![File {
        node: "047b75c6d7a3ef6a2243bd0e99f94f6ea6683597".to_string(),
        perm: "644".to_string(),
        symlink: false,
        executable: false,
        filename: "a".to_string(),
    }];

    if cfg!(unix) {
        use std::os::unix::fs::{self, PermissionsExt};
        
        c.append("b", &["b"]);
        let metadata = std::fs::metadata("b").unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions("b", permissions).unwrap();
        
        fs::symlink("b", "c").unwrap();

        files.extend_from_slice(&["b", "c"]);
        
        manifest.push(File {
            node: "62452855512f5b81522aa3895892760bb8da9f3f".to_string(),
            perm: "755".to_string(),
            symlink: false,
            executable: true,
            filename: "b".to_string(),
        });
        
        manifest.push(File {
            node: "62452855512f5b81522aa3895892760bb8da9f3f".to_string(),
            perm: "644".to_string(),
            symlink: true,
            executable: false,
            filename: "c".to_string(),
        });
    }

    assert!(hg!(c.client, commit, message = "first", addremove = true).is_ok());

    if let Manifest::All(results) = hg!(c.client, manifest, all = true).unwrap() {
        assert_eq!(results, files);
    } else {
        unreachable!();
    }

    if let Manifest::Info(results) = hg!(c.client, manifest).unwrap() {
        assert_eq!(results, manifest);
    } else {
        unreachable!();
    }
}

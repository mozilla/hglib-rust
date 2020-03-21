// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate hglib;

use crate::hglib::version::Version;

#[test]
fn test_compare() {
    let x = Version::from_tuple(&(1, 2, Some(3)));
    //left > right
    assert!(x > Version::from_tuple(&(0, 5, Some(5))));
    assert!(x > Version::from_tuple(&(1, 0, Some(5))));
    assert!(x > Version::from_tuple(&(1, 2, Some(0))));
    assert!(x > Version::from_tuple(&(1, 2, None)));

    //left < right
    assert!(Version::from_tuple(&(0, 5, Some(5))) < x);
    assert!(Version::from_tuple(&(1, 0, Some(5))) < x);
    assert!(Version::from_tuple(&(1, 2, Some(0))) < x);
    assert!(Version::from_tuple(&(1, 2, None)) < x);

    let x = Version::from_tuple(&(1, 2, None));
    assert!(!(Version::from_tuple(&(1, 2, Some(0))) < x));
    assert!(!(Version::from_tuple(&(1, 2, Some(0))) > x));
}

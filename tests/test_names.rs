extern crate langgen;

mod common;

use crate::common::DebugObject;
use langgen::*;

#[test]
fn short_name() {
    let adam = DebugObject::adam();
    assert_eq!(adam.short_name(), "Adam");

    let eva = DebugObject::eva();
    assert_eq!(eva.short_name(), "Eva");
}

#[test]
fn long_name() {
    let adam = DebugObject::adam();
    assert_eq!(adam.long_name(), "Adam Evasman");

    let eva = DebugObject::eva();
    assert_eq!(eva.long_name(), "Eva Adamsfru");
}

extern crate langgen;
mod common;

use common::DebugActor;
use langgen::*;

#[test]
fn short_name() {
    let ove = DebugActor::new("!Ove, !Ove Svensson");
    assert_eq!(ove.short_name(), "Ove");

    let eva = DebugActor::new("Eva, Eva Stinasson");
    assert_eq!(eva.short_name(), "Eva");
}

#[test]
fn long_name() {
    let ove = DebugActor::new("Ove, Ove Svensson");
    assert_eq!(ove.long_name(), "Ove Svensson");

    let eva = DebugActor::new("Eva, Eva Stinasson");
    assert_eq!(eva.long_name(), "Eva Stinasson");
}

extern crate langgen;
mod common;

use common::*;
use langgen::*;

#[test]
fn test_debug_output() {
    let mut out = DebugOutput::new();
    out.write_style("bold");
    out.write_text("test");
    out.write_style("/bold");
    out.done();
    assert_eq!(out.last_text, "<bold>test</bold>");
    assert_eq!(out.text, "");
}

#[test]
fn test_out() {
    let mut out = DebugOutput::new();
    out.out().s(" is").s(" good.");
    assert_eq!(out.last_text, " is good.");
    assert_eq!(out.text, "");
}

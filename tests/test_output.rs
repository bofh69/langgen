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
    let ove = DebugActor::new("!Ove, !Ove Svensson");
    let apple = DebugActor::new("apple, red apple");

    out.out().the(&ove).s(" has ").a(&apple).s(".");
    assert_eq!(out.last_text, "Ove has an apple.");
    assert_eq!(out.text, "");

    out.out().the_(&apple).s(" has ").a(&ove).s(".");
    assert_eq!(out.last_text, "the red apple has Ove.");
    assert_eq!(out.text, "");

    out.can_see = false;
    out.out().the(&ove).s(" has ").a(&apple).s(".");
    assert_eq!(out.last_text, "someone has something.");
    assert_eq!(out.text, "");

    out.out().the_(&apple).s(" has ").a(&ove).s(".");
    assert_eq!(out.last_text, "something has someone.");
    assert_eq!(out.text, "");
}

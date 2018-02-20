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
    let ove = DebugObject::new("!Ove, !Ove Svensson");
    let apple = DebugObject::new("apple, red apple");

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

#[test]
fn test_verb_endings() {
    let mut out = DebugOutput::new();
    let ove = DebugObject::new("!Ove, !Ove Svensson");
    let apple = DebugObject::new("apple, red apple");
    let mut apples = DebugObject::new("apples, red apples");
    apples.gender = Gender::Plural;

    out.out().the(&ove).v_e(&ove, " jump").s(".");
    assert_eq!(out.last_text, "Ove jumps.");
    assert_eq!(out.text, "");

    out.out().the(&apple).v_e(&apple, " fall").s(".");
    assert_eq!(out.last_text, "the apple falls.");
    assert_eq!(out.text, "");

    out.out().the(&apples).v_e(&apples, " fall").s(".");
    assert_eq!(out.last_text, "the apples fall.");
    assert_eq!(out.text, "");

    out.can_see = false;

    out.out().the(&ove).v_e(&ove, " jump").s(".");
    assert_eq!(out.last_text, "someone jumps.");
    assert_eq!(out.text, "");

    out.out().the(&apple).v_e(&apple, " fall").s(".");
    assert_eq!(out.last_text, "something falls.");
    assert_eq!(out.text, "");

    out.can_see = true;

}

#[test]
fn test_out_for_me() {
    let mut out = DebugOutput::new();

    let ove = DebugObject::new("!Ove, !Ove Svensson");

    out.me = true;

    out.out().the(&ove).s(" win.");
    assert_eq!(out.last_text, "you win.");
    assert_eq!(out.text, "");

    out.can_see = false;
    out.out().the(&ove).s(" win.");
    assert_eq!(out.last_text, "you win.");
    assert_eq!(out.text, "");
}

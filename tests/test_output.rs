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
fn test_out_a_and_the() {
    let mut out = DebugOutput::new();
    let ove = DebugObject::new("!Ove, !Ove Svensson");
    let apple = DebugObject::new("apple, red apple");
    let mut water = DebugObject::new("water, cold water");
    water.gender = Gender::Uncountable;

    out.out().the(&ove).s(" has ").a(&apple).s(".");
    assert_eq!(out.last_text, "Ove has an apple.");
    assert_eq!(out.text, "");

    out.out().the_(&apple).s(" has ").a(&ove).s(".");
    assert_eq!(out.last_text, "The red apple has Ove.");
    assert_eq!(out.text, "");

    out.out().the(&water).s(" is cold.");
    assert_eq!(out.last_text, "The water is cold.");
    assert_eq!(out.text, "");

    out.out().a(&water);
    assert_eq!(out.last_text, "Water");
    assert_eq!(out.text, "");

    out.can_see = false;
    out.out().the(&ove).s(" has ").a(&apple).s(".");
    assert_eq!(out.last_text, "Someone has something.");
    assert_eq!(out.text, "");

    out.out().the_(&apple).s(" has ").a(&ove).s(".");
    assert_eq!(out.last_text, "Something has someone.");
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
    assert_eq!(out.last_text, "The apple falls.");
    assert_eq!(out.text, "");

    out.out().the(&apples).v_e(&apples, " fall").s(".");
    assert_eq!(out.last_text, "The apples fall.");
    assert_eq!(out.text, "");

    out.can_see = false;

    out.out().the(&ove).v_e(&ove, " jump").s(".");
    assert_eq!(out.last_text, "Someone jumps.");
    assert_eq!(out.text, "");

    out.out().the(&apple).v_e(&apple, " fall").s(".");
    assert_eq!(out.last_text, "Something falls.");
    assert_eq!(out.text, "");

    out.can_see = true;
}

#[test]
fn test_out_for_me() {
    let mut out = DebugOutput::new();

    let ove = DebugObject::new("!Ove, !Ove Svensson");

    out.me = true;

    out.out().the(&ove).v_e(&ove, " win");
    assert_eq!(out.last_text, "You win");
    assert_eq!(out.text, "");

    out.can_see = false;
    out.out().the(&ove).v_e(&ove, " win");
    assert_eq!(out.last_text, "You win");
    assert_eq!(out.text, "");
}

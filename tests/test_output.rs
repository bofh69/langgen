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
    let adam = DebugObject::adam();
    let apple = DebugObject::apple();
    let water = DebugObject::water();
    let apples = DebugObject::apples();

    out.out().the(&adam).s("has").a(&apple);
    assert_eq!(out.last_text, "Adam has an apple.");
    assert_eq!(out.text, "");

    out.out().the_(&apple).s("has").a(&adam);
    assert_eq!(out.last_text, "The green apple has Adam.");
    assert_eq!(out.text, "");

    out.out().the(&water).s("is cold");
    assert_eq!(out.last_text, "The water is cold.");
    assert_eq!(out.text, "");

    out.out().a(&water);
    assert_eq!(out.last_text, "Some water.");
    assert_eq!(out.text, "");

    out.out().a(&apples).s("and").a_(&apples);
    assert_eq!(out.last_text, "Some apples and some red apples.");
    assert_eq!(out.text, "");

    out.can_see = false;
    out.out().the(&adam).s("has").a(&apple);
    assert_eq!(out.last_text, "Someone has something.");
    assert_eq!(out.text, "");

    out.out().the_(&apple).s("has").a_(&adam);
    assert_eq!(out.last_text, "Something has someone.");
    assert_eq!(out.text, "");
}

#[test]
fn test_verb_endings() {
    let mut out = DebugOutput::new();
    let adam = DebugObject::adam();
    let apple = DebugObject::apple();
    let apples = DebugObject::apples();

    out.out().the(&adam).v_e(&adam, "jump").s("down");
    assert_eq!(out.last_text, "Adam jumps down.");
    assert_eq!(out.text, "");

    out.out().the(&apple).v_e(&apple, "fall");
    assert_eq!(out.last_text, "The apple falls.");
    assert_eq!(out.text, "");

    out.out().the(&apples).v_e(&apples, "fall");
    assert_eq!(out.last_text, "The apples fall.");
    assert_eq!(out.text, "");

    out.can_see = false;

    out.out().the(&adam).v_e(&adam, "jump");
    assert_eq!(out.last_text, "Someone jumps.");
    assert_eq!(out.text, "");

    out.out().the(&apple).v_e(&apple, "fall");
    assert_eq!(out.last_text, "Something falls.");
    assert_eq!(out.text, "");

    out.can_see = true;
}

#[test]
fn test_out_for_me() {
    let mut out = DebugOutput::new();

    let adam = DebugObject::adam();

    out.me = true;

    out.out().the(&adam).v_e(&adam, "win");
    assert_eq!(out.last_text, "You win.");
    assert_eq!(out.text, "");

    out.can_see = false;
    out.out().the(&adam).v_e(&adam, "win");
    assert_eq!(out.last_text, "You win.");
    assert_eq!(out.text, "");
}

#[test]
fn test_my() {
    let mut out = DebugOutput::new();
    let adam = DebugObject::adam();
    let eva = DebugObject::eva();
    let apple = DebugObject::apple();
    let apples = DebugObject::apples();

    out.out().my(&adam, &apple);
    assert_eq!(out.last_text, "His apple.");

    out.out().my(&eva, &apple);
    assert_eq!(out.last_text, "Her apple.");

    out.out().my(&apples, &apple);
    assert_eq!(out.last_text, "Their apple.");

    out.out().my(&apple, &apple);
    assert_eq!(out.last_text, "Its apple.");

    out.out().my_(&adam, &apple);
    assert_eq!(out.last_text, "His green apple.");

    out.out().my_(&eva, &apple);
    assert_eq!(out.last_text, "Her green apple.");

    out.out().my_(&apples, &apple);
    assert_eq!(out.last_text, "Their green apple.");

    out.out().my_(&apple, &apple);
    assert_eq!(out.last_text, "Its green apple.");

    out.can_see = false;
    out.out().my(&adam, &apple);
    assert_eq!(out.last_text, "Something.");

    out.out().my(&adam, &apples);
    assert_eq!(out.last_text, "Something.");

    out.out().my_(&adam, &apple);
    assert_eq!(out.last_text, "Something.");

    out.out().my_(&adam, &apples);
    assert_eq!(out.last_text, "Something.");
}

#[test]
fn test_is_are() {
    let mut out = DebugOutput::new();
    let adam = DebugObject::adam();
    let eva = DebugObject::eva();
    let apple = DebugObject::apple();
    let apples = DebugObject::apples();

    out.out().the(&adam).is(&adam);
    assert_eq!(out.last_text, "Adam is.");

    out.out().the(&eva).is(&eva);
    assert_eq!(out.last_text, "Eva is.");

    out.out().the(&apple).is(&apple);
    assert_eq!(out.last_text, "The apple is.");

    out.out().the(&apples).is(&apples);
    assert_eq!(out.last_text, "The apples are.");

    out.me = true;

    out.out().the(&adam).is(&adam);
    assert_eq!(out.last_text, "You are.");

    out.me = false;

    out.can_see = false;

    out.out().the(&adam).is(&adam);
    assert_eq!(out.last_text, "Someone is.");

    out.out().the(&apple).is(&apple);
    assert_eq!(out.last_text, "Something is.");

    out.out().the(&apples).is(&apples);
    assert_eq!(out.last_text, "Something is.");
}

#[test]
fn test_has_have() {
    let mut out = DebugOutput::new();
    let adam = DebugObject::adam();
    let eva = DebugObject::eva();
    let apple = DebugObject::apple();
    let apples = DebugObject::apples();

    out.out().the(&adam).has(&adam);
    assert_eq!(out.last_text, "Adam has.");

    out.out().the(&eva).has(&eva);
    assert_eq!(out.last_text, "Eva has.");

    out.out().the(&apple).has(&apple);
    assert_eq!(out.last_text, "The apple has.");

    out.out().the(&apples).has(&apples);
    assert_eq!(out.last_text, "The apples have.");

    out.me = true;

    out.out().the(&adam).has(&adam);
    assert_eq!(out.last_text, "You have.");

    out.me = false;

    out.can_see = false;

    out.out().the(&adam).has(&adam);
    assert_eq!(out.last_text, "Someone has.");

    out.out().the(&apple).has(&apple);
    assert_eq!(out.last_text, "Something has.");

    out.out().the(&apples).has(&apples);
    assert_eq!(out.last_text, "Something has.");
}

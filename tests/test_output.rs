extern crate langgen;

mod common;

use crate::common::*;
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

fn assert_call<F>(f: F, result: &[&str; 12])
where
    F: for<'a> Fn((OutputBuilder<'a>, &DebugObject)) -> OutputBuilder<'a>,
{
    let mut out = DebugOutput::new();
    let me = DebugObject::me();
    out.me = true;
    f((out.out(), &me));
    assert_eq!(out.last_text, format!("{}.", result[0]));
    out.me = false;

    let objs = &[
        me,
        DebugObject::adam(),
        DebugObject::apple(),
        DebugObject::dust(),
        DebugObject::apples(),
    ];
    for (o, res) in objs.iter().zip(result[1..].iter()) {
        f((out.out(), o));
        assert_eq!(
            out.last_text,
            format!("{}.", res),
            "{:?} should become {}",
            o.short_name(),
            res
        );
    }
    out.can_see = false;
    out.me = true;
    f((out.out(), &DebugObject::me()));
    assert_eq!(out.last_text, format!("{}.", result[6]));
    out.me = false;
    for (o, res) in objs.iter().zip(result[7..].iter()) {
        f((out.out(), o));
        assert_eq!(
            out.last_text,
            format!("{}.", res),
            "{:?} should become {}",
            o.short_name(),
            res
        );
    }
}

#[test]
fn test_out_a() {
    assert_call(
        |(ob, obj)| ob.a(obj),
        &[
            "You",
            "ItsMe",
            "Adam",
            "An apple",
            "Some dust",
            "Some apples",
            "You",
            "Someone",
            "Someone",
            "Something",
            "Something",
            "Something",
        ],
    );
}

#[test]
fn test_out_the() {
    assert_call(
        |(ob, obj)| ob.the(obj),
        &[
            "You",
            "ItsMe",
            "Adam",
            "The apple",
            "The dust",
            "The apples",
            "You",
            "Someone",
            "Someone",
            "Something",
            "Something",
            "Something",
        ],
    );
}

#[test]
fn test_out_a_and_the() {
    let mut out = DebugOutput::new();
    let adam = DebugObject::adam();
    let apple = DebugObject::apple();
    let dust = DebugObject::dust();
    let apples = DebugObject::apples();

    out.out().the(&adam).has(&adam).a(&apple);
    assert_eq!(out.last_text, "Adam has an apple.");
    assert_eq!(out.text, "");

    out.out().the_(&apple).has(&apple).a(&adam);
    assert_eq!(out.last_text, "The green apple has Adam.");
    assert_eq!(out.text, "");

    out.out().the(&dust).s("is cold");
    assert_eq!(out.last_text, "The dust is cold.");
    assert_eq!(out.text, "");

    out.out().a(&dust);
    assert_eq!(out.last_text, "Some dust.");
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
    let eva = DebugObject::eva();
    let apple = DebugObject::apple();
    let apples = DebugObject::apples();

    out.out().the(&adam).v_e(&adam, "jump").s("down");
    assert_eq!(out.last_text, "Adam jumps down.");
    assert_eq!(out.text, "");

    out.out().the(&adam).v_e(&adam, "SCREAM");
    assert_eq!(out.last_text, "Adam SCREAMS.");

    out.out().the(&eva).v_e(&eva, "kiss");
    assert_eq!(out.last_text, "Eva kisses.");

    out.out().the(&eva).v_e(&eva, "KISS");
    assert_eq!(out.last_text, "Eva KISSES.");

    out.out().the(&adam).v_e(&adam, "pry");
    assert_eq!(out.last_text, "Adam pries.");

    out.out().the(&adam).v_e(&adam, "PRY");
    assert_eq!(out.last_text, "Adam PRIES.");

    out.out().the(&adam).v_e(&adam, "ay");
    assert_eq!(out.last_text, "Adam ays.");

    /*
     * These become incorrect:
     *
    out.out().the(&adam).v_e(&adam, "volly");
    assert_eq!(out.last_text, "Adam vollies.");

    out.out().the(&adam).v_e(&adam, "VOLLY");
    assert_eq!(out.last_text, "Adam VOLLIES.");
    */

    out.out().the(&adam).v_e(&adam, "zzz");
    assert_eq!(out.last_text, "Adam zzzes.");

    out.out().the(&adam).v_e(&adam, "ZZZ");
    assert_eq!(out.last_text, "Adam ZZZES.");

    out.out().the(&adam).v_e(&adam, "o");
    assert_eq!(out.last_text, "Adam oes.");

    out.out().the(&adam).v_e(&adam, "O");
    assert_eq!(out.last_text, "Adam OES.");

    out.out().the(&adam).v_e(&adam, "xxx");
    assert_eq!(out.last_text, "Adam xxxes.");

    out.out().the(&adam).v_e(&adam, "XXX");
    assert_eq!(out.last_text, "Adam XXXES.");

    out.out().the(&adam).v_e(&adam, "ch");
    assert_eq!(out.last_text, "Adam ches.");

    out.out().the(&adam).v_e(&adam, "CH");
    assert_eq!(out.last_text, "Adam CHES.");

    out.out().the(&adam).v_e(&adam, "sh");
    assert_eq!(out.last_text, "Adam shes.");

    out.out().the(&adam).v_e(&adam, "SH");
    assert_eq!(out.last_text, "Adam SHES.");

    out.out().the(&adam).v_e(&adam, "ah");
    assert_eq!(out.last_text, "Adam ahs.");

    out.out().the(&adam).v_e(&adam, "AH");
    assert_eq!(out.last_text, "Adam AHS.");

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

#[test]
fn test_thes() {
    let mut out = DebugOutput::new();
    let adam = DebugObject::adam();
    let eva = DebugObject::eva();
    let apple = DebugObject::apple();
    let apples = DebugObject::apples();

    out.out().thes(&adam);
    assert_eq!(out.last_text, "Adam's.");
    out.out().thes(&eva);
    assert_eq!(out.last_text, "Eva's.");
    out.out().thes(&apple);
    assert_eq!(out.last_text, "The apple's.");
    out.out().thes(&apples);
    assert_eq!(out.last_text, "The apples'.");

    out.can_see = false;

    out.out().thes(&adam);
    assert_eq!(out.last_text, "Someone's.");
    out.out().thes(&eva);
    assert_eq!(out.last_text, "Someone's.");
    out.out().thes(&apple);
    assert_eq!(out.last_text, "Something's.");
    out.out().thes(&apples);
    assert_eq!(out.last_text, "Something's.");

    out.can_see = true;
    out.me = true;

    out.out().thes(&adam);
    assert_eq!(out.last_text, "Your.");
}

#[test]
fn test_thes_() {
    let mut out = DebugOutput::new();
    let adam = DebugObject::adam();
    let eva = DebugObject::eva();
    let apple = DebugObject::apple();
    let apples = DebugObject::apples();

    out.out().thes_(&adam);
    assert_eq!(out.last_text, "Adam Evasman's.");
    out.out().thes_(&eva);
    assert_eq!(out.last_text, "Eva Adamsfru's.");
    out.out().thes_(&apple);
    assert_eq!(out.last_text, "The green apple's.");
    out.out().thes_(&apples);
    assert_eq!(out.last_text, "The red apples'.");

    out.can_see = false;

    out.out().thes_(&adam);
    assert_eq!(out.last_text, "Someone's.");
    out.out().thes_(&eva);
    assert_eq!(out.last_text, "Someone's.");
    out.out().thes_(&apple);
    assert_eq!(out.last_text, "Something's.");
    out.out().thes_(&apples);
    assert_eq!(out.last_text, "Something's.");

    out.can_see = true;
    out.me = true;

    out.out().thes_(&adam);
    assert_eq!(out.last_text, "Your.");
}

#[test]
fn test_suppress_dot() {
    let mut out = DebugOutput::new();

    out.out().s("hello").suppress_dot();
    assert_eq!(out.last_text, "Hello");
}

extern crate langgen;

use langgen::*;

struct DebugOutput {
    text: String,
    last_text: String,
}

impl ::Viewer for DebugOutput {
    fn can_see(&self, _who: &Actor) -> bool {
        true
    }

    // Ie the viewer can "hear" Actor.
    fn can(&self, _verb: &str, _who: &Actor) -> bool {
        true
    }

    // Ie The viewer has the see_curse property?
    fn has(&self, _property: &str) -> bool {
        true
    }
}

impl Output for DebugOutput {
    fn write_text(&mut self, text: &str) {
        self.text += text;
    }

    fn write_style(&mut self, style: &str) {
        self.text += "<";
        self.text += style;
        self.text += ">";
    }

    fn done(&mut self) {
        self.last_text.clear();
        self.last_text += &self.text;
        self.text.clear();
    }
}

impl DebugOutput {
    fn new() -> DebugOutput {
        DebugOutput {
            text: String::new(),
            last_text: String::new(),
        }
    }
}

/*
struct DebugActor {
    named: NamedImpl,
}

impl DebugActor {
    fn new(name: String) -> DebugActor {
        DebugActor { named: NamedImpl::new(name) }
    }
}

impl Viewer for DebugActor {
    fn can_see(&self, _who: &Actor) -> bool {
        true
    }

    fn can(&self, _verb: &str, _who: &Actor) -> bool {
        true
    }

    fn has(&self, _property: &str) -> bool {
        true
    }
}

impl Named for DebugActor {
    fn gender(&self) -> Gender {
        self.named.gender()
    }

    fn is_short_proper(&self) -> bool {
        self.named.is_short_proper()
    }

    fn short_name(&self) -> &str {
        self.named.short_name()
    }

    fn is_long_proper(&self) -> bool {
        self.named.is_long_proper()
    }

    fn long_name(&self) -> &str {
        self.named.long_name()
    }

    fn is_short_plural_proper(&self) -> bool {
        self.named.is_short_plural_proper()
    }

    fn short_plural_name(&self) -> &str {
        self.named.short_plural_name()
    }

    fn is_long_plural_proper(&self) -> bool {
        self.named.is_long_plural_proper()
    }

    fn long_plural_name(&self) -> &str {
        self.named.long_plural_name()
    }
}
*/

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

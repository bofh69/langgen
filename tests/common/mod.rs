extern crate langgen;
extern crate std;
use langgen::*;

pub struct DebugActor {
    named: Box<Named>,
}

impl DebugActor {
    pub fn new(name: &str) -> DebugActor {
        let mut buff = std::io::Cursor::new("man:men\n");
        let nf = NamedFactory::new(&mut buff);
        DebugActor { named: nf.create(name) }
    }
}

impl Actor for DebugActor {}

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

pub struct DebugOutput {
    pub text: String,
    pub last_text: String,
    pub can_see: bool,
}

impl ::Viewer for DebugOutput {
    fn can_see(&self, _who: &Actor) -> bool {
        self.can_see
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

    fn out(&mut self) -> OutputBuilder {
        OutputBuilder::new(self as &mut Output)
    }
}

impl DebugOutput {
    pub fn new() -> DebugOutput {
        DebugOutput {
            text: String::new(),
            last_text: String::new(),
            can_see: true,
        }
    }
}

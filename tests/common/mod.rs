extern crate langgen;
extern crate std;
use langgen::*;

pub struct DebugObject {
    named: Box<Named>,
    pub me: bool,
}

impl DebugObject {
    pub fn new(name: &str, sex: Gender, thing: bool) -> DebugObject {
        let mut buff = std::io::Cursor::new("man:men\n");
        let nf = NamedFactory::new(&mut buff);
        DebugObject {
            named: nf.create(name, sex, thing),
            me: false,
        }
    }
    pub fn adam() -> Self {
        DebugObject::new("!Adam, !Adam Evasman", Gender::Male, false)
    }

    pub fn eva() -> Self {
        DebugObject::new("!Eva, !Eva Adamsfru", Gender::Female, false)
    }

    pub fn apple() -> Self {
        DebugObject::new("apple, green apple", Gender::Neuter, true)
    }

    pub fn apples() -> Self {
        DebugObject::new("apples, red apples", Gender::Plural, true)
    }

    pub fn knife() -> Self {
        DebugObject::new("knife, dull knife", Gender::Neuter, true)
    }

    pub fn water() -> Self {
        DebugObject::new("water, cold water", Gender::Uncountable, true)
    }
}

impl Object for DebugObject {}

impl Viewer for DebugObject {
    fn can_see(&self, _who: &Object) -> bool {
        true
    }

    fn can(&self, _verb: &str, _who: &Object) -> bool {
        true
    }

    fn has(&self, _property: &str) -> bool {
        true
    }

    fn is_me(&self, _who: &Object) -> bool {
        self.me
    }
}

impl Named for DebugObject {
    fn gender(&self) -> Gender {
        self.named.gender()
    }

    fn is_thing(&self) -> bool {
        self.named.is_thing()
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
    pub me: bool,
}

impl crate::Viewer for DebugOutput {
    fn can_see(&self, _who: &Object) -> bool {
        self.can_see
    }

    // Ie the viewer can "hear" Object.
    fn can(&self, _verb: &str, _who: &Object) -> bool {
        true
    }

    // Ie The viewer has the see_curse property?
    fn has(&self, _property: &str) -> bool {
        true
    }

    fn is_me(&self, _who: &Object) -> bool {
        self.me
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
            me: false,
        }
    }
}

extern crate langgen;
extern crate std;
use langgen::*;

pub struct DebugObject {
    named: Box<Named>,
}

#[allow(dead_code)]
impl DebugObject {
    pub fn new(name: &str, sex: Gender, thing: bool) -> Self {
        use crate::named::Factory;

        let mut buff = std::io::Cursor::new("man:men\n");
        let nf = Factory::from_reader(&mut buff).unwrap();
        Self {
            named: nf.create(name, sex, thing),
        }
    }

    pub fn me() -> Self {
        Self::new("!ItsMe, !It Is Me", Gender::Male, false)
    }

    pub fn adam() -> Self {
        Self::new("!Adam, !Adam Evasman", Gender::Male, false)
    }

    pub fn eva() -> Self {
        Self::new("!Eva, !Eva Adamsfru", Gender::Female, false)
    }

    pub fn apple() -> Self {
        Self::new("apple, green apple", Gender::Neuter, true)
    }

    pub fn apples() -> Self {
        Self::new("apples, red apples", Gender::Plural, true)
    }

    pub fn knife() -> Self {
        Self::new("knife, dull knife", Gender::Neuter, true)
    }

    pub fn dust() -> Self {
        Self::new("dust, gold dust", Gender::Uncountable, true)
    }
}

impl Object for DebugObject {}

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

#[allow(dead_code)]
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

#[allow(dead_code)]
impl DebugOutput {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            last_text: String::new(),
            can_see: true,
            me: false,
        }
    }
}

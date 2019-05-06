pub mod named;
mod suffix;
pub mod templates;

/// The gender of Named:s.
#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
    Neuter,
    Plural,
    Uncountable,
}

/// Names of objects.
pub trait Named {
    /// The gender of the Named.
    fn gender(&self) -> Gender;

    fn is_thing(&self) -> bool;

    fn is_short_proper(&self) -> bool;
    fn short_name(&self) -> &str;
    fn is_long_proper(&self) -> bool;
    fn long_name(&self) -> &str;

    fn is_short_plural_proper(&self) -> bool;
    fn short_plural_name(&self) -> &str;
    fn is_long_plural_proper(&self) -> bool;
    fn long_plural_name(&self) -> &str;
}

/**
 * Viewer's answer if they see, hear etc other objects.
 * It also tells about properties of the Viewer.
 */
pub trait Viewer {
    /// Can this Viewer see who?
    fn can_see(&self, who: &Object) -> bool;

    /// Can this Viewer "verb" who?
    fn can(&self, verb: &str, who: &Object) -> bool;

    /// Does the Viewer have "property"?
    fn has(&self, property: &str) -> bool;

    /// Is who this viewer?
    fn is_me(&self, who: &Object) -> bool;
}

/// An Object is an object or subject in templates.
pub trait Object: Named {}

/**
 * The Output trait is used for objects that templates
 * can be rendered to.
 */
pub trait Output: Viewer {
    fn write_text(&mut self, text: &str);
    fn write_style(&mut self, style: &str);
    fn done(&mut self);
    fn out(&mut self) -> OutputBuilder;
}

fn last_char(s: &str) -> Option<char> {
    s.chars().rev().next()
}

fn needs_dot(s: &str) -> bool {
    if let Some(c) = last_char(s) {
        match c {
            '.' | '?' | '!' | ':' | ';' | '"' => false,
            _ => true,
        }
    } else {
        false
    }
}

// Used to decide between a/an.
fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        // y is usually not pronounced like a vowel.
        _ => false,
    }
}

fn uppercase_first_char(s: &str, to: &mut String) {
    let mut c = s.chars();
    if let Some(ch) = c.next() {
        for ch in ch.to_uppercase() {
            to.push(ch);
        }
        to.push_str(c.as_str());
    }
}

/// An Output that just throws away the text.
pub struct NullOutput {}

impl Output for NullOutput {
    /// Does nothing.
    fn write_text(&mut self, _: &str) {}
    /// Does nothing.
    fn write_style(&mut self, _: &str) {}
    /// Does nothing.
    fn done(&mut self) {}
    /// Returns an OutputBuilder for self.
    fn out(&mut self) -> OutputBuilder {
        OutputBuilder::new(self as &mut Output)
    }
}

impl Viewer for NullOutput {
    /// Always returns false.
    fn can_see(&self, _: &Object) -> bool {
        false
    }

    /// Always returns false.
    fn can(&self, _: &str, _: &Object) -> bool {
        false
    }

    /// Always returns false.
    fn has(&self, _: &str) -> bool {
        false
    }

    /// Always returns false.
    fn is_me(&self, _: &Object) -> bool {
        false
    }
}

/// `OutputBuilder` helps with the fluent interface for `Output::out()`
pub struct OutputBuilder<'a> {
    o: &'a mut Output,
    s: String,
    cap_it: bool,
    add_space: bool,
    suppress_dot: bool,
}

/// Calles `Output::done()`.
impl<'a> Drop for OutputBuilder<'a> {
    fn drop(&mut self) {
        self.o.write_text(&self.s);
        if !self.suppress_dot && needs_dot(&self.s) {
            self.o.write_text(".");
        }
        self.o.done();
    }
}

impl<'a> OutputBuilder<'a> {
    /// Creates a new OutputBuilder that will output
    /// the text to Output.
    pub fn new(o: &'a mut Output) -> OutputBuilder<'a> {
        OutputBuilder {
            o,
            s: String::new(),
            cap_it: true,
            add_space: false,
            suppress_dot: false,
        }
    }

    /// Is this gender supposed to be treated as singular?
    fn is_singular(gender: Gender) -> bool {
        match gender {
            Gender::Plural | Gender::Uncountable => false,
            _ => true,
        }
    }

    /// The next thing that is output should not be capitalized.
    pub fn dont_capitalize(mut self) -> Self {
        self.cap_it = false;
        self
    }

    /// The next thing that is output should be capitalized.
    pub fn do_capitalize(mut self) -> Self {
        self.cap_it = true;
        self
    }

    /// Send the text to the Output.
    /// The text is capitalized as needed.
    pub fn s(mut self, text: &str) -> Self {
        if self.add_space {
            self.s.push(' ');
        }
        self.add_space = true;
        if self.cap_it {
            self.cap_it = false;
            uppercase_first_char(text, &mut self.s);
            self
        } else {
            self.s.push_str(text);
            self
        }
    }

    fn add_s(str: &mut String) {
        let mut add: &str = "";
        let mut uc = false;
        let mut remove = 0;

        {
            let mut ci = str.chars().rev();
            if let Some(ch) = ci.next() {
                if ch.is_uppercase() {
                    uc = true;
                }
                add = match ch {
                    's' | 'o' | 'z' | 'x' | 'S' | 'O' | 'Z' | 'X' => "es",
                    'y' | 'Y' => {
                        remove = 1;
                        if let Some(c2) = ci.next() {
                            if !is_vowel(c2) {
                                "ies"
                            } else {
                                remove = 0;
                                "s"
                            }
                        } else {
                            "ies"
                        }
                    }
                    'h' | 'H' => {
                        if let Some(c2) = ci.next() {
                            if c2 == 'c' || c2 == 's' || c2 == 'C' || c2 == 'S' {
                                "es"
                            } else {
                                "s"
                            }
                        } else {
                            "s"
                        }
                    }
                    _ => "s",
                }
            }
        }
        while remove > 0 {
            str.pop();
            remove -= 1;
        }
        if uc {
            str.push_str(&add.to_uppercase());
        } else {
            str.push_str(add);
        }
    }

    /// Send the verb to the Output.
    /// Appends 's' at the end of it, if needed.
    pub fn v_e<T>(mut self, obj: &T, verb: &str) -> Self
    where
        T: Object,
    {
        if self.cap_it {
            unimplemented!();
        }
        if self.add_space {
            self.s.push(' ');
        }
        self.s.push_str(verb);
        self.cap_it = false;
        self.add_space = false;
        if Self::is_singular(obj.gender()) && !self.o.is_me(obj) {
            Self::add_s(&mut self.s);
        }
        self.s("")
    }

    fn add_the_word<T>(mut self, obj: &T, name: &str, is_proper: bool) -> Self
    where
        T: Object,
    {
        if self.o.is_me(obj) {
            self.s("you")
        } else if self.o.can_see(obj) {
            if !is_proper {
                self = self.s("the");
            }
            self.s(name)
        } else if is_proper {
            self.s("someone")
        } else {
            self.s("something")
        }
    }

    /// Sends "the object-short-name" to Output.
    /// If the viewer can't see it, someone/something is sent instead.
    /// The text is capitalized as needed.
    pub fn the<T: Object>(self, obj: &T) -> Self {
        self.add_the_word(obj, obj.short_name(), obj.is_short_proper())
    }

    /// Sends "the object-long-name" to Output.
    /// If the viewer can't see it, someone/something is sent instead.
    /// The text is capitalized as needed.
    pub fn the_<T: Object>(self, obj: &T) -> Self {
        self.add_the_word(obj, obj.long_name(), obj.is_long_proper())
    }

    fn add_a_word<T>(mut self, obj: &T, name: &str, is_prop: bool) -> Self
    where
        T: Object,
    {
        if self.o.is_me(obj) {
            self.s("you")
        } else if self.o.can_see(obj) {
            if !is_prop && Self::is_singular(obj.gender()) {
                let mut should_be_an = false;
                if let Some(c) = name.chars().next() {
                    if is_vowel(c) {
                        should_be_an = true;
                    }
                }
                self = if should_be_an {
                    self.s("an")
                } else {
                    self.s("a")
                }
            } else if !is_prop {
                self = self.s("some");
            }
            self.s(name)
        } else if is_prop {
            self.s("someone")
        } else {
            self.s("something")
        }
    }

    /// Sends "a/an object-short-name" to Output.
    /// If the viewer can't see it, someone/something is sent instead.
    /// The text is capitalized as needed.
    pub fn a<T: Object>(self, obj: &T) -> Self {
        self.add_a_word(obj, obj.short_name(), obj.is_short_proper())
    }

    /// Sends "a/an object-long-name" to Output.
    /// If the viewer can't see it, someone/something is sent instead.
    /// The text is capitalized as needed.
    pub fn a_<T: Object>(self, obj: &T) -> Self {
        self.add_a_word(obj, obj.long_name(), obj.is_long_proper())
    }

    /// Sends "my/his/her/their/its object-short-name" to Output.
    /// If the viewer can't see it, a() is used instead.
    /// The text is capitalized as needed.
    pub fn my<TW, TO>(mut self, who: &TW, obj: &TO) -> Self
    where
        TW: Object,
        TO: Object,
    {
        if self.o.is_me(who) {
            self = self.s("your").s(obj.short_name());
        } else if self.o.can_see(who) {
            let pron = match who.gender() {
                Gender::Male => "his",
                Gender::Female => "her",
                Gender::Plural => "their",
                _ => "its",
            };
            self = self.s(pron).s(obj.short_name());
        } else {
            self = self.a(obj);
        }
        self
    }

    /// Sends "my/his/her/their/its object-long-name" to Output.
    /// If the viewer can't see it, a() is used instead.
    /// The text is capitalized as needed.
    pub fn my_<TW, TO>(mut self, who: &TW, obj: &TO) -> Self
    where
        TW: Object,
        TO: Object,
    {
        if self.o.is_me(who) {
            self = self.s("your").s(obj.long_name());
        } else if self.o.can_see(who) {
            let pron = match who.gender() {
                Gender::Male => "his",
                Gender::Female => "her",
                Gender::Plural => "their",
                _ => "its",
            };
            self = self.s(pron).s(obj.long_name());
        } else {
            self = self.a_(obj);
        }
        self
    }

    fn sing_plur<T>(self, who: &T, singular: &str, plural: &str) -> Self
    where
        T: Object,
    {
        let mut g = who.gender();
        if self.o.is_me(who) {
            g = Gender::Plural;
        } else if !self.o.can_see(who) {
            g = Gender::Male;
        }
        self.s(match g {
            Gender::Plural => plural,
            _ => singular,
        })
    }

    /// Sends "is"/"are" to Output.
    /// The text is capitalized as needed.
    pub fn is<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        self.sing_plur(who, "is", "are")
    }

    /// Sends "has"/"have" to Output.
    /// The text is capitalized as needed.
    pub fn has<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        self.sing_plur(who, "has", "have")
    }

    /// Sends "the <object-short-name>'s" to Output.
    /// If the viewer can't see it, "something's"/"someone's" is used.
    /// The text is capitalized as needed.
    pub fn thes<T>(self, obj: &T) -> Self
    where
        T: Object,
    {
        if self.o.is_me(obj) {
            self.s("your")
        } else if self.o.can_see(obj) {
            if let Some(ch) = obj.short_name().chars().rev().next() {
                let uc = ch.is_uppercase();
                let add = match ch {
                    's' | 'S' => "'",
                    _ => {
                        if uc {
                            "'S"
                        } else {
                            "'s"
                        }
                    }
                };
                let mut s2 = self.the(obj);
                s2.add_space = false;
                s2.s(add)
            } else {
                self
            }
        } else if obj.is_short_proper() {
            self.s("someone's")
        } else {
            self.s("something's")
        }
    }

    /// Sends "your"/"the <object-long-name>'s" to Output.
    /// If the viewer can't see it, "something's"/"someone's" is used.
    /// The text is capitalized as needed.
    pub fn thes_<T>(self, obj: &T) -> Self
    where
        T: Object,
    {
        if self.o.is_me(obj) {
            self.s("your")
        } else if self.o.can_see(obj) {
            if let Some(ch) = obj.long_name().chars().rev().next() {
                let uc = ch.is_uppercase();
                let add = match ch {
                    's' | 'S' => "'",
                    _ => {
                        if uc {
                            "'S"
                        } else {
                            "'s"
                        }
                    }
                };
                let mut s2 = self.the_(obj);
                s2.add_space = false;
                s2.s(add)
            } else {
                self
            }
        } else if obj.is_short_proper() {
            self.s("someone's")
        } else {
            self.s("something's")
        }
    }

    /// Sends "yours"/"the <object-short-name>'s" to Output.
    /// If the viewer can't see it, "something's"/"someone's" is used.
    /// The text is capitalized as needed.
    pub fn thess<T>(self, _who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }

    /// Sends "yours"/"the <object-long-name>'s" to Output.
    /// If the viewer can't see it, "something's"/"someone's" is used.
    /// The text is capitalized as needed.
    pub fn thess_<T>(self, _who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }

    /// Sends "you"/<objects-short-name> to Output.
    /// If the viewer can't see it, "something's"/"someone's" is used.
    /// The text is capitalized as needed.
    pub fn word<T>(self, _who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }

    /// Sends "you"/<objects-long-name> to Output.
    /// If the viewer can't see it, "something's"/"someone's" is used.
    /// The text is capitalized as needed.
    pub fn word_<T>(self, _who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }

    /// The sentance will not have a dot added automatically.
    pub fn suppress_dot(mut self) -> Self {
        self.suppress_dot = true;
        self
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    pub struct DebugObject {
        named: Box<Named>,
        me: bool,
    }

    impl DebugObject {
        pub fn new(name: &str, sex: Gender, thing: bool) -> DebugObject {
            let mut buff = std::io::Cursor::new("man:men\n");
            let nf = named::Factory::from_reader(&mut buff).unwrap();
            DebugObject {
                named: nf.create(name, sex, thing),
                me: false,
            }
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

        fn is_me(&self, _: &Object) -> bool {
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

    #[test]
    fn test_is_singular() {
        assert_eq!(OutputBuilder::is_singular(Gender::Plural), false);
        assert_eq!(OutputBuilder::is_singular(Gender::Uncountable), false);
        assert_eq!(OutputBuilder::is_singular(Gender::Male), true);
        assert_eq!(OutputBuilder::is_singular(Gender::Female), true);
        assert_eq!(OutputBuilder::is_singular(Gender::Neuter), true);
    }

    #[test]
    fn test_is_vowel() {
        for c in "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ".chars() {
            assert_eq!(is_vowel(c), false, "{}", c);
        }

        for c in "aeiouAEIOU".chars() {
            assert_eq!(is_vowel(c), true, "{}", c);
        }
    }

    #[test]
    fn test_uppercase_first_char() {
        for test in vec![
            ("nisse hult", "Nisse hult"),
            ("Nisse", "Nisse"),
            ("åsa", "Åsa"),
            ("\u{DF}-titanic", "SS-titanic"),
            ("ñet", "Ñet"),
        ] {
            let mut s = String::new();
            uppercase_first_char(test.0, &mut s);
            assert_eq!(s, test.1);
        }
    }

    #[test]
    fn test_last_char() {
        assert_eq!(last_char(""), None);
        assert_eq!(last_char("a"), Some('a'));
        assert_eq!(last_char("nissa\u{302}"), Some('\u{302}'));
    }

    #[test]
    fn test_needs_dot() {
        for s in vec!["a", "nissa"] {
            assert_eq!(needs_dot(s), true);
        }
        for s in vec!["", "a.", "b!", "c?", "d:", "e;", "f\""] {
            assert_eq!(needs_dot(s), false);
        }
    }
}

mod suffix;

/*
 * TODO: someone/something should be decided depending on if it is a
 *       thing or not.
 * TODO: word/word_ (long wand)
 * TODO: plural/plural_ (the long wands)
 * TODO: he
 * TODO: he_s
 * TODO: hiss
 * TODO: him
 * TODO: himself
 * TODO: snum - number as string
 * TODO: num - 1st, 2nd, 3rd, 4th ...
 * TODO: Aliases on Named & NamedFactory: Gandalf, Gandalf the gray, %man, %gray
 */

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

/// An Object is an object or Subject in templates.
pub trait Object: Named {}

/**
 * The Output trait is used for objects that templates
 * can be rendered to.
 */
pub trait Output: Viewer {
    fn write_text(&mut self, text: &str);
    fn write_style(&mut self, &str);
    fn done(&mut self);
    fn out(&mut self) -> OutputBuilder;
}

/**
 * Objects are used in templates for obj, env etc.
 */
pub enum ObjectRef<'a> {
    // Object(&'a Object),
    // CountedObject(i64, &'a Object),
    Int(i64),
    String(&'a str),
}

/**
 * Context contains the objects referenced in templates.
 */
pub trait Context {
    fn get_me(&self) -> ObjectRef; // The subject.
    fn get_obj(&self) -> ObjectRef; // The object.
    fn get_env(&self) -> ObjectRef; // Another object.

    fn get(&self, who: &str) -> ObjectRef; // ie obj2, num, str
}

/**
 * Template contains the text's to generate a text.
 */
pub trait Template {
    fn render(&self, ctx: &Context, out: &Output);
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

/// Creates object implementing Named.
pub struct NamedFactory {
    pluralising_suffixes: suffix::Suffix<String>,
}

struct NamedImpl {
    names: Vec<String>,
    short_proper_name: bool,
    long_proper_name: bool,
    short_proper_plural_name: bool,
    long_proper_plural_name: bool,
    gender: Gender,
    thing: bool,
}

impl NamedFactory {
    pub fn new(buff: &mut std::io::BufRead) -> NamedFactory {
        fn is_comment(line: &str) -> bool {
            let line = line.trim();
            if let Some(c) = line.chars().next() {
                c == '#'
            } else {
                false
            }
        }

        let mut map: Vec<(String, String)> = vec![];
        let mut nr = 0;
        let mut line = String::new();
        while let Ok(len) = buff.read_line(&mut line) {
            if len == 0 {
                break;
            }
            nr += 1;
            if !line.trim().is_empty() && !is_comment(&line) {
                let s = line.split(':').map(String::from).collect::<Vec<String>>();
                if s.len() != 2 {
                    panic!(
                        "Pluralized mapping has the wrong format on line {}, should be \
                         from:to: {:?}",
                        nr, s
                    );
                }
                map.push((String::from(s[0].trim()), String::from(s[1].trim())));
            }
            line.clear();
        }

        NamedFactory {
            pluralising_suffixes: suffix::Suffix::new(map),
        }
    }

    fn pluralize(&self, name: &str) -> String {
        // 1. Search the map for translations.
        match self.pluralising_suffixes.lookup(name) {
            None => (),
            Some((k, v)) => {
                if let Some(s) = name.get(0..(name.len() - k.len())) {
                    let mut ret = String::from(s);
                    ret.push_str(v.as_str());
                    return ret;
                }
            }
        }
        // 2. If it ends with s, add "es"
        // 3. Add "s"
        let mut ret = String::from(name);
        if name.ends_with('s') {
            ret.push('e');
        } else {
        }
        ret.push('s');
        ret
    }

    pub fn create(&self, name: &str, gender: Gender, is_thing: bool) -> Box<Named> {
        let mut names: Vec<String> = name.split(',')
            .map(|s| String::from(s.trim_left()))
            .collect();
        if names.len() < 2 {
            let long_name = names[0].clone();
            names.push(long_name);
        }
        let short_proper_name = if let Some(c) = names[0].chars().next() {
            c == '!'
        } else {
            false
        };
        if short_proper_name {
            names[0].remove(0);
        }
        let long_proper_name = if let Some(c) = names[1].chars().next() {
            c == '!'
        } else {
            false
        };
        if names.len() < 3 {
            let long_name = names[0].clone();
            names.push(self.pluralize(&long_name));
        }
        if long_proper_name {
            names[1].remove(0);
        }
        let short_proper_plural_name = if let Some(c) = names[2].chars().next() {
            c == '!'
        } else {
            false
        };
        if names.len() < 4 {
            let long_name = names[1].clone();
            names.push(self.pluralize(&long_name));
        }
        if short_proper_plural_name {
            names[2].remove(0);
        }
        let long_proper_plural_name = if let Some(c) = names[3].chars().next() {
            c == '!'
        } else {
            false
        };
        if long_proper_plural_name {
            names[3].remove(0);
        }
        Box::new(NamedImpl {
            names,
            short_proper_name,
            long_proper_name,
            short_proper_plural_name,
            long_proper_plural_name,
            gender,
            thing: is_thing,
        }) as Box<Named>
    }
}

impl Named for NamedImpl {
    fn gender(&self) -> Gender {
        self.gender
    }

    fn is_thing(&self) -> bool {
        self.thing
    }

    fn is_short_proper(&self) -> bool {
        self.short_proper_name
    }

    fn is_long_proper(&self) -> bool {
        self.long_proper_name
    }

    fn short_name(&self) -> &str {
        self.names[0].as_str()
    }

    fn long_name(&self) -> &str {
        self.names[1].as_str()
    }

    fn is_short_plural_proper(&self) -> bool {
        self.short_proper_plural_name
    }

    fn is_long_plural_proper(&self) -> bool {
        self.long_proper_plural_name
    }

    fn short_plural_name(&self) -> &str {
        self.names[2].as_str()
    }

    fn long_plural_name(&self) -> &str {
        self.names[3].as_str()
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
}

/// Calles `Output::done()`.
impl<'a> Drop for OutputBuilder<'a> {
    fn drop(&mut self) {
        self.o.write_text(&self.s);
        if needs_dot(&self.s) {
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
        // This is never first in a sentance, so no need to
        // capitalize it.
        assert!(!self.cap_it);
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

    fn the__<T>(mut self, obj: &T, name: &str, is_proper: bool) -> Self
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
        self.the__(obj, obj.short_name(), obj.is_short_proper())
    }

    /// Sends "the object-long-name" to Output.
    /// If the viewer can't see it, someone/something is sent instead.
    /// The text is capitalized as needed.
    pub fn the_<T: Object>(self, obj: &T) -> Self {
        self.the__(obj, obj.long_name(), obj.is_long_proper())
    }

    fn a__<T>(mut self, obj: &T, name: &str, is_prop: bool) -> Self
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
        } else if obj.is_long_proper() {
            self.s("someone")
        } else {
            self.s("something")
        }
    }

    /// Sends "a/an object-short-name" to Output.
    /// If the viewer can't see it, someone/something is sent instead.
    /// The text is capitalized as needed.
    pub fn a<T: Object>(self, obj: &T) -> Self {
        self.a__(obj, obj.short_name(), obj.is_short_proper())
    }

    /// Sends "a/an object-long-name" to Output.
    /// If the viewer can't see it, someone/something is sent instead.
    /// The text is capitalized as needed.
    pub fn a_<T: Object>(self, obj: &T) -> Self {
        self.a__(obj, obj.long_name(), obj.is_long_proper())
    }

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

    pub fn sing_plur<T>(self, who: &T, singular: &str, plural: &str) -> Self
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

    pub fn is<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        self.sing_plur(who, "is", "are")
    }

    pub fn has<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        self.sing_plur(who, "has", "have")
    }

    pub fn thes<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }

    pub fn thes_<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }

    pub fn thess<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }

    pub fn thess_<T>(self, who: &T) -> Self
    where
        T: Object,
    {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct DebugObject {
        named: Box<Named>,
        me: bool,
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

    fn get_named_fac() -> NamedFactory {
        let mut pluralizer = std::io::Cursor::new("man:men\nfe:ves\n");
        NamedFactory::new(&mut pluralizer)
    }

    #[test]
    fn short_name() {
        let nf = get_named_fac();
        let ove = nf.create("!Ove, !Ove Svensson", Gender::Male, false);
        assert_eq!(ove.short_name(), "Ove");

        let eva = nf.create("Eva, Eva Stinasson", Gender::Female, false);
        assert_eq!(eva.short_name(), "Eva");
    }

    #[test]
    fn short_plural() {
        let nf = get_named_fac();
        let man = nf.create("man, old man, mob, angry mob", Gender::Male, false);
        assert_eq!(man.short_plural_name(), "mob");

        let orc = nf.create("orc, blue orc", Gender::Male, false);
        assert_eq!(orc.short_plural_name(), "orcs");

        let kiss = nf.create("kiss", Gender::Neuter, true);
        assert_eq!(kiss.short_plural_name(), "kisses");

        let knife = nf.create("knife, dull knife", Gender::Neuter, true);
        assert_eq!(knife.short_plural_name(), "knives");
    }

    #[test]
    fn long_name() {
        let nf = get_named_fac();
        let ove = nf.create("!Ove, !Ove Svensson", Gender::Male, false);
        assert_eq!(ove.long_name(), "Ove Svensson");

        let eva = nf.create("Eva, Eva Stinasson", Gender::Female, false);
        assert_eq!(eva.long_name(), "Eva Stinasson");
    }

    #[test]
    fn long_plural() {
        let nf = get_named_fac();
        let man = nf.create("man, old man, mob, angry mob", Gender::Male, false);
        assert_eq!(man.long_plural_name(), "angry mob");

        let orc = nf.create("orc, blue orc", Gender::Male, false);
        assert_eq!(orc.long_plural_name(), "blue orcs");

        let kiss = nf.create("kiss", Gender::Neuter, true);
        assert_eq!(kiss.long_plural_name(), "kisses");

        let knife = nf.create("knife, dull knife", Gender::Neuter, true);
        assert_eq!(knife.long_plural_name(), "dull knives");
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

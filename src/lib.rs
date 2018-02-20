mod suffix;

/// The gender of Named:s.
#[derive(Copy, Clone)]
pub enum Gender {
    Male,
    Female,
    Neuter,
    Plural,
    Uncountable,
}

/// Names of objects.
pub trait Named {
    fn gender(&self) -> Gender;

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
 * Viewer's answer if they see, hear etc other actors.
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

// Used to decide between a/an.
fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        // y is usually not pronounced like a vowel.
        _ => false,
    }
}

fn a__(name: &str, is_prop: bool, s: &mut String) {
    if !is_prop {
        let mut should_be_an = false;
        if let Some(c) = name.chars().next() {
            if is_vowel(c) {
                should_be_an = true;
            }
        }
        if should_be_an {
            s.push_str("an ");
        } else {
            s.push_str("a ");
        }
    }
    s.push_str(name);
}

fn a(o: &Named, s: &mut String) {
    a__(o.short_name(), o.is_short_proper(), s);
}

fn a_(o: &Named, s: &mut String) {
    a__(o.long_name(), o.is_long_proper(), s);
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

    pub fn create(&self, name: &str) -> Box<Named> {
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
            names: names,
            short_proper_name: short_proper_name,
            long_proper_name: long_proper_name,
            short_proper_plural_name: short_proper_plural_name,
            long_proper_plural_name: long_proper_plural_name,
        }) as Box<Named>
    }
}

impl Named for NamedImpl {
    fn gender(&self) -> Gender {
        Gender::Female
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

    // TODO Aliases:
    // Gandalf, Gandalf the gray, %man, %gray
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

pub struct OutputBuilder<'a> {
    o: &'a mut Output,
    s: String,
    cap_it: bool,
}

impl<'a> Drop for OutputBuilder<'a> {
    fn drop(&mut self) {
        self.o.write_text(&self.s);
        self.o.done();
    }
}

impl<'a> OutputBuilder<'a> {
    pub fn new(o: &'a mut Output) -> OutputBuilder<'a> {
        OutputBuilder {
            o: o,
            s: String::new(),
            cap_it: true,
        }
    }

    fn is_plural(gender: Gender) -> bool {
        match gender {
            Gender::Plural => true,
            _ => false,
        }
    }

    pub fn s(mut self, text: &str) -> Self {
        self.s.push_str(text);
        self.cap_it = false;
        self
    }

    pub fn v_e<T>(mut self, obj: &T, text: &str) -> Self
    where
        T: Object,
    {
        self.s.push_str(text);
        self.cap_it = false;
        if Self::is_plural(obj.gender()) && !self.o.is_me(obj) {
            self.s.push('s');
        }
        self
    }

    fn the__<T>(mut self, obj: &T, name: &str, is_proper: bool) -> Self
    where
        T: Object,
    {
        if self.o.is_me(obj) {
            self.s("you")
        } else if self.o.can_see(obj) {
            if !is_proper {
                self = self.s("the ");
            }
            self.s(name)
        } else if is_proper {
            self.s("someone")
        } else {
            self.s("something")
        }
    }

    pub fn the<T: Object>(self, obj: &T) -> Self {
        self.the__(obj, obj.short_name(), obj.is_short_proper())
    }

    pub fn the_<T: Object>(self, obj: &T) -> Self {
        self.the__(obj, obj.long_name(), obj.is_long_proper())
    }

    pub fn a<T>(mut self, obj: &T) -> Self
    where
        T: Object,
    {
        if self.o.is_me(obj) {
            self.s("you")
        } else if self.o.can_see(obj) {
            a(obj, &mut self.s);
            self.cap_it = false;
            self
        } else if obj.is_long_proper() {
            self.s("someone")
        } else {
            self.s("something")
        }
    }

    pub fn a_<T>(mut self, obj: &T) -> Self
    where
        T: Object,
    {
        if self.o.is_me(obj) {
            self.s("you")
        } else if self.o.can_see(obj) {
            a_(obj, &mut self.s);
            self.cap_it = false;
            self
        } else if obj.is_long_proper() {
            self.s("someone")
        } else {
            self.s("something")
        }
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
        pub fn new(name: &str) -> DebugObject {
            let mut buff = std::io::Cursor::new("man:men\n");
            let nf = NamedFactory::new(&mut buff);
            DebugObject {
                named: nf.create(name),
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
        let ove = nf.create("!Ove, !Ove Svensson");
        assert_eq!(ove.short_name(), "Ove");

        let eva = nf.create("Eva, Eva Stinasson");
        assert_eq!(eva.short_name(), "Eva");
    }

    #[test]
    fn short_plural() {
        let nf = get_named_fac();
        let man = nf.create("man, old man, mob, angry mob");
        assert_eq!(man.short_plural_name(), "mob");

        let orc = nf.create("orc, blue orc");
        assert_eq!(orc.short_plural_name(), "orcs");

        let kiss = nf.create("kiss");
        assert_eq!(kiss.short_plural_name(), "kisses");

        let knife = nf.create("knife, dull knife");
        assert_eq!(knife.short_plural_name(), "knives");
    }

    #[test]
    fn long_name() {
        let nf = get_named_fac();
        let ove = nf.create("!Ove, !Ove Svensson");
        assert_eq!(ove.long_name(), "Ove Svensson");

        let eva = nf.create("Eva, Eva Stinasson");
        assert_eq!(eva.long_name(), "Eva Stinasson");
    }

    #[test]
    fn long_plural() {
        let nf = get_named_fac();
        let man = nf.create("man, old man, mob, angry mob");
        assert_eq!(man.long_plural_name(), "angry mob");

        let orc = nf.create("orc, blue orc");
        assert_eq!(orc.long_plural_name(), "blue orcs");

        let kiss = nf.create("kiss");
        assert_eq!(kiss.long_plural_name(), "kisses");

        let knife = nf.create("knife, dull knife");
        assert_eq!(knife.long_plural_name(), "dull knives");
    }

    #[test]
    fn test_a() {
        let mut s: String = "".into();
        let ove = DebugObject::new("!Ove, !Ove Svensson");
        a(&ove, &mut s);
        assert_eq!(s, "Ove");

        let apple = DebugObject::new("apple");
        let mut s: String = "".into();
        a(&apple, &mut s);
        assert_eq!(s, "an apple");

        let man = DebugObject::new("man, old man, mob, angry mob");
        let mut s: String = "".into();
        a(&man, &mut s);
        assert_eq!(s, "a man");
    }

    #[test]
    fn test_is_plural() {
        assert_eq!(OutputBuilder::is_plural(Gender::Plural), true);
        assert_eq!(OutputBuilder::is_plural(Gender::Male), false);
        assert_eq!(OutputBuilder::is_plural(Gender::Female), false);
    }

    #[test]
    fn test_is_vowel() {
        for c in "bcdfghjklmnpqrstvwxyz".chars() {
            assert_eq!(is_vowel(c), false, "{}", c);
        }

        for c in "aeiou".chars() {
            assert_eq!(is_vowel(c), true, "{}", c);
        }
    }
}

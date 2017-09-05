/// The gender of Actor:s.
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
    fn can_see(&self, who: &Actor) -> bool;

    /// Can this Viewer "verb" who?
    fn can(&self, verb: &str, who: &Actor) -> bool;

    /// Does the Viewer have "property"?
    fn has(&self, property: &str) -> bool;
}

/// An Actor is an object or Subject in templates.
pub trait Actor: Named + Viewer {}


/**
 * The Output trait is used for objects that templates
 * can be rendered to.
 */
pub trait Output: Viewer {
    fn write_text(&mut self, text: &str);
    fn write_style(&mut self, &str);
    fn done(&mut self);
}

/**
 * Objects are used in templates for obj, env etc.
 */
pub enum Object<'a> {
    Act(&'a Actor),
    // CountedAct(i64, &'a Actor),
    Int(i64),
    String(&'a str),
}

/**
 * Context contains the objects referenced in templates.
 */
pub trait Context<'a> {
    fn get_me(&self) -> &Actor; // The subject.
    fn get_obj(&self) -> &Actor; // The object.
    fn get_env(&self) -> &Object; // Another object.

    fn get(&self, who: &str) -> Object<'a>; // ie obj2, num, str
}

/**
 * Template contains the text's to generate a text.
 */
pub trait Template {
    fn render(&self, ctx: &Context, out: &Output);
}

/// Creates object implementing Named.
pub struct NamedFactory {
    pluralizing_prefixes: std::collections::BTreeMap<String, String>,
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
        fn is_comment(line: &String) -> bool {
            let line = line.trim();
            if let Some(c) = line.chars().next() {
                c == '#'
            } else {
                false
            }
        }

        let mut map: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
        let mut nr = 0;
        let mut line = String::new();
        while let Ok(len) = buff.read_line(&mut line) {
            if len <= 0 {
                break;
            }
            nr += 1;
            if line.trim().len() > 0 && !is_comment(&line) {
                let s = line.split(':').map(|s| String::from(s)).collect::<Vec<String>>();
                if s.len() != 2 {
                    panic!("Pluralized mapping has the wrong format on line {}, should be \
                            from:to: {:?}",
                           nr,
                           s);
                }
                map.insert(String::from(s[0].trim()), String::from(s[1].trim()));
            }
            line.clear();
        }

        NamedFactory { pluralizing_prefixes: map }
    }

    pub fn create(&self, name: String) -> Box<Named> {
        let mut names: Vec<String> = name.split(",")
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
            // TODO: Pluralise it.
            names.push(long_name);
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
            // TODO: Pluralise it.
            names.push(long_name);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_name() {
        let mut pluralizer = std::io::Cursor::new("man:men\nve:fes\n");
        let nf = NamedFactory::new(&mut pluralizer);
        let ove = nf.create(String::from("!Ove, !Ove Svensson"));
        assert_eq!(ove.short_name(), "Ove");

        let eva = nf.create(String::from("Eva, Eva Stinasson"));
        assert_eq!(eva.short_name(), "Eva");
    }

    #[test]
    fn long_name() {
        let mut pluralizer = std::io::Cursor::new("man:men\nve:fes\n");
        let nf = NamedFactory::new(&mut pluralizer);
        let ove = nf.create(String::from("!Ove, !Ove Svensson"));
        assert_eq!(ove.long_name(), "Ove Svensson");

        let eva = nf.create(String::from("Eva, Eva Stinasson"));
        assert_eq!(eva.long_name(), "Eva Stinasson");
    }
}

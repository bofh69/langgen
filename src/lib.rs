pub enum Gender {
    Male,
    Female,
    Neuter,
    Plural,
    Uncountable,
}

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

pub trait Viewer {
    fn can_see(&self, who: &Actor) -> bool;

    // Ie the viewer can "hear" Actor.
    fn can(&self, verb: &str, who: &Actor) -> bool;

    // Ie The viewer has the see_curse property?
    fn has(&self, property: &str) -> bool;
}

pub trait Actor: Named + Viewer {}


pub trait Output: Viewer {
    fn write_text(&mut self, text: &str);
    fn write_style(&mut self, &str);
    fn done(&mut self);
}

pub enum Object<'a> {
    Act(&'a Actor),
    Int(i64),
    String(&'a str),
}

pub trait Context<'a> {
    fn get_me(&self) -> &Actor; // The subject.
    fn get_obj(&self) -> &Actor; // The object.
    fn get_env(&self) -> &Actor; // Another object.

    fn get(&self, who: &str) -> Object<'a>; // ie obj2, num, str
}

pub trait Template {
    fn render(&self, ctx: &Context, out: &Output);
}

// -------------------------------------------

pub struct NamedFactory {}

struct NamedImpl {
    names: Vec<String>,
    short_proper_name: bool,
    long_proper_name: bool,
    short_proper_plural_name: bool,
    long_proper_plural_name: bool,
}

impl NamedFactory {
    pub fn create(name: String) -> Box<Named> {
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
        let long_proper_plural_name = if let Some(c) = names[3].chars().next() {
            c == '!'
        } else {
            false
        };
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
        let ove = NamedFactory::create(String::from("!Ove, !Ove Svensson"));
        assert_eq!(ove.short_name(), "Ove");

        let eva = NamedFactory::create(String::from("Eva, Eva Stinasson"));
        assert_eq!(eva.short_name(), "Eva");
    }

    #[test]
    fn long_name() {
        let ove = NamedFactory::create(String::from("Ove, Ove Svensson"));
        assert_eq!(ove.long_name(), "Ove Svensson");

        let eva = NamedFactory::create(String::from("Eva, Eva Stinasson"));
        assert_eq!(eva.long_name(), "Eva Stinasson");
    }
}
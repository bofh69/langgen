pub enum Gender {
    Male,
    Female,
    Neuter,
    Plural,
    Uncountable,
}

pub trait Named {
    fn gender(&self) -> Gender;

    fn is_proper(&self) -> bool;
    fn short_name(&self) -> &str;
    fn long_name(&self) -> &str;

    fn is_plural_proper(&self) -> bool;
    fn short_plural_name(&self) -> &str;
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
    fn write_text(&self, text: &str);
    fn write_style(&self, &str);
    fn done(&self);
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

struct NamedImpl {
    names: Vec<String>,
}

impl NamedImpl {
    fn new(name: String) -> NamedImpl {
        let mut names: Vec<String> = name.split(",")
            .map(|s| String::from(s.trim_left()))
            .collect();
        // TODO: Parse flags.
        if names.len() < 2 {
            let long_name = names[0].clone();
            names.push(long_name);
        }
        if names.len() < 3 {
            let long_name = names[0].clone();
            // TODO: Pluralise it.
            names.push(long_name);
        }
        if names.len() < 4 {
            let long_name = names[1].clone();
            // TODO: Pluralise it.
            names.push(long_name);
        }
        NamedImpl { names: names }
    }
}

impl Named for NamedImpl {
    fn gender(&self) -> Gender {
        Gender::Female
    }

    fn is_proper(&self) -> bool {
        true
    }

    fn short_name(&self) -> &str {
        self.names[0].as_str()
    }

    fn long_name(&self) -> &str {
        self.names[1].as_str()
    }

    fn is_plural_proper(&self) -> bool {
        false
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
    use Named;

    #[test]
    fn short_name() {
        let ove = ::NamedImpl::new(String::from("Ove, Ove Svensson"));
        assert_eq!(ove.short_name(), "Ove");

        let eva = ::NamedImpl::new(String::from("Eva, Eva Stinasson"));
        assert_eq!(eva.short_name(), "Eva");
    }

    #[test]
    fn long_name() {
        let ove = ::NamedImpl::new(String::from("Ove, Ove Svensson"));
        assert_eq!(ove.long_name(), "Ove Svensson");

        let eva = ::NamedImpl::new(String::from("Eva, Eva Stinasson"));
        assert_eq!(eva.long_name(), "Eva Stinasson");
    }
}

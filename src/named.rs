use crate::suffix;
use crate::{Gender, Named};

/// Creates object implementing Named.
pub struct Factory {
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

impl Factory {
    pub fn from_reader(buff: &mut std::io::BufRead) -> Result<Self, String> {
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
                    return Err(format!(
                        "Pluralized mapping has the wrong format on line {}, should be \
                         from:to: {:?}",
                        nr, s
                    ));
                }
                map.push((String::from(s[0].trim()), String::from(s[1].trim())));
            }
            line.clear();
        }

        Ok(Self {
            pluralising_suffixes: suffix::Suffix::new(map),
        })
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
        let mut names: Vec<String> = name
            .split(',')
            .map(|s| String::from(s.trim_start()))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_named_fac() -> Factory {
        let mut pluralizer = std::io::Cursor::new("man:men\nfe:ves\n");
        Factory::from_reader(&mut pluralizer).unwrap()
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
}

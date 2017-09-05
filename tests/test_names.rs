extern crate langgen;
use langgen::*;

struct DebugActor {
    named: Box<Named>,
}

impl DebugActor {
    fn new(name: String) -> DebugActor {
        let mut buff = std::io::Cursor::new("man:men\n");
        let nf = NamedFactory::new(&mut buff);
        DebugActor { named: nf.create(name) }
    }
}

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

#[test]
fn short_name() {
    let ove = DebugActor::new(String::from("!Ove, !Ove Svensson"));
    assert_eq!(ove.short_name(), "Ove");

    let eva = DebugActor::new(String::from("Eva, Eva Stinasson"));
    assert_eq!(eva.short_name(), "Eva");
}

#[test]
fn long_name() {
    let ove = DebugActor::new(String::from("Ove, Ove Svensson"));
    assert_eq!(ove.long_name(), "Ove Svensson");

    let eva = DebugActor::new(String::from("Eva, Eva Stinasson"));
    assert_eq!(eva.long_name(), "Eva Stinasson");
}

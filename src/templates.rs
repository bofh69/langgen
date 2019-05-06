use crate::{Object, Output};

/**
 * Objects are used in templates for obj, env etc.
 */
pub enum ObjectRef<'a> {
    Object(&'a Object),
    CountedObject(i64, &'a Object),
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

/*
#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_template() {
    }
}
    */

use crate::{Object, Output};

/**
 * Objects are used in templates for obj, env etc.
 */
pub enum ObjectRef<'a> {
    Object(&'a dyn Object),
    CountedObject(i64, &'a dyn Object),
    Int(i64),
    String(&'a str),
}

/**
 * Context contains the objects referenced in templates.
 */
pub trait Context {
    fn get_me(&self) -> ObjectRef<'_>; // The subject.
    fn get_obj(&self) -> ObjectRef<'_>; // The object.
    fn get_env(&self) -> ObjectRef<'_>; // Another object.

    fn get(&self, who: &str) -> ObjectRef<'_>; // ie obj2, num, str
}

/**
 * Template contains the text's to generate a text.
 */
pub trait Template {
    fn render(&self, ctx: &dyn Context, out: &dyn Output);
}

/*
#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_template() {
    }
}
    */

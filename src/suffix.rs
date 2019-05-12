pub struct Suffix<T> {
    data: Vec<(String, T)>,
}

impl<T: Clone> Suffix<T> {
    pub fn new(data: Vec<(String, T)>) -> Self {
        Self { data }
    }

    pub fn lookup(&self, what: &str) -> Option<(&str, T)> {
        for entry in &self.data {
            if what.ends_with(&entry.0) {
                return Some((&entry.0, entry.1.clone()));
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_new() {
        let s = super::Suffix::new(vec![(String::from("abc"), 1)]);
        match s.lookup("abc") {
            Some((s, 1)) => assert_eq!(s, "abc"),
            _ => panic!("lookup did not find 'abc'"),
        }
        match s.lookup("xabc") {
            Some((s, 1)) => assert_eq!(s, "abc"),
            _ => panic!("lookup did not find 'xabc'"),
        }
        match s.lookup("cba") {
            None => (),
            _ => panic!("lookup found 'abc'"),
        }
    }
}

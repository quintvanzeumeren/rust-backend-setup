use slug::slugify;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slug(pub String);

impl Slug {
    pub fn new(s: String) -> Self {
        let s = slugify(s);
        return Self(s)
    }
    
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Slug {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&'static str> for Slug {
    fn from(value: &'static str) -> Self {
        Self::new(value.to_string())
    }
}





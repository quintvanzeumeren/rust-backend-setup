use slug::slugify;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Slug(String);

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






use std::hash::{Hash, Hasher};

/// A wrapper type around [`regex::Regex`] that admits equality tests and
/// hashing by checking whether the string representations of the two regexes
/// are the same.
#[derive(Clone, Debug)]
pub struct Regex(regex::Regex);

impl Regex {
    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }
}

impl From<regex::Regex> for Regex {
    fn from(v: regex::Regex) -> Self {
        Self(v)
    }
}

impl Eq for Regex {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Hash for Regex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state)
    }
}

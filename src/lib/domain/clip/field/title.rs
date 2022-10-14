use crate::domain::clip::ClipError;
use derive_more::From;
use serde::{Deserialize, Serialize};
use derive_more::Constructor;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Title(Option<String>);

impl Title {
    // Using generics allow for string or Option to passin
    pub fn new<T: Into<Option<String>>>(title: T ) -> Self {
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    return Self(Some(title))
                } else {
                    return Self(None)
                }
            }
            None => return Self(None)
        }
    }

    pub fn into_inner(self) -> Option<String> {
        return self.0
    }

}
// If no title, iimplement none
impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use derive_more::Constructor;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);


impl Password {
    // Using generics allow for string or Option to passin
    pub fn new<T: Into<Option<String>>>(password: T ) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    return Ok(Self(Some(password)))
                } else {
                    return Ok(Self(None))
                }
            }
            None => return Ok(Self(None))
        }
    }

    pub fn into_inner(self) -> Option<String> {
        return self.0
    }

    pub fn has_password(&self) -> bool {
        // In order to determine this, PartialEq and PartialOrd are used
        self.0.is_some()
    }
}
// If no password, iimplement none
impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
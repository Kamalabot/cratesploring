#![allow(warnings)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

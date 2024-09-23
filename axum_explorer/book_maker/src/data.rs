#![allow(warnings)]
#![allow(unused_imports)]

use crate::book::Book;
// using lazy to create a Global Data
// that can be used as database
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| {
    Mutex::new(HashMap::from([
        (
            1,
            Book {
                id: 1,
                title: "Antigone".into(),
                author: "Sophocles".into(),
            },
        ),
        (
            2,
            Book {
                id: 2,
                title: "Beloved".into(),
                author: "Morrison".into(),
            },
        ),
        (
            3,
            Book {
                id: 3,
                title: "Candide".into(),
                author: "Voltaire".into(),
            },
        ),
    ]))
});

pub use serde_json::{json, Value};
pub use crate::models::{testtableview, insert_testtable};
use diesel::{self, prelude::*};
pub use crate::db;
pub use crate::models;
pub use crate::schema;

#[get("/")]
pub fn index() -> Value {
    let sample: Value = json!({
        "firstname": "Hayden",
        "lastname": "Rose"
    });
    sample
}
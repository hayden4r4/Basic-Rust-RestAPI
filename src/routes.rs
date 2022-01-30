use crate::models::{testtableview, insert_testtable};
use diesel::{self, prelude::*};
use crate::DBPool;
use crate::models;
use crate::schema;
use schema::testtable::dsl::*;

use serde_json::Value;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::{Debug, status::Created};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub fn index() -> Value {
    let sample: Value = json!({
        "firstname": "Hayden",
        "lastname": "Rose"
    });
    sample
}

#[post("/insert", data = "<test_table>")]
pub async fn insert_to_testtable(
    conn: DBPool,
    test_table: Json<Vec<insert_testtable>>,
) -> Result<Created<Value>> {
    let test_table_values = test_table.clone();
    let result = conn.run(move |c| -> QueryResult<usize>{
        diesel::insert_into(schema::testtable::table)
            .values(&test_table_values)
            .execute(c)
    }).await?;
    let response: Value = json!(format!("{{ 'columns_inserted': {result} }}"));
    Ok(Created::new("/insert").body(response))
}

#[get("/get_table")]
pub async fn get_from_testtable(
    conn: DBPool
) -> Result<Json<Vec<testtableview>>> {
    let result = conn.run(move |c| {
        testtable
            .load(c)
    }).await?;

    Ok(Json(result))
}
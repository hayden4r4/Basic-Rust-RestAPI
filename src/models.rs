use crate::schema::testtable;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Queryable)]
pub struct testtableview {
    pub id: i64,
    pub timestamp: NaiveDate,
    pub symbol: String,
    pub price: f32,
}

#[derive(Clone, Serialize, Deserialize, Insertable)]
#[table_name = "testtable"]
pub struct insert_testtable {
    pub timestamp: NaiveDate,
    pub symbol: String,
    pub price: f32,
}

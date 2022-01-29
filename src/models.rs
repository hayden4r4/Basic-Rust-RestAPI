use crate::schema::testtable;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Queryable)]
pub struct testtableview {
    pub id: i64,
    pub symbol: String,
    pub timestamp: NaiveDate,
    pub price: f64,
}

#[derive(Deserialize, Insertable)]
#[table_name = "testtable"]
pub struct insert_testtable {
    pub symbol: String,
    pub timestamp: NaiveDate,
    pub price: f64,
}

#[macro_use] 
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate chrono;


pub use diesel::prelude::*;
pub use rocket_sync_db_pools::{database};

pub use serde_json::{json, Value};
pub use serde::{Serialize, Deserialize};

pub mod db;
pub mod models;
pub mod schema;
pub mod routes;

#[database("rockettest")]
pub struct DBPool(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::index])
        .attach(DBPool::fairing())
}   
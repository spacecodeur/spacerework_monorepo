use diesel::prelude::*;
use std::env;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
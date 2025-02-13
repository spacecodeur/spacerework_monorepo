use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be defined in the .env file");

    Database::connect(database_url).await
}

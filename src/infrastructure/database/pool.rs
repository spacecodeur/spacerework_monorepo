use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use std::{env, sync::Arc};

#[derive(Debug, Clone)]
pub struct DatabaseService {
    pub connection: Arc<DatabaseConnection>,
}

impl DatabaseService {
    pub async fn init() -> Self {
        dotenvy::dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be defined in the .env file");

        let mut connection_options = ConnectOptions::new(database_url);
        connection_options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

        let connection = Database::connect(connection_options)
            .await
            .expect("Can't connect to database");

        Self {
            connection: Arc::new(connection),
        }
    }
}

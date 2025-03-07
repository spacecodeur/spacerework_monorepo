use std::{env, time::Duration};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use uuid::Uuid;

pub fn get_db_connection_uri() -> String {
    dotenvy::dotenv().expect(".env file not found, not readable or invalid");
    env::var("DATABASE_HOST_URL").expect("DATABASE_HOST_URL must be defined in the .env file")
}

pub fn get_app_db_name() -> String {
    "postgres".to_owned()
}

pub fn get_temp_db_name() -> String {
    format!("test_db_{}", Uuid::new_v4().to_string().replace('-', ""))
}

pub async fn setup(
    db_connection_uri: &str,
    app_db_name: &str,
    temp_db_name: &str,
) -> DatabaseConnection {
    let mut app_db_options = ConnectOptions::new(format!("{}/{}", db_connection_uri, app_db_name));
    app_db_options
        .max_connections(1)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(2))
        .acquire_timeout(Duration::from_secs(2))
        .idle_timeout(Duration::from_secs(1))
        .max_lifetime(Duration::from_secs(5))
        .sqlx_logging(false);

    let app_db_connection = Database::connect(app_db_options)
        .await
        .expect("Can't connect to host database");

    // Create the temporary database
    let create_db_query = format!(r#"CREATE DATABASE "{}""#, temp_db_name);
    app_db_connection
        .execute(sea_orm::Statement::from_string(
            app_db_connection.get_database_backend(),
            create_db_query,
        ))
        .await
        .expect("Failed to create the temporary database");

    // Build the connection URL for the temporary database
    let mut temp_db_options =
        ConnectOptions::new(format!("{}/{}", db_connection_uri, temp_db_name));
    temp_db_options
        .max_connections(1) // Permet plusieurs connexions pour les tests parallèles
        .min_connections(1)
        .connect_timeout(Duration::from_secs(2))
        .acquire_timeout(Duration::from_secs(2))
        .idle_timeout(Duration::from_secs(2))
        .max_lifetime(Duration::from_secs(5))
        .sqlx_logging(false);

    let temp_db_connection = Database::connect(temp_db_options)
        .await
        .expect("Unable to connect to the temporary database");

    // Appliquer les migrations
    Migrator::up(&temp_db_connection, None)
        .await
        .expect("Migration failed");

    temp_db_connection
}

pub async fn cleanup(db_connection_uri: &str, app_db_name: &str, temp_db_name: &str) {
    let app_db_connection = Database::connect(format!("{}/{}", db_connection_uri, app_db_name))
        .await
        .expect("Unable to connect to PostgreSQL");

    let drop_db_query = format!(r#"DROP DATABASE "{}" WITH (FORCE)"#, temp_db_name);
    app_db_connection
        .execute(sea_orm::Statement::from_string(
            app_db_connection.get_database_backend(),
            drop_db_query,
        ))
        .await
        .expect("Failed to delete the temporary database");
}

use sqlx::{Connection, PgConnection, PgPool};
use tars::configuration::DatabaseSettings;

pub async fn configure_postgres(config: DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_wthout_name())
        .await
        .expect("failed to connect to the database");
    sqlx::query(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .execute(&mut connection)
        .await
        .expect("Create Database Failed");
    connection
        .close()
        .await
        .expect("unable to close the connection");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    // Insert default roles needed by the tests
    sqlx::query(
        "INSERT INTO user_roles (name, description) VALUES ('artist', 'Default artist role'), ('admin', 'Admin role')"
    )
    .execute(&connection_pool)
    .await
    .expect("Failed to insert default roles");

    connection_pool
}


use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

/* DB */
pub async fn connect_to_db() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to db using url {} ...", database_url);

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create DB pool.");

    println!("Connected to the database");
    pg_pool
}

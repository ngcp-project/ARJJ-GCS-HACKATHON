mod schema;
mod insert_programs;
mod fetch_programs;
mod api;

use sqlx::postgres::PgPoolOptions;
use schema::create_schema;
use insert_programs::insert_test_programs;
use api::start_server;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "postgres://postgres:root@localhost:5431/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("Connected to the database.");

    // Create the schema
    create_schema(&pool).await?;
    println!("Schema created successfully.");

    // Insert test programs
    insert_test_programs(&pool).await?;

    // Start the API server
    start_server(pool).await;

    Ok(())
}

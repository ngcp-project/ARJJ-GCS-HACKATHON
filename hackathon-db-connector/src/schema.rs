use sqlx::postgres::PgPool;

pub async fn create_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Programs table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS programs (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            image_url TEXT NOT NULL
        );"
    )
    .execute(pool)
    .await?;

    // Users table: username, password (plaintext for demo), and barcode (integer)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            barcode BIGINT NOT NULL
        );"
    )
    .execute(pool)
    .await?;

    Ok(())
}
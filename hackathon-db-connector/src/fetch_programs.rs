use sqlx::postgres::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct Program {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: String,
}

pub async fn fetch_programs(pool: &PgPool) -> Result<Vec<Program>, sqlx::Error> {
    let rows = sqlx::query_as::<_, Program>(
        "SELECT id, name, description, image_url FROM programs"
    )
    .fetch_all(pool)
    .await?;

    println!("Fetched programs: {:?}", rows);
    Ok(rows)
}
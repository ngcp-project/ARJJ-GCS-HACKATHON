use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn insert_test_programs(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Wipe the database
    sqlx::query("DELETE FROM programs")
        .execute(pool)
        .await?;

    // Insert the specified programs with real image URLs
    let programs = vec![
        ("Scuba Diving", "Explore the underwater world with our scuba diving program.", "https://sustainabletravel.org/wp-content/uploads/Blog-Header-Diver-School-of-Fish.jpg"),
        ("Rock Climbing", "Reach new heights with our rock climbing adventures.", "https://cdn.theatlantic.com/thumbor/2XUbA-FsydAKoQbklWw3CxLTkdc=/0x74:2808x1654/1952x1098/media/img/2015/09/CULT_Rich_Honnold_Opener_HP/original.jpg"),
        ("Swim Lessons", "Learn to swim with our expert instructors.", "https://cdn.britannica.com/14/242014-050-EDF0B3A1/Michael-Phelps-celebrates-gold-medals-2016-Olympics.jpg?w=300"),
    ];

    for (name, description, image_url) in programs {
        sqlx::query(
            "INSERT INTO programs (name, description, image_url) VALUES ($1, $2, $3)"
        )
        .bind(name)
        .bind(description)
        .bind(image_url)
        .execute(pool)
        .await?;
    }
    
    // Insert default users (plaintext passwords for demo only)
    sqlx::query("DELETE FROM users")
        .execute(pool)
        .await?;

    let users = vec![
        ("alice", "alicepass", 1001001i64, false),
        ("bob", "bobpass", 1001002i64, false),
        ("carol", "carolpass", 1001003i64, false),
        ("admin", "adminpass", 9999999i64, true),
    ];

    for (username, password, barcode, _is_admin) in users {
        sqlx::query(
            "INSERT INTO users (username, password, barcode) VALUES ($1, $2, $3)"
        )
        .bind(username)
        .bind(password)
        .bind(barcode)
        .execute(pool)
        .await?;
    }

    // Fetch and print users to aid debugging
    let rows = sqlx::query("SELECT id, username, password, barcode FROM users")
        .fetch_all(pool)
        .await?;

    println!("Inserted users:");
    for r in rows {
        let id: i32 = r.get("id");
        let username: String = r.get("username");
        let password: String = r.get("password");
        let barcode: i64 = r.get("barcode");
        println!("- id={} username={} password={} barcode={}", id, username, password, barcode);
    }

    Ok(())
}
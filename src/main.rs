use dotenv;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
struct Todo {
    id: i64,
    title: String,
    descripion: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DB_URL").expect("db_url must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    // Create table if not exist
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS todo (
    id SERIAL PRIMARY KEY,
    title text NOT NULL,
    description text,
    completed bool
);        "#,
    )
    .execute(&pool)
    .await?;

    let rows = sqlx::query("SELECT * FROM todo").fetch_all(&pool).await?;

    let str_result = rows
        .iter()
        .map(|r| {
            format!(
                "{} - {} : {}",
                r.get::<String, _>("title"),
                r.get::<String, _>("description"),
                r.get::<bool, _>("completed")
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", str_result);

    sqlx::query("INSERT INTO todo (title, description, completed) VALUES ($1, $2, $3)")
        .bind("test")
        .bind("Beschreibung")
        .bind(false)
        .execute(&pool)
        .await?;
    let rows = sqlx::query("SELECT * FROM todo").fetch_all(&pool).await?;

    let str_result = rows
        .iter()
        .map(|r| {
            format!(
                "{} - {} : {}",
                r.get::<String, _>("title"),
                r.get::<String, _>("description"),
                r.get::<bool, _>("completed")
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", str_result);

    sqlx::query("DROP TABLE todo").execute(&pool).await?;

    Ok(())
}

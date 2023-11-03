use sqlx::postgres::PgPoolOptions;

use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug, FromRow)]
pub struct DbTodo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
pub struct Todo {
    title: String,
    description: String,
    completed: bool,
}
impl Todo {
    pub fn new(title: String, desc: String, completed: bool) -> Self {
        Todo {
            title: title,
            description: desc,
            completed: completed,
        }
    }
}
pub async fn create_db_pool(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    Ok(pool)
}

pub async fn create_table(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS todo (
    id SERIAL PRIMARY KEY,
    title text NOT NULL,
    description text,
    completed bool
);        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_tasks(pool: &Pool<Postgres>) -> Result<Vec<DbTodo>, sqlx::Error> {
    let rows = sqlx::query_as::<_, DbTodo>("SELECT * FROM todo")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn insert_new_todo(pool: &Pool<Postgres>, task: Todo) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO todo (title, description, completed) VALUES ($1, $2, $3) ")
        .bind(task.title)
        .bind(task.description)
        .bind(task.completed)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn select_by_id(pool: &Pool<Postgres>, id: i32) -> Result<DbTodo, sqlx::Error> {
    let todo = sqlx::query_as::<_, DbTodo>("SELECT * FROM todo WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(todo)
}

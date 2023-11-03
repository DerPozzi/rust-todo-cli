// TODO: Clap Argument parsing
// TODO: Pretty print the tasks

use dotenv;

mod database;
use database::{create_db_pool, create_table, get_all_tasks, insert_new_todo, Todo};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let new_task = Todo::new("Title".to_string(), "Description".to_string(), false);
    let db_url = std::env::var("DB_URL").expect("db_url must be set");

    let pool = create_db_pool(&db_url).await?;
    // Create table if not exist
    create_table(&pool).await?;
    match insert_new_todo(&pool, new_task).await {
        Ok(_) => {}
        Err(msg) => println!("ERROR: {}", msg),
    }
    let rows = get_all_tasks(&pool).await?;

    for task in rows {
        println!(
            "Id: {} - {} - {} : {}",
            task.id, task.title, task.description, task.completed
        )
    }

    Ok(())
}

/*
 TODOs: update todo, pretty print
*/
use anyhow::{self, Error};
use clap::{command, Parser, ValueEnum};
use dotenv;

mod database;
use database::{
    create_db_pool, create_table, delete_by_id, get_all_tasks, insert_new_todo, select_by_id,
    update_by_id, Todo,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,
    /// Id to to get from DB
    #[arg(short)]
    id: Option<i32>,
    /// Title of the new todo
    #[arg(short)]
    title: Option<String>,
    /// Description of the todo
    #[arg(short)]
    description: Option<String>,
    /// Is Todo completed?
    #[arg(short)]
    completed: bool,
}

#[derive(ValueEnum, Debug, Clone)]
enum Mode {
    Insert,
    Update,
    Select,
    Delete,
}

#[tokio::main]

async fn main() -> Result<(), Error> {
    let args = Args::parse();
    dotenv::dotenv().ok();

    let db_url = std::env::var("DB_URL").expect("db_url must be set");
    let pool = create_db_pool(&db_url)
        .await
        .expect("Connection to db failed");
    create_table(&pool)
        .await
        .expect("Creating table in DB failed");

    match args.mode {
        Mode::Insert => {
            if let None = args.title {
                return Err(Error::msg("Title must be provided"));
            }
            let new_task = Todo::new(
                args.title.unwrap(),
                args.description.unwrap_or("".to_string()),
                args.completed,
            );
            insert_new_todo(&pool, new_task).await?;
        }
        Mode::Update => {
            if let Some(id) = args.id {
                let old_todo = select_by_id(&pool, id).await?;
                let updated_todo = Todo::new(
                    {
                        if let Some(title) = args.title {
                            title
                        } else {
                            old_todo.title
                        }
                    },
                    {
                        if let Some(desc) = args.description {
                            desc
                        } else {
                            old_todo.description
                        }
                    },
                    old_todo.completed || args.completed,
                );

                update_by_id(&pool, id, updated_todo).await?;
            } else {
                return Err(Error::msg("Id must be provided"));
            }
        }
        Mode::Select => {
            if let Some(id) = args.id {
                println!("Displaying todo [{}]", id);
                let todo = select_by_id(&pool, id).await?;
                println!(
                    "Id: {} - {} - {} : {}",
                    todo.id, todo.title, todo.description, todo.completed
                );
            } else {
                let rows = get_all_tasks(&pool).await?;
                println!("Displaying all Todos");
                for task in rows {
                    println!(
                        "Id: {} - {} - {} : {}",
                        task.id, task.title, task.description, task.completed
                    )
                }
            }
        }
        Mode::Delete => {
            if let Some(id) = args.id {
                delete_by_id(&pool, id).await?;
            } else {
                return Err(Error::msg("Id must be provided"));
            }
        }
    }

    Ok(())
}

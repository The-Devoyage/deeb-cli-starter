use clap::Parser;

use cli::{Cli, RootCommands, TodoCommands};
use deeb::Deeb;
use environment::Environment;
use models::todo::Todo;

pub mod cli;
pub mod environment;
pub mod models;

pub type CliResult<T> = Result<T, Box<dyn std::error::Error>>;

// Handles running and logging at top level
#[tokio::main]
async fn main() -> CliResult<()> {
    if let Err(err) = run().await {
        eprintln!("❌ Error: {err}");
        std::process::exit(1);
    }

    Ok(())
}

// Run the app
async fn run() -> CliResult<()> {
    // Init environment
    let _environment = Environment::new()?;

    // Init logger
    pretty_env_logger::init();
    log::info!("Starting Deeb CLI Starter!");
    log::debug!("You can log anything...");

    // Start CLI
    let cli = Cli::parse();

    // Start Deeb DB
    let db = Deeb::new();
    db.add_instance("todos", "./todos.json", vec![Todo::entity()])
        .await?;

    match cli.root_commands {
        RootCommands::Todo(todo_commands) => match todo_commands {
            TodoCommands::Add { name, task } => {
                Todo::new(&db, name, task).await?;
            }
            TodoCommands::Ls => Todo::list(&db).await?,
        },
    }

    Ok(())
}

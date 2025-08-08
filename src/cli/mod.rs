use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli")]
#[command(about = "My awesome CLI with easy data persistence")]

/// Top Level of CLI
pub struct Cli {
    #[command(subcommand)]
    pub root_commands: RootCommands,
}

/// First command that can be run
#[derive(Subcommand)]
pub enum RootCommands {
    /// Manage Todos
    #[command(subcommand)]
    Todo(TodoCommands),
}

/// Subcommands for the todo command
#[derive(Subcommand)]
pub enum TodoCommands {
    /// Add a new Todo
    Add {
        #[arg(required = true)]
        name: String,

        #[arg(required = true)]
        task: String,
    },

    /// View To Dos
    Ls,
}

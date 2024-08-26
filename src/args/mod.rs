use clap::{Parser, Subcommand, Args};

/// A small CLI todoapp.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustodoArgs{
    #[clap(subcommand)]
    pub action: ActionType
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Add a new task
    Add(Task),
    /// Mark off a task as done
    Mark,
    /// List your tasks
    List
}

#[derive(Debug, Args)]
pub struct Task {
    /// Title of the task
    pub title: String,

    #[clap(skip)]
    /// Toggles if the task is marked as done.
    pub is_done: bool
}
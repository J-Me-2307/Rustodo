use clap::{Args, Parser, Subcommand};
use serde::Deserialize;
use tabled::Tabled;

/// A small CLI todoapp.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustodoArgs {
    #[clap(subcommand)]
    pub action: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Add a new task
    Add(Task),
    /// Mark off a task as done
    Mark,
    /// List your tasks
    List(ListOptions)
}

#[derive(Debug, Args)]
pub struct ListOptions{
        /// Show only tasks that are completed
        #[clap(short, long)]
        pub completed: bool,
    
        /// Show only tasks that are not completed
        #[clap(short, long)]
        pub pending: bool,
    
        /// Sort tasks by title
        #[clap(short='t', long)]
        pub sort_by_title: bool,
    
        /// Reverse the order of tasks
        #[clap(short, long)]
        pub reverse: bool,
}

#[derive(Debug, Args, Deserialize, Tabled)]
pub struct Task {
    /// Title of the task
    pub title: String,

    #[clap(skip)]
    /// Toggles if the task is marked as done.
    pub is_done: bool,
}

impl Task {
    pub fn to_record(self) -> Vec<String> {
        vec![self.title, self.is_done.to_string()]
    }
}

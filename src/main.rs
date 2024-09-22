use std::{
    error::Error,
    fs::{File, OpenOptions},
    path::Path,
    process,
};

use args::{ActionType, ListOptions, RustodoArgs, Task};
use clap::Parser;
use tabled::{settings::Style, Table};

mod args;

fn main() {
    let args = RustodoArgs::parse();

    if let Err(e) = run(args) {
        eprint!("{e}");
        process::exit(1);
    }
}

fn run(args: RustodoArgs) -> Result<(), Box<dyn Error>> {
    let file = get_csv()?;

    match args.action {
        ActionType::Add(todo) => {
            write_single_to_csv(&file, todo.to_record())?;
        }
        ActionType::List(list_option) => {
            let tasks = read_csv(&file)?;
            list_tasks(list_option, tasks);
        }
        ActionType::Mark(task) => {
            mark_task_as_done(file, task.title)?;
        }
        ActionType::Reset => {
            delete_csv()?;
        }
    }

    Ok(())
}

/// Marks a task as done.
fn mark_task_as_done(file: File, title: String) -> Result<(), Box<dyn Error>>{
    let tasks = read_csv(&file)?;
    delete_csv()?;
    let file = get_csv()?;

    for mut task in tasks {
        if task.title == title {
            task.is_done = true;
        }
        write_single_to_csv(&file, task.to_record())?;
    }

    Ok(())
}

/// Lists all tasks that match the options.
fn list_tasks(list_option: ListOptions, tasks: Vec<Task>) {
    let mut tasks_to_show: Vec<Task> = Vec::new();

    if tasks.is_empty() {
        eprintln!("You currently have no tasks.");
        return;
    }

    // Get only tasks that match the filter.
    if list_option.completed {
        for task in tasks {
            if task.is_done {
                tasks_to_show.push(task)
            }
        }
    } else if list_option.pending {
        for task in tasks {
            if !task.is_done {
                tasks_to_show.push(task)
            }
        }
    } else {
        tasks_to_show = tasks
    }

    // Check if any tasks with that filter exist.
    if tasks_to_show.is_empty() {
        eprintln!("You have no tasks that match the current filter!");
        return;
    }

    // Order the tasks depending on the flags.
    if list_option.sort_by_title {
        tasks_to_show.sort_by(|a, b| a.title.cmp(&b.title))
    }

    if list_option.reverse {
        tasks_to_show.reverse()
    }

    // pretty print the tasks
    let mut table = Table::new(tasks_to_show);
    table.with(Style::rounded());

    println!("{}", table)
}

/// Reads all data from the csv
fn read_csv(file: &File) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut tasks: Vec<Task> = Vec::new();

    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        tasks.push(result?)
    }

    Ok(tasks)
}

fn delete_csv() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./data.csv");

    std::fs::remove_file(path)?;

    Ok(())
}

/// Gets the csv file. If the file doesn't exist, it creates one and writes the header to it.
fn get_csv() -> Result<File, Box<dyn Error>> {
    let path = Path::new("./data.csv");

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;

    if file.metadata()?.len() == 0 {
        let header = vec![String::from("title"), String::from("is_done")];
        write_single_to_csv(&file, header)?;
    }

    Ok(file)
}

/// Writes a single record to the csv.
fn write_single_to_csv(file: &File, record: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(record)?;
    wtr.flush()?;

    Ok(())
}
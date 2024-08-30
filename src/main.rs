use std::{
    error::Error,
    fs::{File, OpenOptions},
    path::Path,
    process, vec,
};

use args::{ActionType, ListOptions, RustodoArgs, Task};
use clap::Parser;
use tabled::{settings::{object::Rows, style::BorderColor, Alignment, Modify, Style}, Table};

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
            let tasks = read_csv(file)?;
            list_tasks(list_option, tasks);
        }
        ActionType::Mark => {
            todo!()
        }
    }

    Ok(())
}

/// Lîsts all tasks that match the options.
fn list_tasks(list_option: ListOptions, tasks: Vec<Task>) {
    let mut tasks_to_show: Vec<Task> = Vec::new();

    // Get only tasks that match the filter.
    if list_option.completed {
        for task in tasks{
            if task.is_done{
                tasks_to_show.push(task)
            }
        }
    } else if list_option.pending {
        for task in tasks{
            if !task.is_done{
                tasks_to_show.push(task)
            }
        }
    } else {
        tasks_to_show = tasks
    }

    if list_option.sort_by_title{
        tasks_to_show.sort_by(|a, b| a.title.cmp(&b.title))
    }

    if list_option.reverse{
        tasks_to_show.reverse()
    }

    let mut table = Table::new(tasks_to_show);

    table.with(Style::rounded());

    println!("{}", table)
}

fn read_csv(file: File) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut tasks: Vec<Task> = Vec::new();

    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        tasks.push(result?)
    }

    Ok(tasks)
}

/// Gets the csv file. If the file doesn't exist, it creates one and writes the header to it.
fn get_csv() -> Result<File, Box<dyn Error>> {
    let path = Path::new("./data.csv");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
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

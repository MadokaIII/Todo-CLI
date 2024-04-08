mod task;
use clap::{Arg, Command};
use dirs::home_dir;
use std::{fs::OpenOptions, io::Write};
use task::{string_to_prio, Priority, Task};

fn ensure_todo_file_exists() -> std::io::Result<()> {
    if let Some(mut path) = home_dir() {
        path.push(".todo-cli.json");
        OpenOptions::new().create(true).write(true).open(path)?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory could not be determined.",
        ));
    }
    Ok(())
}

fn cli() -> Command {
    Command::new("todo-cli")
        .bin_name("todo-cli")
        .about("Todo List in a CLI")
        .version("0.1.0")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Add a Task to the List")
                .args([
                    Arg::new("description")
                        .value_name("DESCRIPTION")
                        .help("A basic description of the Task")
                        .short('d')
                        .long("description")
                        .required(true)
                        .num_args(1..)
                        .allow_hyphen_values(true),
                    Arg::new("priority")
                        .value_name("PRIORITY")
                        .help("Higher is more important")
                        .short('p')
                        .long("priority")
                        .value_parser(["0", "1", "2", "3", "4"])
                        .default_value("2")
                        .required(false)
                        .num_args(1),
                ])
                .arg_required_else_help(true),
        )
}

fn main() {
    if let Err(e) = ensure_todo_file_exists() {
        eprintln!("Failed to ensure .todo-cli.json file exists: {}", e);
    }
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let description = sub_matches
                .get_many::<String>("description")
                .expect(
                    "Failed to read description: This should be unreachable due to clap validation",
                )
                .fold(String::new(), |acc, s| {
                    if acc.is_empty() {
                        s.to_string()
                    } else {
                        acc + " " + s
                    }
                });

            let priority = sub_matches
                .get_one::<String>("priority")
                .map(string_to_prio)
                .unwrap_or(Priority::Normal); // Assuming Priority::Default is a valid default value

            let task = Task::new(description, priority);

            let data =
                serde_json::to_string_pretty(&task).expect("Failed to serialize task") + "\n";

            let mut path = home_dir().expect("Failed to find home directory");
            path.push(".todo-cli.json");

            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .open(path)
                .expect("Failed to open todo file");

            file.write_all(data.as_bytes())
                .expect("Failed to write to todo file");
        }
        _ => unreachable!(),
    }
}

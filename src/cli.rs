use crate::tasks::{get_all_tasks, save_tasks};
use clap::{Parser, ValueEnum};
use dialoguer::{Confirm, Select};

#[derive(Debug, Clone, ValueEnum)]
enum Command {
    #[value(name = "help")]
    Help,
    #[value(name = "create")]
    Create,
    #[value(name = "list")]
    List,
    #[value(name = "edit")]
    Edit,
    #[value(name = "mark")]
    Mark,
    #[value(name = "delete")]
    Delete,
}

#[derive(Parser)]
struct Cli {
    command: Command,
    // content: String,
}

pub fn start() {
    let args = Cli::parse();

    match args.command {
        Command::Help => {
            println!("Доступные команды");
            println!("- list");
            println!("- create");
            println!("- delete");
            println!("- edit");
            println!("- mark");
        }
        Command::Create => {
            println!("1235")
        }
        Command::List => {
            let tasks = get_all_tasks();
            for task in tasks {
                println!("[ ] => {}", task.title);
                if task.description.len() != 0 {
                    println!("{}", task.description);
                }
            }
        }
        Command::Edit => {
            println!("1236")
        }
        Command::Mark => {
            println!("1237")
        }
        Command::Delete => {
            let mut tasks = get_all_tasks();
            let deletion_idx = Select::new()
                .with_prompt("Какую таску хотите удалить?")
                .items(&tasks.iter().map(|task| task.title.clone()).collect::<Vec<String>>())
                .default(0)
                .interact()
                .unwrap();

            let confirmation = Confirm::new()
                .with_prompt("Уверены?")
                .interact()
                .unwrap();

            if confirmation {
                tasks.remove(deletion_idx);
                save_tasks(tasks);
            }
        }
    }
}

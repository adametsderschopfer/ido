use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use crate::tasks::{Task, TaskStatus, get_all_tasks, save_tasks};
use clap::{Parser, ValueEnum};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use serde::Serialize;

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
    content: Option<String>,
}

pub fn start() {
    let args = Cli::parse();
    let mut tasks = get_all_tasks();

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
            println!("Введите название задачи:");
            let task_title = Input::<String>::with_theme(&ColorfulTheme::default())
                .interact()
                .unwrap();

            let new_task = Task {
                id: uuid::Uuid::new_v4().to_string(),
                title: task_title,
                status: TaskStatus::Idle,
            };
            tasks.push(new_task);
            save_tasks(tasks);
        }
        Command::List => {
            for task in tasks {
                let status;

                match task.status {
                    TaskStatus::Idle => status = "",
                    TaskStatus::Done => status = "Y",
                    TaskStatus::Canceled => status = "-",
                }

                println!("[{status}] => {}", task.title);
            }
        }
        Command::Edit => {
            let item_idx = Select::new()
                .with_prompt("Изменить задачу")
                .items(
                    &tasks
                        .iter()
                        .map(|task| task.title.clone())
                        .collect::<Vec<String>>(),
                )
                .default(0)
                .interact()
                .unwrap();
            let ct = tasks.index_mut(item_idx);
            let task_title = Input::<String>::with_theme(&ColorfulTheme::default())
                .default(ct.title.clone())
                .interact()
                .unwrap();
            ct.title = task_title;
            save_tasks(tasks);
        }
        Command::Mark => {
            let mark_idx = Select::new()
                .with_prompt("Выберите задачу")
                .items(
                    &tasks
                        .iter()
                        .map(|task| task.title.clone())
                        .collect::<Vec<String>>(),
                )
                .default(0)
                .interact()
                .unwrap();
            let ct = tasks.index_mut(mark_idx);

            let status_idx = Select::new()
                .with_prompt(ct.title.clone())
                .items([
                    "В работе",
                    "Выполнена",
                    "Отменена",
                ])
                .default(0)
                .interact()
                .unwrap();

            match status_idx {
                0 => {
                    ct.status = TaskStatus::Idle
                },
                1 => {
                    ct.status = TaskStatus::Done
                },
                2 => {
                    ct.status = TaskStatus::Canceled
                }
                _ => {}
            }
            save_tasks(tasks)
        }
        Command::Delete => {
            let deletion_idx = Select::new()
                .with_prompt("Удалить задачу")
                .items(
                    &tasks
                        .iter()
                        .map(|task| task.title.clone())
                        .collect::<Vec<String>>(),
                )
                .default(0)
                .interact()
                .unwrap();

            let confirmation = Confirm::new().with_prompt("Уверены?").interact().unwrap();

            if confirmation {
                tasks.remove(deletion_idx);
                save_tasks(tasks);
            }
        }
    }
}

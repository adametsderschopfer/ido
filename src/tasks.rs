use serde::{Deserialize, Serialize};

const DATA_FILE_PATH: &str = "data/data.json";

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Idle,
    Canceled,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub status: TaskStatus, // default: TodoStatus::IDLE
    pub title: String,
}

impl Task {
    fn mark(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }
}

pub fn save_tasks(tasks: Vec<Task>) {
    let j = serde_json::to_string(&tasks).unwrap();

    let write_res = std::fs::write(DATA_FILE_PATH, j);
    match write_res {
        Err(err) => {
            panic!("Unable to save tasks: {}", err);
        }
        _ => {}
    }
}

pub fn get_all_tasks() -> Vec<Task> {
    let result: Vec<Task>;
    let data_content = std::fs::read_to_string(DATA_FILE_PATH);
    match data_content {
        Ok(content) => {
            result = serde_json::from_str::<Vec<Task>>(&content).unwrap();
        }
        Err(_) => {
            panic!("Unable to read {DATA_FILE_PATH} file");
        }
    }

    result
}



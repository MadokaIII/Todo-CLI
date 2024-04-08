use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Priority {
    Lowest,
    Low,
    Normal,
    High,
    Highest,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub priority: Priority,
    pub creation_time: DateTime<Local>,
}

impl Task {
    pub fn new(description: String, priority: Priority) -> Self {
        Self {
            description,
            priority,
            creation_time: Local::now(),
        }
    }
}

pub fn string_to_prio(number: &String) -> Priority {
    match number.as_str() {
        "0" => Priority::Lowest,
        "1" => Priority::Low,
        "2" => Priority::Normal,
        "3" => Priority::High,
        "4" => Priority::Highest,
        _ => panic!("Invalid priority value: {}", number), // This should theoretically never happen
    }
}

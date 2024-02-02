use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Idle,
    Active,
    Done,
    Archived,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    description: String,
    tags: Vec<String>,
    stack: Vec<String>,
    created: chrono::NaiveDate,
    status: Option<Status>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            tags: Vec::new(),
            stack: Vec::new(),
            status: Some(Status::Idle),
            created: chrono::Local::now().naive_utc().date(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

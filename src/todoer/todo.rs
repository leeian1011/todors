use chrono::Local;

use super::print::{Colour, Printer};

#[derive(serde::Deserialize)]
struct Task {
    name: String,
    description: String,
    priority: Colour,
    completed: bool,
    created: i64,
}

impl Task {
    fn new(name: String, desc: Option<String>, prio: Option<Colour>) -> Self {
        Self {
            name,
            description: match desc {
                Some(desc) => desc,
                None => "".to_string(),
            },
            priority: match prio {
                Some(prio) => prio,
                None => Colour::CyanText,
            },
            completed: false,
            created: Local::now().timestamp_millis(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = String::new();
        match std::env::vars().find(|(var, _)| var == "TODORS_CACHE") {
            None => {
                path.push('.');
            }
            Some((_, _path)) => {
                path = _path;
            }
        };

        let tasks = std::fs::read(path + "/todorstasks.json")?;
        let tasks = serde_json::from_str::<Vec<Task>>(&String::from_utf8_lossy(&tasks))?;

        Ok(Self { 
            tasks,
        })
    }

    pub fn add(&mut self, name: String, desc: Option<String>, prio: Option<Colour>) -> Result<(), TodoError> {
        match self.tasks.iter().find(|task| task.name == name) {
            Some(_) => return Err(TodoError(format!("Task with name {} already exists!", name))),
            None => {}
        }
        self.tasks.push(Task::new(name, desc, prio));
        Ok(())
    }

    pub fn remove(&mut self, name: String) {
        self.tasks.retain(|task| task.name != name);
    }

    pub fn list(&self) {
        let mut printable_array: [Vec<&str>;2] = [vec!["Name"], vec!["Description"]];
        
        let mut colour_array: Vec<Colour> = vec![Colour::MagentaText];
        let tasks = &self.tasks;

        for task in tasks {
            printable_array[0].push(&task.name);
            printable_array[1].push(&task.description);
            if task.completed {
                colour_array.push(Colour::GreenText);
            } else {
                colour_array.push(task.priority);
            }
        }


        _ = Printer::table_print(&printable_array, &colour_array);
    }
}

pub struct TodoError(pub String);

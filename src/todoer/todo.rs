use chrono::Local;

use super::print::{Colour, Printer};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Task {
    name: String,
    tag: String,
    description: String,
    priority: Colour,
    completed: bool,
    created: i64,
}

impl Task {
    fn new(name: String, prio: Option<Colour>, tag: Option<String>) -> Self {
        Self {
            name,
            tag: match tag {
                Some(tag) => tag,
                None => "".to_string(),
            },
            description: "".to_string(),
            priority: match prio {
                Some(prio) => prio,
                None => Colour::CyanText,
            },
            completed: false,
            created: Local::now().timestamp_millis(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
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

        Ok(Self { tasks })
    }

    pub fn describe(&mut self, name: &str, desc: String) -> Result<(), TodoError> {
        match self.tasks.iter_mut().find(|task| task.name == name) {
            Some(task) => task.description = desc,
            None => return Err(TodoError(format!("{} is not an existing task!", name))),
        }

        Ok(())
    }

    pub fn add(
        &mut self,
        name: String,
        prio: Option<Colour>,
        tag: Option<String>,
    ) -> Result<(), TodoError> {
        match self.tasks.iter().find(|task| task.name == name) {
            Some(_) => {
                return Err(TodoError(format!(
                    "Task with name {} already exists!",
                    name
                )))
            }
            None => {}
        }
        self.tasks.push(Task::new(name, prio, tag));
        Ok(())
    }

    pub fn remove(&mut self, name: String) {
        self.tasks.retain(|task| task.name != name);
    }

    pub fn complete(&mut self, name: &str) -> Result<(), TodoError> {
        match self.tasks.iter_mut().find(|task| task.name == name) {
            Some(task) => task.completed = true,
            None => {
                return Err(TodoError(format!(
                    "Task with name {} does not exist!",
                    name
                )))
            }
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        target: &str,
        mut name: Option<String>,
        tag: Option<String>,
        prio: Option<Colour>,
        completed: Option<bool>,
    ) -> Result<(), TodoError> {
        if name.is_some() {
            let temp_name = name.take().unwrap();
            match self.tasks.iter().find(|task| task.name == temp_name) {
                None => name = Some(temp_name),
                Some(_) => return Err(TodoError(format!("Task name '{}' already exists!", temp_name))),
            };
        }

        match self.tasks.iter_mut().find(|task| task.name == target) {
            None => return Err(TodoError(format!("Task with name {} does not exist!", target))),
            Some(task) => {
                if name.is_some() {
                    task.name = name.unwrap();
                };

                if tag.is_some() {
                    task.tag = tag.unwrap();
                }

                if prio.is_some() {
                    task.priority = prio.unwrap();
                }

                if completed.is_some() {
                    task.completed = completed.unwrap();
                }
            }
        }

        Ok(())
    }

    pub fn list(&self) {
        let mut printable_array: [Vec<&str>; 4] = [
            vec!["Name"],
            vec!["Tag"],
            vec!["Description"],
            vec!["Completed"],
        ];

        let mut colour_array: Vec<Colour> = vec![Colour::MagentaText];
        let tasks = &self.tasks;

        for task in tasks {
            printable_array[0].push(&task.name);
            printable_array[1].push(&task.tag);
            printable_array[2].push(&task.description);
            printable_array[3].push(match task.completed {
                true => "true",
                false => "false",
            });
            if task.completed {
                colour_array.push(Colour::GreenText);
            } else {
                colour_array.push(task.priority);
            }
        }

        _ = Printer::table_print(&printable_array, &colour_array);
    }

    pub fn list_tag(&self, tag: &str) {
        let mut printable_array: [Vec<&str>; 3] =
            [vec!["Name"], vec!["Description"], vec!["Completed"]];
        let mut colour_array: Vec<Colour> = vec![Colour::MagentaText];
        let tasks = &self.tasks;

        for task in tasks {
            if task.tag == tag {
                printable_array[0].push(&task.name);
                printable_array[1].push(&task.description);
                printable_array[2].push(match task.completed {
                    true => "true",
                    false => "false",
                });
                if task.completed {
                    colour_array.push(Colour::GreenText);
                } else {
                    colour_array.push(task.priority);
                }
            }
        }

        _ = Printer::table_print(&printable_array, &colour_array);
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_json = serde_json::to_string(&self.tasks)?;
        let mut path = String::new();
        match std::env::vars().find(|(var, _)| var == "TODORS_CACHE") {
            None => {
                path.push('.');
            }
            Some((_, _path)) => {
                path = _path;
            }
        };

        std::fs::write(path + "/todorstasks.json", serialized_json.as_bytes())?;
        Ok(())
    }
}

pub struct TodoError(pub String);

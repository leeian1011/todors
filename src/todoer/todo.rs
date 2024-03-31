#[derive(serde::Deserialize)]
struct Task {
    name: String,
    created_at: u64,
}

#[derive(serde::Deserialize)]
struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = String::new();
        match std::env::vars().find(|(var, _)| var == "TODORS_CACHE") {
            None => {
                path.push_str(".");
            }
            Some((_, _path)) => {
                path = _path;
            }
        };

        let tasks = std::fs::read(path + "/todorstasks.json")?;
        let tasks = String::from_utf8_lossy(&tasks);
        let tasks = serde_json::from_str::<Vec<Task>>(&tasks)?;
        Ok(Self { tasks })
    }
}

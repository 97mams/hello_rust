#[derive(Debug)]
pub struct Task {
    id: u32,
    title: String,
    completed: bool
}

impl Task {

    pub fn new(id: u32, title: String) -> Task {
        Task { id, title, completed: false }
    }

    pub fn is_completed(&self) -> bool {
        !self.completed
    }

    pub fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        let status = if task.completed { "[X]" } else { "[ ]" };
        println!("{}. {} {}", task.id, task.title, status);
    }
}

}   
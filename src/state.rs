use std::sync::Mutex;
use crate::models::Task;

// Define TaskList as a shared resource
pub type TaskList = Mutex<Vec<Task>>;

// Initialize the task list
pub fn init_task_list() -> TaskList {
    Mutex::new(vec![])
}

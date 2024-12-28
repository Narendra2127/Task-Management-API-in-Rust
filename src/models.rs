use serde::{Serialize, Deserialize};

// Define the Task structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

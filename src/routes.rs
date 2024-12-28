use rocket::{get, post, put, delete, routes};
use rocket::serde::json::Json;
use rocket::State;
use crate::models::Task;
use crate::state::TaskList;

#[post("/task", format = "json", data = "<new_task>")]
pub fn create_task(new_task: Json<Task>, state: &State<TaskList>) -> Json<Task> {
    let mut tasks = state.lock().unwrap();
    let task = new_task.into_inner();  
    tasks.push(task.clone());  
    Json(task)  
}


#[get("/tasks")]
pub fn get_tasks(state: &State<TaskList>) -> Json<Vec<Task>> {
    let tasks = state.lock().unwrap();
    Json(tasks.clone())
}

#[put("/task/<id>", format = "json", data = "<updated_task>")]
pub fn update_task(id: u32, updated_task: Json<Task>, state: &State<TaskList>) -> Json<Option<Task>> {
    let mut tasks = state.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        let task_clone = updated_task.into_inner();  
        *task = task_clone;  // Update the task with the cloned task
        return Json(Some(task.clone()));  // Return the updated task as a Json response
    }
    Json(None)  // If task not found, return None
}

#[delete("/task/<id>")]
pub fn delete_task(id: u32, state: &State<TaskList>) -> Json<Option<Task>> {
    let mut tasks = state.lock().unwrap();
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        return Json(Some(tasks.remove(index)));
    }
    Json(None)
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![create_task, get_tasks, update_task, delete_task]
}

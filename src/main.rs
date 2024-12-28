use rocket::{get, post, put, delete, routes, launch, State};

mod routes;  
mod models;  
mod state;   
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(state::init_task_list())
        .mount("/", routes::get_routes())
}

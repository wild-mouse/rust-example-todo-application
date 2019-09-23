#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tasks")]
fn get_tasks() -> &'static str {
    "Going to be able to get all tasks."
}

#[get("/tasks/<id>")]
fn get_task(id: String) -> &'static str {
    "Going to be able to get single task."
}

#[post("/tasks")]
fn add_task() -> &'static str {
    "Going to be able to add task."
}

#[put("/tasks/<id>")]
fn update_task(id: String) -> &'static str {
    "Going to be able to update task."
}

#[delete("/tasks/<id>")]
fn delete_task(id: String) -> &'static str {
    "Going to be able to delete task."
}

fn main() {
    rocket::ignite()
        .mount("/",
               routes![
                index,
                 get_tasks,
                  get_task,
                   add_task,
                    update_task,
                     delete_task,
                ])
        .launch();
}

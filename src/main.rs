mod api; 
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, get_user, delete_user, update_user, get_all_users}; //import the handler here
use repository::mongodb_repos::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![update_user])
        .mount("/", routes![get_all_users])
}


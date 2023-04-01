use rocket::Rocket;
#[path="../controllers/users.rs"]
pub mod users;

#[get("/all")]
fn get_users() -> &'static str {
    users::get_users()
}

#[get("/<id>")]
fn get_user_by_id(id: u32) -> String {
    users::get_user_by_id(id)
}

pub fn main(rocket: Rocket<rocket::Build>) -> Rocket<rocket::Build> {
    rocket.mount("/user", routes![get_users, get_user_by_id])
}
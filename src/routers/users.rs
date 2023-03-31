use rocket::Rocket;
#[get("/users")]
fn get_users() -> &'static str {
    "Hello, Users!"
}

#[get("/<id>")]
fn get_user(id: u32) -> String {
    format!("Hello, User {}!", id)
}

pub fn main(rocket: Rocket<rocket::Build>) -> Rocket<rocket::Build> {
    rocket.mount("/user", routes![get_users, get_user])
}
use rocket::Rocket;
pub mod users;

pub fn index_router(rocket: Rocket<rocket::Build>) -> Rocket<rocket::Build> {
    users::main(rocket)
}

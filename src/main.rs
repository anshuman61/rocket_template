#[macro_use] extern crate rocket;

pub mod routers;

#[get("/ping")]
fn ping() -> &'static str {
    "Pong"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut rocket = rocket::build()
        .mount("/", routes![ping]);
    rocket = routers::index_router(rocket);
    rocket.launch().await?;
    Ok(())
}
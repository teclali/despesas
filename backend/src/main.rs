#[macro_use]
extern crate rocket;

#[get("/<name>")]
fn world(name: &str) -> String {
    format!("Hello there, {name}")
}

#[post("/")]
fn hello() -> &'static str {
    "Hello world"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world, hello])
}

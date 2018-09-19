#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> String {
    "this is index".to_owned()
}

#[get("/<name>/<num>")]
fn greet(name: String, num: u8) -> String {
    format!("Hello, {} u8 {}.", name, num)
}

#[get("/<name>/<num>", rank=2)]
fn greet2(name: String, num: u32) -> String {
    format!("Hello, {} u32 {}.", name, num)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![greet, greet2])
        .launch();
}


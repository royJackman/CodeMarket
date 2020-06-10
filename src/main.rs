#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;

mod base;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

fn main() {
    rocket::ignite()
           .mount("/", routes![base::index])
           .attach(Template::fairing())
           .register(catchers![base::not_found])
           .launch();
}
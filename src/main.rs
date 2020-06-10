#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use std::fmt;
use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod base;
pub mod shop;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

fn main() {
    rocket::ignite()
           .mount("/", StaticFiles::from("templates"))
           .mount("/", routes![base::index])
           .attach(Template::fairing())
           .register(catchers![base::not_found])
           .launch();
}
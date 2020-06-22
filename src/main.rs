#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate nanoid;

#[cfg(test)] mod tests;

use rand::Rng;
use std::fmt;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod base;
mod ledger;
mod authorization;
pub mod shop;
pub mod purchase;
pub mod util;

fn main() {
    let mut session_ledger = ledger::Ledger::new();
    let mut ids = vec![];
    ids.push(session_ledger.register_vendor("New Vendor".to_string(), None).unwrap_or("".to_string()));
    ids.push(session_ledger.register_vendor("Old Vendor".to_string(), Some("oldies".to_string())).unwrap_or("".to_string()));
    ids.push(session_ledger.register_vendor("Hot Vendor".to_string(), None).unwrap_or("".to_string()));
    ids.push(session_ledger.register_vendor("Cold Vendor".to_string(), Some("icees".to_string())).unwrap_or("".to_string()));

    rocket::ignite()
           .manage( ledger::MutLedger{session_ledger: Arc::new(RwLock::new(session_ledger))} )
           .mount("/", StaticFiles::from("templates"))
           .mount("/", routes![base::index])
           .mount("/vendors", routes![shop::market_home, shop::vendor, purchase::purchase])
           .attach(Template::fairing())
           .register(catchers![base::not_found])
           .launch();
}
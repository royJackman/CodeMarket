#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate nanoid;

#[cfg(test)] mod tests;

use std::fmt;
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};

use config::*;
use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod base;
mod ledger;
mod authorization;
mod tera_functions;
pub mod shop;
pub mod purchase;
pub mod util;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("Config.toml")).unwrap();
        settings
    });
}

fn main() {
    let mut session_ledger = ledger::Ledger::new();
    let mut ids = vec![];
    ids.push(session_ledger.register_vendor(util::name_generator(), None).unwrap_or("".to_string()));
    ids.push(session_ledger.register_vendor(util::name_generator(), Some("oldies".to_string())).unwrap_or("".to_string()));
    ids.push(session_ledger.register_vendor(util::name_generator(), None).unwrap_or("".to_string()));
    ids.push(session_ledger.register_vendor(util::name_generator(), Some("icees".to_string())).unwrap_or("".to_string()));
    println!("{:#?}", ids);
    session_ledger.show_avg_prices();
    rocket::ignite()
           .manage( ledger::MutLedger{session_ledger: Arc::new(RwLock::new(session_ledger))} )
           .mount("/", StaticFiles::from("templates"))
           .mount("/", routes![base::index, authorization::register, purchase::http_purchase, purchase::form_purchase, purchase::purchase_page])
           .mount("/vendors", routes![shop::market_home, shop::vendor])
           .attach(Template::custom(|engines| {
               let var = BTreeMap::new();
               engines.tera.register_function("catchphrase_generator", tera_functions::make_catchphrase_generator());
               engines.tera.register_function("get_rust_type_index", tera_functions::make_get_rust_type_index(var.clone()));
               engines.tera.register_function("intparse", tera_functions::make_intparse(var.clone()));
           }))
           .register(catchers![base::not_found])
           .launch();
}
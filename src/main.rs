#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate nanoid;

#[cfg(test)] mod tests;

use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};

use config::*;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::de;

mod authorization;
mod base;
mod ledger;
mod tera_functions;

pub mod purchase;
pub mod shop;
pub mod util;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new({
        let mut options = Config::default();
        options.merge(File::with_name("Config.toml")).unwrap();
        options
    });
}

/// Try to get a value from the config file
/// 
/// # Arguments
/// 
/// * `key` - The key to check in the config
pub fn get_config<T: de::DeserializeOwned>(key: &str) -> Option<T> {
    match CONFIG.read().unwrap().get::<T>(key) {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

/// Rust market launching point. This function starts the application, default
/// port 8000. All settings can be changed using `Config.toml`
/// 
/// # Config options
/// 
/// * `generated_vendors`   - Number of AI vendors to generate
#[allow(unused_variables, unused_assignments)]
fn main() {
    let mut session_ledger = ledger::Ledger::new();
    let mut ids = vec![];
    match get_config::<usize>("generated_vendors") {
        Some(gv) => {
            for mut i in 0..gv {
                match session_ledger.register_vendor(util::name_generator(), None) {
                    Ok(id) => { ids.push(id); }, Err(_) => { i -= 1; }
                }
            }
        },
        None => {
            ids.push(session_ledger.register_vendor(util::name_generator(), None).unwrap_or("".to_string()));
            ids.push(session_ledger.register_vendor(util::name_generator(), Some("oldies".to_string())).unwrap_or("".to_string()));
            ids.push(session_ledger.register_vendor(util::name_generator(), None).unwrap_or("".to_string()));
            ids.push(session_ledger.register_vendor(util::name_generator(), Some("icees".to_string())).unwrap_or("".to_string()));
        }
    }

    println!("{:#?}", ids);
    session_ledger.show_avg_prices();
    
    rocket::ignite()
           .manage( ledger::MutLedger{session_ledger: Arc::new(RwLock::new(session_ledger))} )
           .mount("/", StaticFiles::from("templates"))
           .mount("/", routes![
               authorization::register, 
               base::index,
               purchase::form_purchase, 
               purchase::purchase_page,
               shop::form_stock,
               shop::stock_page])
           .mount("/api", routes![
               ledger::request_ledger_state,
               ledger::request_vendor_names,
               ledger::request_vendor_urls,
               purchase::http_purchase,
               shop::http_stock])
           .mount("/vendors", routes![
               shop::market_home, 
               shop::vendor])
           .attach(Template::custom(|engines| {
               let var = BTreeMap::new();
               engines.tera.register_function("catchphrase_generator", tera_functions::make_catchphrase_generator());
               engines.tera.register_function("get_rust_type_index", tera_functions::make_get_rust_type_index(var.clone()));
               engines.tera.register_function("intparse", tera_functions::make_intparse(var.clone()));
           }))
           .register(catchers![
               base::bad_request,
               base::internal_error,
               base::not_found])
           .launch();
}
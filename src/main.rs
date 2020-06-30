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
use rocket_contrib::templates::tera::{GlobalFn, Value, Error, from_value, to_value};

mod base;
mod ledger;
mod authorization;
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

fn make_intparse(_num: BTreeMap<String, String>) -> GlobalFn {
    Box::new(move |args| -> Result<Value, Error> {
        match args.get("num") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => Ok(to_value(v.parse::<i32>().unwrap()).unwrap()),
                Err(_) => Err("Input `num` is not an integer".into()),
            },
            None => Err("Input `num` not provided".into()),
        }
    })
}

fn make_catchphrase_generator() -> GlobalFn {
    Box::new(move |_args| -> Result<Value, Error> {
        Ok(to_value(util::catchphrase_generator()).unwrap())
    })
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
               let num = BTreeMap::new();
               engines.tera.register_function("intparse", make_intparse(num));
               engines.tera.register_function("catchphrase_generator", make_catchphrase_generator());
           }))
           .register(catchers![base::not_found])
           .launch();
}
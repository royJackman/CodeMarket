#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rand::Rng;
use std::fmt;
use std::collections::HashMap;
use std::sync::Mutex;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod base;
pub mod shop;
pub mod purchase;
pub mod util;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

#[derive(Serialize)]
pub struct Market {
    pub vendors: Mutex<Vec<shop::Vendor>>
}

impl Market {
    pub fn new() -> Market {
        Market{ vendors: Mutex::new(vec![]) }
    }
    
    fn spawn_vendors(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for i in 0..count {
            let mut temp_vendor = shop::Vendor::new(format!("Vendor{}", i), format!("vendor_{}", i), rng.gen_range(700.0, 1300.0));
            for j in 0..rng.gen_range(4,7) {
                temp_vendor.items.push(shop::Item::new(
                    format!("item_{}", j), 
                    ((rng.gen_range(1.0, 10.0) as f64) * 100.0).round() / 100.0, 
                    rng.gen_range(30, 70)))
            }
            self.vendors.get_mut().unwrap().push(temp_vendor);
        }
    }

    pub fn get_vendor(&mut self, name: &String) -> Option<&shop::Vendor> {
        self.vendors.get_mut().unwrap().iter().find(|v| &v.name == name)
    }
    pub fn get_vendor_by_url(&mut self, url: &String) -> Option<&shop::Vendor> {
        self.vendors.get_mut().unwrap().iter().find(|v| &v.url == url)
    }
}


fn main() {
    let mut market = Market::new();
    market.spawn_vendors(5);
    rocket::ignite()
           .manage( market )
           .mount("/", StaticFiles::from("templates"))
           .mount("/", routes![base::index])
           .mount("/vendors", routes![shop::vender_home, shop::vendor, purchase::purchase])
           .attach(Template::fairing())
           .register(catchers![base::not_found])
           .launch();
}
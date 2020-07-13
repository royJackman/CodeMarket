use std::collections::BTreeMap;
use std::fmt::Display;

use rocket::response::content;
use rocket::request::{Form, FormError};
use rocket::State;

#[derive(Debug, FromForm)]
pub struct Registration {
    pub vendor_name: String,
    pub vendor_url: String
}

/// The endpoint for registering new vendors using a JSON object. When a valid
/// new vendor is registered, a session-specific UUID is returned, and it must
/// be used to validate purchase requests.
/// 
/// # Arguments
/// 
/// * `registration_data`   - JSON object with registration info
/// * `ledger`              - The current ledger state
#[post("/register", data="<registration>")]
pub fn register(registration: Result<Form<Registration>, FormError<'_>>, ledger: State<super::ledger::MutLedger>) -> content::Json<String> {
    let mut output_vars: BTreeMap<String, Box<dyn Display>> = BTreeMap::new();

    let registration = match registration {
        Ok(r) => r.into_inner(),
        Err(_) => {
            output_vars.insert("Registration".to_string(), Box::new("incorrectly formatted".to_string()));
            return super::util::construct_json(&output_vars)
        }
    };

    if registration.vendor_name == "".to_string() {
        output_vars.insert("vendor_name".to_string(), Box::new("is empty".to_string()));
        return super::util::construct_json(&output_vars)
    }

    let arc_ledger = ledger.inner().session_ledger.clone();
    {
        let ledger = &*arc_ledger.read().unwrap();
        for vendor in ledger.get_vendors().iter() {
            if vendor.name == registration.vendor_name {
                output_vars.insert("vendor_name".to_string(), Box::new("is in use".to_string()));
            }
            if vendor.url == registration.vendor_url {
                output_vars.insert("vendor_name".to_string(), Box::new("is in use".to_string()));
            }
        }
    }
    if output_vars.len() > 0 {
        return super::util::construct_json(&output_vars)
    } else {
        let url = match registration.vendor_url.contains(":") {
            true => None, false => Some(registration.vendor_url)
        };
        let mut ledger = (*arc_ledger).write().unwrap();
        match ledger.register_vendor(registration.vendor_name, url) {
            Ok(uuid) => output_vars.insert("uuid".to_string(), Box::new(uuid)),
            Err(_) => output_vars.insert("error".to_string(), Box::new("registration unsuccessful, aborting"))
        };
        return super::util::construct_json(&output_vars)
    }
}
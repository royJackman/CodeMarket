use std::io::{self, Read};
use std::fmt::Display;
use std::collections::BTreeMap;

use rocket::http::Status;
use rocket::response::content;
use rocket::{State, Request, Data, Outcome::*};
use rocket::data::{FromData, Outcome, Transform, Transformed};

const BUFFER_SIZE: u64 = 256;

pub enum AuthorizationError {
    ExistingVendor,
    Io(io::Error)
}

#[derive(Debug)]
pub struct Registration {
    pub vendor_name: String,
    pub vendor_url: String
}

impl Registration {
    pub fn from_data(data: RegistrationData) -> Registration {
        let clean_string = |string: &str| String::from(string).replace("\"", "").replace(",", "").replace("\r", "");
        Registration {
            vendor_name: clean_string(data.vendor_name),
            vendor_url: clean_string(data.vendor_url)
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct RegistrationData<'a> {
    pub vendor_name: &'a str,
    pub vendor_url: &'a str
}

impl<'a> FromData<'a> for RegistrationData<'a> {
    type Error = AuthorizationError;
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open().take(BUFFER_SIZE);
        let mut string = String::with_capacity((BUFFER_SIZE/2) as usize);
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(e) => Failure((Status::InternalServerError, Self::Error::Io(e)))
        };

        Transform::Borrowed(outcome)
    }

    fn from_data(_: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        let string = outcome.borrowed()?;
        let tabboo: Vec<&str> = vec![&r"{", &r"}"];
        let whitespace: Vec<&str> = vec![&"", &"\n"];
        let splits: Vec<&str> = string.trim()
                                      .split("\n")
                                      .filter(move |x| {
                                          for t in tabboo.iter() {
                                              if x.contains(t) { return false }
                                          }
                                          for w in whitespace.iter() {
                                              if x == w { return false }
                                          }
                                          true
                                      })
                                      .collect();
        let to_remove: &[_] = &[',', '\n'];
        Success(RegistrationData{
            vendor_name: &splits[0].trim().trim_matches(to_remove).split("\"").filter(|x| x != &"").collect::<Vec<&str>>().last().unwrap(),
            vendor_url: &splits[1].trim().trim_matches(to_remove).split("\"").filter(|x| x != &"").collect::<Vec<&str>>().last().unwrap()
        })
    }
}

#[post("/register", format="application/json", data="<registration_data>")]
pub fn register(registration_data: RegistrationData, ledger: State<super::ledger::MutLedger>) -> content::Json<String> {
    let mut output_vars: BTreeMap<String, Box<dyn Display>> = BTreeMap::new();
    let registration = Registration::from_data(registration_data);
    if registration.vendor_name == "".to_string() {
        output_vars.insert("vendor_name".to_string(), Box::new("is empty".to_string()));
        return super::util::construct_json(&output_vars)
    }

    let arc_ledger = ledger.inner().session_ledger.clone();
    {
        let ledger = &*arc_ledger.read().unwrap();
        for vendor in ledger.vendors.lock().unwrap().iter() {
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
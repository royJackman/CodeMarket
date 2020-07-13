use rocket_contrib::templates::tera::{GlobalFn, Value, Error, from_value, to_value};
use std::collections::BTreeMap;
use super::util;

/// Tera function for generating a catchphrase for the individual vendor pages
pub fn make_catchphrase_generator() -> GlobalFn {
    Box::new(move |_args| -> Result<Value, Error> {
        Ok(to_value(super::util::catchphrase_generator()).unwrap())
    })
}

/// Tera function to find the index of a type in the static constant list
/// 
/// # Arguments
/// 
/// * `_data`   - Placeholder for data
pub fn make_get_rust_type_index(_data: BTreeMap<String, String>) -> GlobalFn {
    Box::new(move |args| -> Result<Value, Error> {
        match args.get("data") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) => Ok(to_value(util::get_rust_type_index(v.parse::<String>().unwrap())).unwrap()),
                Err(_) => Err("Input `data` is not a string".into())
            },
            None => Err("Input `data` not provided".into())
        }
    })
}

/// Tera function to parse an integer
/// 
/// # Arguments
/// 
/// * `_num`    - Placeholder for data
pub fn make_intparse(_num: BTreeMap<String, String>) -> GlobalFn {
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
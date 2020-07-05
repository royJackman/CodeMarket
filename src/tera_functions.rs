use super::util;
use std::collections::BTreeMap;
use rocket_contrib::templates::tera::{GlobalFn, Value, Error, from_value, to_value};

pub fn make_catchphrase_generator() -> GlobalFn {
    Box::new(move |_args| -> Result<Value, Error> {
        Ok(to_value(super::util::catchphrase_generator()).unwrap())
    })
}

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
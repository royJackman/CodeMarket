use std::fmt::Display;
use std::collections::BTreeMap;
use rocket::response::content;

pub fn construct_json(values: &BTreeMap<String, Box<dyn Display>>) -> content::Json<String> {
    let mut format_string = String::from("{ ");

    for (k, v) in values.iter() {
        format_string.push_str("\"");
        format_string.push_str(&k);
        format_string.push_str("\": \"");
        format_string.push_str(&format!("{}", &v));
        format_string.push_str("\", ");
    }

    format_string = String::from(&format_string[..(format_string.len() - 2)]);
    format_string.push_str(" }");

    content::Json(format_string)
}
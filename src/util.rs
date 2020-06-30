use std::fmt::Display;
use std::collections::BTreeMap;

use rand::Rng;
use rand::seq::SliceRandom;
use rocket::response::content;

const RUST_TYPES: &'static [&'static str] = &["bool", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "str", "char", "never"];
const ADJECTIVES: &'static [&'static str] = &["Dry", "Oafish", "Unusual", "Extra-Large", "Thirsty", "Alluring", "Bewildered", "Steadfast", "Rotund", "Unsightly", "Swanky", "Majestic", "Torpid", "Capricious", "Vacuous", "Exclusive"];
const OCCUPATION: &'static [&'static str] = &["Vendor", "Dealer", "Merchant", "Hawker", "Peddler", "Huckster", "Clerk", "Salesperson", "Trader", "Agent", "Chef", "Agriculturalist", "Pusher", "Capitalist", "Suspect", "Seal Clubber"];
const CATCHPHRASE: &'static [&'static str] = &["Best prices this side of the http", "1-800-bits4bytes", "Your favorite fiscal fenagler!", "Friendly neighborhood objectivism-man", "My manager \"produces\" value", "Great sales for the highest bidder!", "Being dead inside makes you more aerodynamic!", "Ask not what your vendor can sell to you, ask what you can buy from your vendor!", "Buy, or buy not, there is no haggle.", "Life is a box of types", "This is exactly how the dark web works", "Imagine how much easier this would be if we had healthcare"];

/// Converts displayable values into JSON object for shipping
/// 
/// # Arguments
/// 
/// * `values`  - BTreeMap of Strings to printable (Display) objects
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

pub fn catchphrase_generator() -> String {
    let mut rng = rand::thread_rng();
    format!("{}", CATCHPHRASE[rng.gen_range(0, CATCHPHRASE.len())])
}

pub fn name_generator() -> String {
    let mut rng = rand::thread_rng();
    format!("{} {}", ADJECTIVES[rng.gen_range(0, ADJECTIVES.len())], OCCUPATION[rng.gen_range(0, OCCUPATION.len())])
}

pub fn get_rust_types(count: usize) -> Vec<&'static str> { 
    let mut rng = rand::thread_rng();
    RUST_TYPES.choose_multiple(&mut rng, count).cloned().collect()
}
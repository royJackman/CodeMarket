use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;

use rand::Rng;
use rand::seq::SliceRandom;
use rocket::response::content;

const ADJECTIVES: &'static [&'static str] = &["Dry", "Oafish", "Unusual", "Extra-Large", "Thirsty", "Alluring", "Bewildered", "Steadfast", "Rotund", "Unsightly", "Swanky", "Majestic", "Torpid", "Capricious", "Vacuous", "Exclusive"];
const CATCHPHRASE: &'static [&'static str] = &["Best prices this side of the http", "1-800-bits4bytes", "Your favorite fiscal fenagler!", "Friendly neighborhood objectivism-man", "My manager \"produces\" value", "Great sales for the highest bidder!", "Being dead inside makes you more aerodynamic!", "Ask not what your vendor can sell to you, ask what you can buy from your vendor!", "Buy, or buy not, there is no haggle.", "Life is a box of types", "This is exactly how the dark web works", "Imagine how much easier this would be if we had healthcare"];
const OCCUPATION: &'static [&'static str] = &["Vendor", "Dealer", "Merchant", "Hawker", "Peddler", "Huckster", "Clerk", "Salesperson", "Trader", "Agent", "Chef", "Agriculturalist", "Pusher", "Capitalist", "Suspect", "Seal Clubber"];
const RUST_TYPES: &'static [&'static str] = &["bool", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "str", "char", "never"];

/// Returns a random catchphrase from a static list
pub fn catchphrase_generator() -> String {
    let mut rng = rand::thread_rng();
    format!("{}", CATCHPHRASE[rng.gen_range(0, CATCHPHRASE.len())])
}

/// Converts values into JSON object for shipping
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

/// Converts minimal HashMap of prices (only containing items currently tracked
/// by the ledger) into an ordered vector containing zeros in place of any
/// untracked items
/// 
/// # Arguments
/// 
/// * `avg_prices`  - A mapping from item to average price
pub fn convert_minimal_to_full(avg_prices: HashMap<String, f64>) -> Vec<f64> {
    let mut vec_prices = vec![0.0; RUST_TYPES.len()];
    for (i, t) in RUST_TYPES.iter().enumerate() {
        if avg_prices.contains_key(&t.to_string()){
            vec_prices[i] = avg_prices[&t.to_string()];
        }
    }
    vec_prices
}

/// Gets the index of an item according to the RUST_TYPES static vector
/// 
/// # Arguments
/// 
/// * `t`   - The item name in question
pub fn get_rust_type_index(t: String) -> usize { RUST_TYPES.iter().position(|&x| x == &*t).unwrap() }

/// Gets a random set of types. If count is zero, returns copy of types list
/// 
/// # Arguments
/// 
/// * `count`   - The number of types to return, use 0 for full list
pub fn get_rust_types(count: usize) -> Vec<&'static str> { 
    if count == 0 { return RUST_TYPES.to_vec(); }
    let mut rng = rand::thread_rng();
    RUST_TYPES.choose_multiple(&mut rng, count).cloned().collect()
}

/// Generates a random name using the static name lists
pub fn name_generator() -> String {
    let mut rng = rand::thread_rng();
    format!("{} {}", ADJECTIVES[rng.gen_range(0, ADJECTIVES.len())], OCCUPATION[rng.gen_range(0, OCCUPATION.len())])
}
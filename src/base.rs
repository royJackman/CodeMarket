use rocket::State;
use rocket_contrib::templates::Template;
use serde_json::to_value;
use super::ledger::MutLedger;

/// Index page for the application. Contains overview information for the
/// current market state and price distribution.
/// 
/// # Arguments
/// 
/// * `ledger`  - The current ledger state
#[get("/")]
pub fn index(ledger: State<MutLedger>) -> Template {
    let mut map = super::HashMap::new();
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    map.insert("urls", to_value(ledger.get_vendor_urls()).unwrap());
    map.insert("names", to_value(ledger.get_vendor_names()).unwrap());
    let mut types = ledger.get_ledger_items();
    types.sort();
    map.insert("types", to_value(types).unwrap());
    map.insert("history", to_value(ledger.get_price_history()).unwrap());
    map.insert("ticker_items", to_value(vec!["Welcome to CodeMarket!".to_string(), 
                                             "Your one-stop shop for types from all over the Internet!".to_string(), 
                                             "Don't forget to inform your local ledger with every purchase!".to_string()]).unwrap());
    Template::render("index", &map)
}

/// 404 error catcher. 
/// 
/// # Arguments
/// 
/// * `req` - The current http request information
#[catch(404)]
pub fn not_found(req: &super::Request<'_>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
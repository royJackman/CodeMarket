use rocket::State;
use super::ledger::MutLedger;
use rocket_contrib::templates::Template;

//Index page endpoint
#[get("/")]
pub fn index(ledger: State<MutLedger>) -> Template {
    let mut map = super::HashMap::new();
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    map.insert("urls", ledger.get_vendor_urls());
    map.insert("names", ledger.get_vendor_names());
    map.insert("ticker_items", vec!["Welcome to CodeMarket!".to_string(), 
                                    "Your one-stop shop for types from all over the Internet!".to_string(), 
                                    "Don't forget to inform your local ledger with every purchase!".to_string()]);
    Template::render("index", &map)
}

//Page not found catcher
#[catch(404)]
pub fn not_found(req: &super::Request<'_>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
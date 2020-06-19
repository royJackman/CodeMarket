use rocket::State;
use super::ledger::Ledger;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(ledger: State<Ledger>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("market", ledger.inner());
    Template::render("index", &map)
}

#[catch(404)]
pub fn not_found(req: &super::Request<'_>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
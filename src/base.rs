use rocket::State;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(market: State<super::Market>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("market", market.inner());
    Template::render("index", &map)
}

#[catch(404)]
pub fn not_found(req: &super::Request<'_>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    let context = super::TemplateContext { 
        name: String::from("Steve"), 
        items: vec!["Flask", "Lightsaber", "Sticky notes"]
    };
    Template::render("index", &context)
}

#[catch(404)]
pub fn not_found(req: &super::Request<'_>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
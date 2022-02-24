use rocket::response::Redirect;
use serde::Serialize;

use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct Context {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/tera", hello(name = "Your Name")))
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "index",
        Context {
            title: "Hello",
            name: Some(String::from(name)),
            items: vec!["One", "Two", "Three"],
        },
    )
}

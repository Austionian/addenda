#[macro_use]
extern crate rocket;

mod tera;

use rocket::response::content::Html;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r#"Find the addenda <a href="tera">here</a>."#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/tera", routes![tera::index, tera::hello])
        .attach(Template::fairing())
}

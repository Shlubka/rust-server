#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket_dyn_templates::{context, Template};
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserData {
    language: String,
    code: String,
    promo: String,
}

#[post("/lang", data = "<data>")]
async fn get_keys(data: Json<UserData>) -> Redirect {
    let mut static_promo = HashMap::new();
    let mut found_promo = false;
    static_promo.insert("admin", "Kirill");
    rocket::info!(
        "Received data: language = {}, code = {}, promo = {:?}",
        data.language,
        data.code,
        data.promo
    );

    let language = data.language.to_string();
    let code = data.code.to_string();
    let promo = data.promo.to_string();

    /*for (key, value) in static_promo.iter() {
        if promo == *key {
            return Redirect::to(uri!(wellcom(value.to_string())));
        }
    }*/

    let local_link = format!("/lang/{}?code={}&promo={}", language, code, promo);
    rocket::info!("Redirect to {local_link}");

    Redirect::to(uri!(start_logick(language, code, promo)))
}

#[get("/wellcom/<user>")]
async fn wellcom(user: String) -> Template {
    let context = context! {
        name: user,
    };
    Template::render("wellcom", &context)
}

#[get("/lang/<lang>?<code>&<promo>")]
async fn start_logick(lang: &str, code: &str, promo: &str) -> String {
    rocket::info!("Enter in logika");
    format!(
        "Lang = \"{}\"; \nCode = \"{}\" \nPromoCode = \"{}\"",
        lang, code, promo
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![get_keys, wellcom, start_logick])
        .mount("/", FileServer::from(relative!("/static")))
}

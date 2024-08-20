#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserData<'r> {
    language: &'r str,
    code: &'r str,
    promo: Option<&'r str>,
}

#[post("/lang", data = "<data>")]
fn get_keys(data: Json<UserData<'_>>) -> Redirect {
    rocket::info!(
        "Received data: language = {}, code = {}, promo = {:?}",
        data.language,
        data.code,
        data.promo
    );

    let language = data.language.to_string();
    let code = data.code.to_string();
    let promo = data.promo.map(|s| s.to_string()).unwrap_or_default();

    let local_link = format!("/lang/{}?code={}&promo={}", language, code, promo);
    rocket::info!("Redirect to {local_link}");

    Redirect::to(uri!(start_logick(language)))
}

#[get("/lang/<lang>")]
fn start_logick(lang: &str) -> String {
    rocket::info!("Enter in logika",);
    format!(
        "Lang = \"{}\"; \nCode = \"{}\" \nPromoCode = \"{}\"",
        lang, "code", "promo"
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![/*index, home,, files*/ get_keys, start_logick])
        .mount("/", FileServer::from(relative!("static")))
}

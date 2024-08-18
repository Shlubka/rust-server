#[macro_use]
extern crate rocket;

use rocket::http::uri::Uri;
use rocket::serde::{json::Json, Deserialize};
use rocket::{fs::NamedFile, response::Redirect};
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserData<'r> {
    language: &'r str,
    code: &'r str,
    promo: Option<&'r str>,
}

#[get("/")]
fn index() -> Redirect {
    let redirect = Redirect::to(uri!("/index"));
    redirect
}

#[get("/index")]
async fn home() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}

#[post("/lang", data = "<data>")]
fn get_keys(data: Json<UserData<'_>>) -> Redirect {
    // Log the received data
    rocket::info!(
        "Received data: language = {}, code = {}, promo = {:?}",
        data.language,
        data.code,
        data.promo
    );

    // Extract fields from the JSON payload
    let language = data.language.to_string();
    let code = data.code.to_string();
    let promo = data.promo.map(|s| s.to_string()).unwrap_or_default();

    // Construct the redirect URI
    let local_link = format!("/lang/{}?code={}&promo={}", language, code, promo);
    rocket::info!("Redirect to {local_link}");

    // Redirect to the constructed URI
    Redirect::to(uri!(start_logick(language, code, promo)))
}

#[get("/lang/<lang>?<code>&<promo>")]
fn start_logick(lang: &str, code: Option<&str>, promo: Option<&str>) -> String {
    // Return a formatted string containing the query parameters
    let code = code.unwrap_or("");
    let promo = promo.unwrap_or("");
    rocket::info!("Enter in logika",);
    format!(
        "Lang = \"{}\"; \nCode = \"{}\" \nPromoCode = \"{}\"",
        lang, code, promo
    )
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, home, get_keys, files, start_logick])
}

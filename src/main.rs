#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket_dyn_templates::{context, Template};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;
use chrono::Local;

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
async fn start_logick(lang: &str, code: &str, promo: &str) -> Result<Redirect, io::Error> {
    rocket::info!("Enter in logika");

    let file_path = processing(lang, code).await?;
    let file_str = file_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file path"))?;

    Ok(Redirect::to(uri!(upload_file("test.json"))))
}

#[get("/<path>")]
async fn upload_file(path: &str) -> Result<NamedFile, io::Error> {
    println!("path: {path}");
    NamedFile::open(path).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![get_keys, wellcom, start_logick, upload_file])
        .mount("/", FileServer::from(relative!("/static")))
}

async fn processing(lang: &str, code: &str) -> Result<PathBuf, io::Error> {
    // Получаем текущее время
    let now = Local::now();
    let timestamp = format!(
        "{}-{}-{}-{}-{}-{}-{}",
        now.year_ce(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.timestamp_subsec_millis()
    );
    let file_name = "example";
    let initial_extension = match lang {
        "C" => "c",
        "Rust" => "rs",
        "Java" => "java",
        "C++" => "cpp",
        _ => "broken",
    };

    let file_path = format!("{}-{}.{}", timestamp, file_name, initial_extension);

    // Записываем код в файл
    fs::write(&file_path, code).await?;

    // Выполняем команду
    let output = Command::new("./json-compiler")
        .arg(lang)
        .arg(&file_path)
        .output()
        .await?;

    if output.status.success() {
        Ok(PathBuf::from(file_path))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Command failed"))
    }
}

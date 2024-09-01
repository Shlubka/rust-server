#[macro_use]
extern crate rocket;

use chrono::prelude::*;
use rocket::fs::FileServer;
//use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket_dyn_templates::{context, Template};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;

use rocket::http::Status;
use rocket_download_response::DownloadResponse;
use std::{io::ErrorKind, path::Path};

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

    let output_file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let output_file_path = format!("outfiles/{}.json", output_file_name);
    Ok(Redirect::to(uri!(download_file(output_file_path))))
}

/*#[get("/<path>")]
async fn download_file(path: &str) -> Result<NamedFile, io::Error> {
    println!("path: {path}");
    NamedFile::open(path).await
}*/

#[get("/<path>")]
async fn download_file(path: &str) -> Result<DownloadResponse, Status> {
    //let path = Path::join(Path::new("examples"), Path::join(Path::new("images"), "image(貓).jpg"));
    let path1 = Path::new(path);

    DownloadResponse::from_file(path1, None::<String>, None)
        .await
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                Status::NotFound
            } else {
                Status::InternalServerError
            }
        })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![get_keys, wellcom, start_logick, download_file])
        .mount("/", FileServer::from("/home/webserv/webpenis/static")) //ментяь перед отправкой на сервер
                                                                       /*.mount(
                                                                           "/",
                                                                           FileServer::from("/home/kira/webpenis/rust-server/static"),
                                                                       ) */
}

async fn processing(lang: &str, code: &str) -> Result<PathBuf, io::Error> {
    // Получаем текущее время
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%Y-%m-%d-%H-%M-%S-%3f").to_string();
    let file_name = "example";
    let initial_extension = match lang {
        "C" => "c",
        "Rust" => "rs",
        "Java" => "java",
        "C++" => "cpp",
        _ => "broken",
    };

    let file_path = format!("input/{}-{}.{}", timestamp, file_name, initial_extension);

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

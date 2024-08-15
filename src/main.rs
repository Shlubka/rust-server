#[macro_use] extern crate rocket;


use rocket::{fs::NamedFile, response::Redirect};
use rocket::serde::{Deserialize, json::Json};
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserData<'r> {
    language: &'r str,
    code: &'r str,
    promo: &'r str,
}

#[get("/")]
fn index() -> Redirect {
    let redirect = Redirect::to(uri!("/index"));
    redirect
}

#[get("/index")]
async fn home () -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}


#[post("/lang", data = "<data>")]
fn get_keys(data: Json<UserData<'_>>) -> String {
    format!("Hello, world! on {}! with code: \"{}\" and promoocode {}",data.language, data.code, data.promo)
}


#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("").join(file)).await.ok()
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, home, get_keys, files])
}

/*
 * Rust Backend Developement Part 1 - How to create an API Endpoint in Rocket.rs
 * - Starting a Rocket project
 * - Variables inicluding optional query variables
 * - Redirection
 * - Returning Json
 * - Returning Two Responses
*/

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};

#[macro_use] 
extern crate rocket;

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(TAURI_RELEASES_PREFIX, google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)))
}

// tauri-releases/google-keep-dektop/win64/1.18.0?msg=""
#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
fn google_keep_desktop_api(_platform: &str, current_version: &str, msg: Option<&str>) -> Result<Value, Status> {
    //Status::NoContent
    //error prone logic -> Option / Result

    if let Some(msg) = msg {
        println!("{msg}");
        return Err(Status::NoContent);
    }

    Ok(json!({
        "notes" : "IT Works"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/tauri-releases", routes![google_keep_desktop_api])
}


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

mod utils;

type ID = usize;

#[derive(FromForm)]
struct UserInput<'f> {
    // The raw, undecoded value. You _probably_ want `String` instead.
    value: &'f RawStr,
}

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String,
}

//#[get("/")]
//fn index() -> &'static str {
//    "Hello, world."
//}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "ig" => utils::instagram::construct_instagram_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

#[get("/get/<id>")]
fn get(id: ID) -> Json<Message> {
    Json(Message {
        id: Some(id),
        contents: "Hello from json".to_string(),
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[post("/submit", data = "<user_input>")]
fn submit_task(user_input: Form<UserInput>) -> String {
    format!("Your value: {}", user_input.value)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![search, submit_task, get])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}

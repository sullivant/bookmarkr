#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;

mod utils;

#[derive(FromForm)]
struct UserInput<'f> {
    // The raw, undecoded value. You _probably_ want `String` instead.
    value: &'f RawStr,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world."
}

#[get("/links")]
fn get_links() -> &'static str {
    "this is a list of links."
}

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

#[post("/submit", data = "<user_input>")]
fn submit_task(user_input: Form<UserInput>) -> String {
    format!("Your value: {}", user_input.value)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, search, get_links, submit_task])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        //.mount("/links", routes![links])
        .launch();
}

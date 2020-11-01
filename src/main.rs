use serde::{Deserialize, Serialize};
use isahc::ResponseExt;

mod update;

// Create a version struct allowing us to convert from a string
pub struct Version {

    major: i32,
    minor: i32,
    rev: i32,
    build: i32
}

// How our update data will be laid out
#[derive(Serialize, Deserialize)]
struct Update {

    repo: String,
    tag: String,
    version: String
}

#[derive(Serialize, Deserialize)]
struct Updates {

    updates: Vec<Update>
}

fn main() {

    let updates = get_updates();

    println!("repo is \"{}\", tag is \"{}\", version is \"{}\"", updates.updates[0].repo, updates.updates[0].tag, updates.updates[0].version);
}

fn get_updates() -> Updates {

    let updates_url: &str = "https://raw.githubusercontent.com/InnoxiumTech/CandorManager/dev/updates/updates.json";

    let mut response = isahc::get(updates_url).unwrap();
    let body = response.text().unwrap();

    return serde_json::from_str(&*body).unwrap();
}
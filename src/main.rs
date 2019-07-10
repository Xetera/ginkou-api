#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use] 
extern crate serde_derive;
#[macro_use] 
extern crate rocket_contrib;

extern crate structopt;
extern crate rusqlite;
extern crate mecab;
use rocket::http::route::Error;
use rocket::http::RawStr;

use rocket_contrib::json::{Json, JsonValue};

mod language;

#[derive(Serialize, Deserialize)]
struct SearchResponse {
  sentence: Option<String>
}

#[get("/lookup/<name>")]
fn lookup(name: &RawStr) -> Result<JsonValue, rusqlite::Error> {
  let path = ".ginkoudb";
  let conn = language::conn_from_disk(path)?;
  let search_target = &name.percent_decode()?;
  let result = language::matching_word(&conn, search_target, false)?;

  return Ok(
    json!({ "sentences": result })
  );
}

#[get("/")]
fn index() {
  "Ginkou API\nUsage: /lookup/<word>\nExample: /lookup/僕"
}


fn main() {
  rocket::ignite().mount("/", routes![index, lookup]).launch();
}
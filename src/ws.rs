use crate::ncda;

use rocket::config::{Config, Environment};
use serde_json::{Value};
use rocket_contrib::json::Json;
use std::collections::HashMap;


// http://0.0.0.0:8080/v1/check?ids=cb32752361d,cb32752361d,cb32752361d
#[get("/v1/check?<ids>")]
fn check(ids: String) -> Json<Vec<Value>> {
    let ids = ids.split(",").into_iter().map(|id| {
        match ncda::check(id) {
            Ok(()) => {
                let mut map = HashMap::new();

                map.insert(id, true);

                serde_json::to_value(&map).expect("Failed to deserialize data")
            },
            Err(e) => {
                let mut map = HashMap::new();

                map.insert(id, e);

                serde_json::to_value(&map).expect("Failed to deserialize error")
            }
        }
    }).collect::<Vec<Value>>();

    Json(ids)
}

// http://0.0.0.0:8080/v1/checksum?ids=cb32752361d,cb32752361d,cb32752361d
#[get("/v1/checksum?<ids>")]
fn checksum(ids: String) -> Json<Vec<Value>> {
    let result = ids.split(",").into_iter().map(|id| {
        
        match ncda::checksum(id) {
            Ok(checksum_char) => {
                let mut map = HashMap::new();

                map.insert(id, checksum_char);

                serde_json::to_value(&map).expect("Failed to deserialize data")
            },
            Err(e) => {
                let mut map = HashMap::new();

                map.insert(id, e);

                serde_json::to_value(&map).expect("Failed to deserialize error")
            }
        }
    }).collect::<Vec<Value>>();

    Json(result)
}

pub fn launch_ws(port: Option<u16>) {
    let port = match port {
        Some(p) => p,
        None => 8080
    };

    let config = Config::build(Environment::Production)
        .port(port)
        .finalize().unwrap();

    let rocket_ws = rocket::custom(config);

    rocket_ws
        .mount(
            "/",
            routes![check, checksum],
        )
        .launch();
}
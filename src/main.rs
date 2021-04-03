#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::{Json, JsonValue};
use seshat::unicode::CodePoint;

mod properties_api;
use crate::properties_api::properties_api;


#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/echo/<num>")]
fn echo(num: String) -> ApiResponse {
    if num == "1" {
        ApiResponse {
            json: json!({"error": "first error!"}),
            status: Status::NotFound,
        }
    } else {
        ApiResponse {
            json: json!({"ok": "done!"}),
            status: Status::Ok,
        }
    }
}

#[get("/api/unicode/properties/<cp>")]
fn properties(cp: String) -> ApiResponse {
    let cp = format!("{}", cp);
    let cp = u32::from_str_radix(&cp, 16);
    match cp {
        Ok(val) => {
            if val < 0 || val > 0x10FFFF {
                return ApiResponse {
                    json: json!({"message": "The Unicode number is out of range"}),
                    status: Status::NotFound,
                };
            }
            let codepoint = CodePoint::new(val).unwrap();
            ApiResponse {
                // json: json!({"message": "Ok"}),
                json: properties_api(codepoint),
                status: Status::Ok,
            }
        },
        Err(e) => {
            ApiResponse {
                json: json!({"message": format!("{}", e)}),
                status: Status::BadRequest,
            }
        },
    }
}

fn main() {
    rocket::ignite().mount("/", routes![hello, properties]).launch();
}
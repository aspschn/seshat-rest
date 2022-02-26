// #![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit="512"]

#[macro_use] extern crate rocket;
// #[macro_use] extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use rocket::http::{ContentType, Status, Header};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json;
use rocket::serde::json::{json};
use seshat::unicode::CodePoint;

mod properties_api;
use crate::properties_api::properties_api;
mod segmentation_api;
use crate::segmentation_api::segmentation_graphemes_api;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    }
}

#[derive(Debug)]
struct ApiResponse {
    json: json::Value,
    status: Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
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

#[get("/api/v2/unicode/properties/<cp>")]
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

#[get("/api/v2/unicode/segmentation/graphemes/<text>")]
fn segmentation_grapheme(text: String) -> ApiResponse {
    ApiResponse {
        json: segmentation_graphemes_api(text),
        status: Status::Ok,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, properties, segmentation_grapheme]).attach(Cors)
}

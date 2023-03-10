extern crate chrono;

use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use crate::holidays::compute_holidays;

pub mod holidays;

#[http_component]
fn cloud_start(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let year = get_year_from_query(&req);
    let holidays = compute_holidays(year);

    let body = format!("{:?}", holidays);

    Ok(http::Response::builder()
        .status(200)
        .body(Some(body.into()))?)
}

fn get_year_from_query(req: &Request) -> i32 {
    let mut year = 1970;

    let query_string = req.uri()
        .query()
        .unwrap_or_default();
    let query = querystring::querify(query_string);

    for (k, v) in query {
        if k == "year" {
            year = v.parse::<i32>().unwrap();
        }
    }

    year
}

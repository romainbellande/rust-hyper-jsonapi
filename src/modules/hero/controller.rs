use futures::future;
use hyper::{Method, StatusCode, Response, Body, Request};
use diesel::pg::PgConnection;

use diesel::prelude::*;
use crate::helpers::{BoxFuture};
use crate::schema::heroes;
use super::model::Hero;

pub fn find_all() {

}

fn send<T>(body: T) -> Response<T> {
    Response::builder()
        .header("Content-Type", "application/vnd.api+json")
        .body(body)
        .unwrap()
}

fn send_japi(body: String) -> Response<Body> {
    send(Body::from(body))
}

pub fn controller(request: Request<Body>, connection: &PgConnection) -> BoxFuture {
    let mut response = Response::new(Body::empty());

    match (request.method(), request.uri().path()) {
        (&Method::GET, "/heroes") => {
            let heroes_data: Vec<Hero> = heroes::table.order(heroes::id).load::<Hero>(connection).unwrap();
            let json = serde_json::to_string_pretty(&heroes_data).unwrap();
            response = send_japi(json)
        },
        (&Method::POST, "/heroes") => {
            response = send(request.into_body())
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Box::new(future::ok(response))
}

use hyper::{Method, StatusCode, Response, Body, Request, Chunk};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::thread;

use futures::future;
use futures::Stream;
use futures::Future;

use crate::helpers::{BoxFuture};
use super::schema::heroes;
use super::model::{Hero, HeroDto};
use japi::api::{DocumentDto};
use japi::model::JApiModel;

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
            let hero_doc = request.into_body().concat2().and_then(move |body: Chunk| {
                let json: DocumentDto<HeroDto> = serde_json::from_slice(&body).unwrap();
                let hero_dto = json.deserialize();
                let data = Hero::create(hero_dto, &connection).serialize();
                *response.body_mut() = Body::from(serde_json::to_string_pretty(&data).unwrap());
                Ok(response)
            });
            return Box::new(hero_doc);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Box::new(future::ok(response))
}

use hyper::{Response, Body, Request, Chunk};

use futures::future;
use futures::Stream;
use futures::Future;

use crate::helpers::{BoxFuture};
use crate::db;
use super::model::{Hero, HeroDto};
use japi::api::{DocumentDto};
use japi::model::JApiModel;
use japi::controller::Controller;

fn send<T>(body: T) -> Response<T> {
    Response::builder()
        .header("Content-Type", "application/vnd.api+json")
        .body(body)
        .unwrap()
}

fn send_japi(body: String) -> Response<Body> {
    send(Body::from(body))
}

fn find_all(request: Request<Body>) -> BoxFuture {
    let connection = db::establish_connection();
    let serialized_heroes: Vec<_> = Hero::read(&connection).iter().map(|hero| hero.serialize()).collect();
    let json = serde_json::to_string_pretty(&serialized_heroes).unwrap();
    let response = send_japi(json);
    Box::new(future::ok(response))
}

fn create(request: Request<Body>) -> BoxFuture {
    let mut response = Response::new(Body::empty());
    let hero_doc = request.into_body().concat2().and_then(move |body: Chunk| {
        let connection = db::establish_connection();
        let json: DocumentDto<HeroDto> = serde_json::from_slice(&body).unwrap();
        let hero_dto = json.deserialize();
        let data = Hero::create(hero_dto, &connection).serialize();
        *response.body_mut() = Body::from(serde_json::to_string_pretty(&data).unwrap());
        Ok(response)
    });
    return Box::new(hero_doc);
}

pub fn controller(request: Request<Body>) -> BoxFuture {
    Controller::new("/heroes", request)
        .get("", find_all)
        .post("", create)
        .exec()
}

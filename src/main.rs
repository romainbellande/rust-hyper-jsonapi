extern crate dotenv;
extern crate serde;
extern crate hyper;
extern crate futures;
extern crate japi;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate serde_json;

pub mod db;
pub mod helpers;
// pub mod schema;

pub mod modules {
    pub mod hero {
        pub mod model;
        pub mod controller;
        pub mod schema;
    }
}

use hyper::{Server, Body, Request};
use hyper::service::service_fn;
use futures::Future;

use self::helpers::{BoxFuture};

pub fn manager(req: Request<Body>) -> BoxFuture {
    modules::hero::controller::controller(req)
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(self::manager));

    println!("Listening on http://{}", addr);

    hyper::rt::run(server.map_err(|e| {
        eprintln!("server error: {}", e);
    }));
}

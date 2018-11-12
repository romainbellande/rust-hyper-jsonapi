use hyper;
use hyper::{Method, Response, Body, Request};
use hyper::rt::{Future};
use std::mem;
use url::Url;

use futures::future;

pub type BoxFuture = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

pub struct Controller<'a> {
  path: String,
  request: Request<Body>,
  endpoints: Vec<(&'a Method, String, fn(request: Request<Body>) -> BoxFuture)>,
}

impl<'a> Controller<'a> {
    pub fn new(path: &str, request: Request<Body>) -> Self {
        Controller {
            path: path.to_string(),
            request,
            endpoints: Vec::new(),
        }
    }

    pub fn get(&mut self, path: &str, callback: fn(request: Request<Body>) -> BoxFuture) -> &mut Self {
        self.register_endpoint(&Method::GET, path, callback);
        self
    }

    pub fn post(&mut self, path: &str, callback: fn(request: Request<Body>) -> BoxFuture) -> &mut Self {
        self.register_endpoint(&Method::POST, path, callback);
        self
    }

    pub fn exec(&mut self) -> BoxFuture {
        let response = Response::new(Body::empty());

        let endpoint_index = &self.endpoints.iter()
            .position(|(method, path, _callback)| {
                method == &self.request.method() && path == &self.request.uri().path()
            });

        if endpoint_index.is_some() {
            let index = endpoint_index.unwrap();
            let endpoints = &self.endpoints[index];
            let request_copy = mem::replace(&mut self.request, Request::default());

            return endpoints.2(request_copy);
        }

        Box::new(future::ok(response))
    }

    fn register_endpoint(&mut self, method: &'a Method, path: &str, callback: fn(request: Request<Body>) -> BoxFuture) {
        let url = self.create_url(path.to_string());
        self.endpoints.push((method, url, callback));
    }

    fn create_url(&self, url: String) -> String {
        let mut endpoint_path = String::new();
        endpoint_path.push_str(&self.path);
        if url != "/" {
            endpoint_path.push_str(&url);
        }
        endpoint_path
    }
}

use hyper;
use hyper::{Method, Response, Body, Request};
use hyper::rt::{Future};

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

    pub fn exec(&mut self) -> BoxFuture{
        let response = Response::new(Body::empty());

        let endpointIndex = self.endpoints.iter()
            .position(|(method, path, _callback)| {
                method == self.request.method() && path == self.request.uri().path()
            });

        if endpointIndex.is_some() {
            let index = endpointIndex.unwrap();
            let endpoint = &self.endpoints[index];
            return endpoint.2(self.request);
        }

        Box::new(future::ok(response))
    }

    fn register_endpoint(&mut self, method: &'a Method, path: &str, callback: fn(request: Request<Body>) -> BoxFuture) {
        let mut endpoint_path = String::new();
        endpoint_path.push_str(&self.path);
        endpoint_path.push_str(&path.to_string());
        self.endpoints.push((method, endpoint_path, callback));
    }
}

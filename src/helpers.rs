use hyper::rt::{Future};
use hyper::{Request, Response, Body};

pub type BoxFuture = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;
pub type ControllerWrapper = fn(req: Request<Body>) -> BoxFuture;

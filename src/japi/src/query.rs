use hyper::{Response, Body};

pub fn build_reponse<T>(body: T) -> Response<T> {
    Response::builder()
        .header("Content-Type", "application/vnd.api+json")
        .body(body)
        .unwrap()
}

pub fn build_japi_reponse(body: String) -> Response<Body> {
    build_reponse(Body::from(body))
}

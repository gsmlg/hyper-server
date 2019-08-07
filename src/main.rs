extern crate futures;
extern crate hyper;

use futures::future::FutureResult;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Get, StatusCode};
use std::{thread, time};

fn heavy_work() -> String {
    let duration = time::Duration::from_millis(200);
    thread::sleep(duration);
    "done".to_string()
}

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    // This method handles actually processing requests
    // We only handle GET requests on /data and ignore everything else
    // returning a HTTP 404
    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/data") => {
                let b = heavy_work().into_bytes();
                Response::new()
                    .with_header(ContentLength(b.len() as u64))
                    .with_body(b)
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}

fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}

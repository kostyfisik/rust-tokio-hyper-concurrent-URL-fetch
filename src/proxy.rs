#![deny(warnings)]

use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

static mut CLIENT: Client<HttpConnector> = None;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let URI = "http://httpbin.org/ip".parse::<hyper::Uri>().unwrap();
    let _resp = CLIENT.get(URI).await;
    // println!("Response: {:?}", _resp);
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    CLIENT = Client::new();

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = Client::new();
    let uri = "http://localhost:3000".parse::<hyper::Uri>().unwrap();
    let _resp = client.get(uri).await;
    // println!("Response: {:?}", _resp);
    Ok(Response::new(_resp.unwrap().into_body()))
    // Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

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

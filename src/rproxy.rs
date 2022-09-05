use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

fn debug_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri().path().starts_with("/request") {
        let client = Client::new();
        let uri = "http://localhost:3000".parse::<hyper::Uri>().unwrap();
        let _resp = client.get(uri).await;
        Ok(Response::new(_resp.unwrap().into_body()))
    } else {
        debug_request(req)
    }
}

#[tokio::main]
async fn main() {
    let bind_addr = "127.0.0.1:8000";
    let addr: SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");

    let make_svc = make_service_fn(|conn: &AddrStream| async move {
        Ok::<_, Infallible>(service_fn(move |req| handle(req)))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Running server on {:?}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

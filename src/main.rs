use futures::prelude::*;
use hyper::{body, client::Client};
use hyper_tls::HttpsConnector;
use std::{
    io::{self, Write},
    iter,
};
use tokio;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = 100)]
    concurrent_n: usize,
    #[clap(short, long, value_parser, default_value_t = 10)]
    flows_n: usize,
    #[clap(short, long, value_parser, default_value_t = 10)]
    nodes_in_flow_n: usize,
}

// const N_CONCURRENT: usize = 80;
// const N_CONCURRENT: usize = 50;

#[tokio::main(worker_threads = 1)]
// #[tokio::main]
async fn main() {
    let args = Args::parse();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // let client = Client::new();

    // let uri = "https://jsonplaceholder.typicode.com/photos/12"
    //     .parse()
    //     .unwrap();
    let uris = create_flow_list("https://jsonplaceholder.typicode.com/photos/12");

    stream::iter(uris)
        .map(move |uri| client.get(uri))
        .buffer_unordered(args.concurrent_n)
        .then(|res| async {
            let res = res.expect("Error making request: {}");
            println!("Response: {}", res.status());

            body::to_bytes(res).await.expect("Error reading body")
        })
        .for_each(|body| async move {
            io::stdout().write_all(&body).expect("Error writing body");
        })
        .await;
}

fn create_flow_list(uri_str: &str) -> std::iter::Take<std::iter::Repeat<hyper::Uri>> {
    let uri = uri_str.parse::<hyper::Uri>().unwrap();
    iter::repeat(uri).take(2)
}

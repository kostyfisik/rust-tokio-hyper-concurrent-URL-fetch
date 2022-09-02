use futures::prelude::*;
use hyper::{body, client::Client};
use hyper_tls::HttpsConnector;
use std::io::{self, Write};
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
    #[clap(short, long, value_parser, default_value_t = false)]
    silent: bool,
}

// const N_CONCURRENT: usize = 80;
// const N_CONCURRENT: usize = 50;

#[tokio::main(worker_threads = 1)]
// #[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("use -s to keep silent");
    println!("Silent mode: {}", args.silent);
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // let client = Client::new();

    // let uri = "https://jsonplaceholder.typicode.com/photos/12"
    //     .parse()
    //     .unwrap();
    let uris = create_flow_list(args.flows_n).into_iter();

    // based on https://stackoverflow.com/a/49089303/4280547
    stream::iter(uris)
        .map(move |uri| execute_flow(uri, &client))
        .buffer_unordered(args.concurrent_n)
        .then(|res| async {
            let res = res.expect("Error making request: {}");
            if !args.silent {
                println!("Response: {}", res.status());
            }
            body::to_bytes(res).await.expect("Error reading body")
        })
        .for_each(|body| async move {
            if !args.silent {
                io::stdout().write_all(&body).expect("Error writing body");
            }
        })
        .await;
}

fn create_flow_list(flow_n: usize) -> Vec<hyper::Uri> {
    let mut vec = Vec::new();
    for n in 1..flow_n + 1 {
        let max_index = 5000;
        let index = n % max_index;
        let uri_str =
            "https://jsonplaceholder.typicode.com/photos/".to_string() + &index.to_string();
        let uri = uri_str.parse::<hyper::Uri>().unwrap();
        vec.push(uri.clone());
    }
    vec
}

fn execute_flow(
    uri: hyper::Uri,
    client: &hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
) -> hyper::client::ResponseFuture {
    client.get(uri)
}

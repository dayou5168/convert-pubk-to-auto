use base58::ToBase58;
use blake2::{Blake2b512, Digest};
use clap::Parser;
use hex;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use tokio::net::TcpListener;

pub fn key_to_address(prefix: u16, key: &str) -> Result<String, String> {
    let formatted_key = if key.starts_with("0x") {
        &key[2..key.len()]
    } else {
        key
    };

    let raw_key = hex::decode(formatted_key);
    match raw_key {
        Err(e) => Err(format!("Hex decoding error: {e:?}")),
        Ok(mut raw_key) => {
            if raw_key.len() != 32 {
                Err(format!(
                    "Public key has wrong length: {} != 32",
                    raw_key.len()
                ))
            } else {
                let mut hasher = Blake2b512::new();
                hasher.update(b"SS58PRE");
                let simple_prefix: u8 = (prefix & 0x3F) as _;
                let full_prefix = 0x4000 | ((prefix >> 8) & 0x3F) | ((prefix & 0xFF) << 6);
                let prefix_hi: u8 = (full_prefix >> 8) as _;
                let prefix_low: u8 = (full_prefix & 0xFF) as _;
                if prefix == simple_prefix as u16 {
                    hasher.update([simple_prefix]);
                } else {
                    hasher.update([prefix_hi]);
                    hasher.update([prefix_low]);
                }
                hasher.update(&raw_key);
                let checksum = hasher.finalize();

                let mut raw_address: Vec<u8> = Vec::with_capacity(64);
                if prefix == simple_prefix as u16 {
                    raw_address.push(simple_prefix);
                } else {
                    raw_address.push(prefix_hi);
                    raw_address.push(prefix_low);
                }
                raw_address.append(&mut raw_key);
                raw_address.extend_from_slice(&checksum[0..2]);

                Ok(raw_address[..].to_base58())
            }
        }
    }
}

#[derive(Parser)] // requires `derive` feature
#[command(name = "pubkey-to-auto")]
#[command(bin_name = "pubkey-to-auto")]
enum ConvertCommand {
    Convertserver(ServerArgs),
    Convertcmd(ConvertArgs),
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = "Covert a publickey to autonomys address")]
struct ConvertArgs {
    #[arg(long, short)]
    publickey: String,
}
#[derive(clap::Args, Debug)]
#[command(version, about, long_about = "Listen to a port to start a service")]
struct ServerArgs {
    #[arg(long, short)]
    listen: String,
}

async fn handle_convert(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    log::info!("Request uri: {}",req.uri().path());
    if req.uri().path().contains("/pub/") {
        let pb = req.uri().path().split("/").last().unwrap();
        if pb != "" {
            let auto_result = key_to_address(2254, pb);
            match auto_result {
                Ok(autoad) => {
                    return Ok(Response::new(Full::new(Bytes::from(autoad))));
                }
                Err(_) => {
                    return Ok(Response::new(Full::new(Bytes::from(
                        "Unavailbe pubkey received, Please check you public key",
                    ))));
                }
            }
        }
    } else {
        return Ok(Response::new(Full::new(Bytes::from(
            "request to a wrong url",
        ))));
    }
    Ok(Response::new(Full::new(Bytes::from(
        "public key to autonomys address tool",
    ))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().json().init();
    match ConvertCommand::parse() {
        ConvertCommand::Convertserver(serverarg) => {
            let listener = TcpListener::bind(serverarg.listen).await?;

            // We start a loop to continuously accept incoming connections
            loop {
                let (stream, _) = listener.accept().await?;
                log::info!("client ip: {}",stream.peer_addr().unwrap());
                // Use an adapter to access something implementing `tokio::io` traits as if they implement
                // `hyper::rt` IO traits.
                let io = TokioIo::new(stream);
                // Spawn a tokio task to serve multiple connections concurrently
                tokio::task::spawn(async move {
                 
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(io, service_fn(handle_convert))
                        .await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
        }
        ConvertCommand::Convertcmd(pk) => {
            let s58_addr = key_to_address(2254, &pk.publickey).unwrap();
            println!("{}", s58_addr);
        }
    }
    Ok(())
}

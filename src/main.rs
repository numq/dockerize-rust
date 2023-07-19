use std::convert::Infallible;
use std::env::var;
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::str::FromStr;

use hyper::{Body, Method, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

const SERVICE_HOSTNAME: &str = "SERVICE_HOSTNAME";
const SERVICE_PORT: &str = "SERVICE_PORT";

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.method() == Method::GET && req.uri().path().trim_end_matches("/") == "/status" {
        let response = Response::builder()
            .body(Body::empty())
            .unwrap();
        Ok(response)
    } else {
        let response = Response::builder()
            .status(404)
            .body(Body::empty())
            .unwrap();
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::from_path(Path::new("./.env"))?;

    let hostname = IpAddr::from_str(&var(SERVICE_HOSTNAME)?)?;
    let port = var(SERVICE_PORT)?.parse::<u16>()?;

    let addr = SocketAddr::new(hostname, port);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use hyper::{Client, Uri};

    use super::*;

    #[tokio::test]
    async fn test_server() {
        dotenv::from_path(Path("./.env")).unwrap();

        let client = Client::new();

        let hostname = var(SERVICE_HOSTNAME).map(|hostname| {
            if hostname == "0.0.0.0" {
                String::from("127.0.0.1")
            } else {
                hostname
            }
        }).unwrap();
        let port = var(SERVICE_PORT).unwrap();

        let address = format!("http://{}:{}/status", hostname, port);

        let uri = Uri::from_str(&address).unwrap();

        let response = client.get(uri).await.unwrap();

        assert_eq!(response.status(), 200);
    }
}
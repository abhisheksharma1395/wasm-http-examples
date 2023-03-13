use std::net::SocketAddr;
use std::fs::read;
use std::ffi::OsStr;
use std::path::Path;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let mut response = Response::new(Body::empty());
    let res_headers = response.headers_mut();
    let headers = req.headers();
    // HTTP headers
    headers.iter().for_each(|(name, value)| {
        res_headers.insert(name, value.clone());
    });

    // Handle each HTTP verb
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Hello world from Rust with wasm!");
        }
        (&Method::GET, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        (&Method::GET, "/noop") => {}
        (&Method::GET, "/index") => {
            *response.body_mut() = Body::from(
                read("/index.html")
                .expect("Should be able to read index.html")
            )
        }
        (&Method::POST, "/fib") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_str = String::from_utf8_lossy(&body_bytes);
            let n: u32 = match body_str.trim().parse::<u32>() {
                Ok(n) => n,
                Err(_) => return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::empty())
                    .unwrap()),
            };
    
            // Call the Fibonacci function and return the result as a JSON response
            let result = fibonacci(n);
            *response.body_mut() = Body::from(result.to_string());
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port https://localhost:{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(router)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
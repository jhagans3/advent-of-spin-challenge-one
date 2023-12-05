use http::{Method, StatusCode};
use spin_sdk::{http::IntoResponse, http_component, key_value::Store};

#[http_component]
fn handle_request(req: http::Request<Vec<u8>>) -> anyhow::Result<impl IntoResponse> {
    // Open the default key-value store
    let store = Store::open_default()?;

    let response = match *req.method() {
        Method::POST => {
            // Add the request (URI, body) tuple to the store
            store.set(req.uri().path(), req.body().as_slice())?;
            println!(
                "Storing value in the KV store with {:?} as the key",
                req.uri().path()
            );
            http::Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(None)?
        }
        Method::GET => {
            // Get the value associated with the request URI, or return a 404 if it's not present
            match store.get(req.uri().path())? {
                Some(value) => {
                    println!("Found value for the key {:?}", req.uri().path());
                    http::Response::builder()
                        .status(StatusCode::OK)
                        .header("content-type", "application/json")
                        .body(Some(value))?
                }
                None => {
                    println!("No value found for the key {:?}", req.uri().path());
                    http::Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("content-type", "application/json")
                        .body(None)?
                }
            }
        }
        Method::DELETE => {
            // Delete the value associated with the request URI, if present
            store.delete(req.uri().path())?;
            println!("Delete key {:?}", req.uri().path());
            http::Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(None)?
        }
        Method::HEAD => {
            if store.exists(req.uri().path())? {
                println!("{:?} key found", req.uri().path());
                http::Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(None)?
            } else {
                println!("{:?} key not found", req.uri().path());
                http::Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("content-type", "application/json")
                    .body(None)?
            }
        }
        _ => http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "application/json")
            .body(None)?,
    };
    Ok(response)
}
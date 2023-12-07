use http::{Method, StatusCode};
use spin_sdk::{http::IntoResponse, http_component, key_value::Store};

#[http_component]
fn handle_advent_of_spin_challenge_one(
    req: http::Request<Vec<u8>>,
) -> anyhow::Result<impl IntoResponse> {
    // Open the default key-value store
    let store = Store::open_default()?;

    let response = match *req.method() {
        Method::POST => {
            // Add the request (URI, body) tuple to the store
            store.set(req.uri().query().unwrap_or(""), req.body().as_slice())?;
            println!(
                "Storing value in the KV store with {:?} as the key",
                req.uri().query().unwrap_or("")
            );
            http::Response::builder()
                .status(StatusCode::CREATED)
                .body(None)?
        }
        Method::GET => {
            // Get the value associated with the request URI, or return a 404 if it's not present
            match store.get(req.uri().query().unwrap_or(""))? {
                Some(value) => {
                    println!(
                        "Found value for the key {:?}",
                        req.uri().query().unwrap_or("")
                    );
                    http::Response::builder()
                        .status(StatusCode::OK)
                        .header("content-type", "application/json")
                        .body(Some(value))?
                }
                None => {
                    println!(
                        "No value found for the key {:?}",
                        req.uri().query().unwrap_or("")
                    );
                    http::Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(None)?
                }
            }
        }
        Method::DELETE => {
            // Delete the value associated with the request URI, if present
            store.delete(req.uri().query().unwrap_or(""))?;
            println!("Delete key {:?}", req.uri().query().unwrap_or(""));
            http::Response::builder()
                .status(StatusCode::OK)
                .body(None)?
        }
        Method::HEAD => {
            if store.exists(req.uri().query().unwrap_or(""))? {
                println!("{:?} key found", req.uri().query().unwrap_or(""));
                http::Response::builder()
                    .status(StatusCode::OK)
                    .body(None)?
            } else {
                println!("{:?} key not found", req.uri().query().unwrap_or(""));
                http::Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(None)?
            }
        }
        _ => http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(None)?,
    };
    Ok(response)
}

use netlify_lambda_http::{handler, lambda::{self, Context}, IntoResponse, Request, RequestExt};
use serde_json::{json};
use http::{header::HeaderValue, HeaderMap};
use std::collections::HashMap;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn convert(headers: &HeaderMap<HeaderValue>) -> HashMap<String, Vec<String>> {
    let mut header_hashmap = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }
    header_hashmap
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(hello)).await?;
    Ok(())
}
/*
async fn hello(
    request: Request,
    _: Context
) -> Result<impl IntoResponse, Error> {
    Ok(format!(
        "hello {} from {}",
        request
            .query_string_parameters()
            .get("name")
            .unwrap_or_else(|| "stranger"),
        request.uri()
    ))
}
*/

async fn hello(
    request: Request,
    _: Context
) -> Result<impl IntoResponse, Error> {
    let b = request.headers();
    let uri = request.uri().path().split("/").collect::<Vec<&str>>();
    Ok(json!({ "request": convert(b), "uriSegments": uri }))
}
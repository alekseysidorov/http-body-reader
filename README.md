# http-body-reader

<!-- ANCHOR: description -->

A convenient way to read the http body. This library provides an easy way to
read the HTTP body in the most common formats such as UTF-8 encoded text, JSON,
form data.

# Usage

<!-- ANCHOR: example -->

```rust
use http::Request;
use http_body_reader::ResponseExt as _;
use reqwest::Client;
use tower_reqwest::HttpClientService;
use tower_service::Service as _;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new client
    let mut client = HttpClientService::new(Client::new());
    // Execute request by using this service.
    let response = client
        .call(Request::get("https://example.com").body(reqwest::Body::default())?)
        .await?;

    let text = response.body_reader().utf8().await?;
    println!("{text}");

    Ok(())
}
```

<!-- ANCHOR_END: example -->

<!-- ANCHOR_END: description -->

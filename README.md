# http-body-reader

[![tests](https://github.com/alekseysidorov/http-body-reader/actions/workflows/ci.yml/badge.svg)](https://github.com/alekseysidorov/http-body-reader/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/http-body-reader.svg)](https://crates.io/crates/http-body-reader)
[![Documentation](https://docs.rs/http-body-reader/badge.svg)](https://docs.rs/http-body-reader)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/http-body-reader)](./LICENSE)

<!-- ANCHOR: description -->

This library provides an easy way to read the HTTP body in the most common
formats such as UTF-8 encoded text, JSON, form data.

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

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `http-body-reader` by you, shall be licensed as MIT, without
any additional terms or conditions.

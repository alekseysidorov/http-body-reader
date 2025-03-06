use http::Response;
use http_body_reader::ResponseExt as _;
use http_body_util::Full;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new response with the given text as the body.
    let message = b"Hello world";
    let response = Response::builder().body(Full::new(message.as_ref()))?;
    // Read the response body as bytes and check that it matches the original message.
    assert_eq!(response.body_reader().bytes().await?, message.as_ref());

    Ok(())
}

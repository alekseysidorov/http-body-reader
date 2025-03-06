use http::Response;
use http_body_reader::ResponseExt as _;
use http_body_util::Full;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new response with the given text as the body.
    let message = "Hello world";
    let response = Response::builder().body(Full::new(message.as_ref()))?;
    // Read the response body as UTF-8 and check that it matches the original message.
    assert_eq!(response.body_reader().utf8().await?, message);

    Ok(())
}

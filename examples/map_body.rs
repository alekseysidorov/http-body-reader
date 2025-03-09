use http::Response;
use http_body_reader::ResponseExt as _;
use http_body_util::{Full, Limited};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a response with a body
    let response = Response::builder().body(Full::new("Lorem ipsum dolor sit amet".as_ref()))?;
    // Create a body reader with a limit of 8 bytes
    let body_reader = response.body_reader().map(|body| Limited::new(body, 8));
    // Make sure the body reader returns an error when the limit is exceeded.
    assert_eq!(
        body_reader.utf8().await.unwrap_err().to_string(),
        "length limit exceeded"
    );

    Ok(())
}

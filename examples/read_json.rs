use http::Response;
use http_body_reader::ResponseExt as _;
use http_body_util::Full;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SomeInfo {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let info = SomeInfo {
        id: 1234,
        name: "Alice".to_string(),
    };

    // Create a response with JSON body.
    let data = serde_json::to_vec(&info)?;
    let response = Response::builder().body(Full::new(data.as_ref()))?;
    // Read the response body as JSON and check that it matches the original info.
    assert_eq!(response.body_reader().json::<SomeInfo>().await?, info);

    Ok(())
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: serde_json::Value,
}

impl Response {
    pub fn new() -> Self {
        Response {
            ..Default::default()
        }
    }
}
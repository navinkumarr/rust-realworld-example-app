#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorWrapper {
    pub error: ErrorDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetails {
    pub status: i32,
    pub message: String,
    pub message_shortcode: String,
    pub datetime: String,
    pub url: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

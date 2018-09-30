use chrono::Local;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorWrapper {
    pub error: ErrorDetails,
}

impl ErrorWrapper {
    pub fn repo_error(m: String) -> ErrorWrapper{
        let date = Local::now();
        ErrorWrapper {
            error: ErrorDetails {
                status: 400,
                message: String::from(m),
                message_shortcode: String::from("repo_error"),
                datetime: date.format("%Y%m%d%H%M%S").to_string(),
                error_type: String::from("RepoError"),
            }
        }
    }

    pub fn invalid_input(m: String) -> ErrorWrapper{
        let date = Local::now();
        ErrorWrapper {
            error: ErrorDetails {
                status: 400,
                message: String::from(m),
                message_shortcode: String::from("invalid_input"),
                datetime: date.format("%Y%m%d%H%M%S").to_string(),
                error_type: String::from("IncompleteOrInvalidParameterException"),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetails {
    pub status: i32,
    pub message: String,
    pub message_shortcode: String,
    pub datetime: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

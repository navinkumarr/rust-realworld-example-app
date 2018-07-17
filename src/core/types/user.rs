#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username : String,
    pub email : String,
    pub token : String,
    pub bio : String,
    pub image : String,
}

impl User {
	pub fn new() -> User {
		User{
			username : String::from(""),
			email : String::from(""),
			token : String::from(""),
			bio : String::from(""),
			image : String::from(""),
		}
	}
}
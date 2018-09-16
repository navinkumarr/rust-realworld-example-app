use core::types::RepoError;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username : String,
    pub email : String,
    pub token : String,
    pub bio : String,
    pub image : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username : String,
    pub email : String,
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

pub trait UserRepo {
    fn find_user_by_email(&self, email: String) -> Result<Option<User>, RepoError>;
    fn save_new_user(&self, new_user: &NewUser) -> Result<usize, RepoError>;
}
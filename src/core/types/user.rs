use core::types::RepoError;

#[derive(Debug, Serialize, Deserialize)]
#[derive(Default)]
pub struct User {
    pub username : String,
    pub email : String,
    pub password : String,
    pub token : Option<String>,
    pub bio : Option<String>,
    pub image : Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username : String,
    pub email : String,
    pub password : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email : String,
    pub password : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    pub username : String,
}

pub trait UserRepo {
    fn find_user_by_email(&self, email: &String) -> Result<Option<User>, RepoError>;
    fn find_user_by_username(&self, username: &String) -> Result<Option<User>, RepoError>;
    fn save_new_user(&self, new_user: &NewUser) -> Result<usize, RepoError>;
    fn update_user(&self, username: &String, update_user: &UpdateUser) -> Result<usize, RepoError>;
    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>, RepoError>;
}
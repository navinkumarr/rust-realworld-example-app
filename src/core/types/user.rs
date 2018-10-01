use core::types::error::RepoError;

#[derive(Debug, Serialize, Deserialize)]
#[derive(Default)]
pub struct User {
    pub id : u32,
    pub username : String,
    pub email : String,
    pub password : String,
    pub token : Option<String>,
    pub bio : Option<String>,
    pub image : Option<String>,
}

// -- Current user

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    pub username : String,
    pub id : u32,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUserInput {
    pub user: CurrentUser,
}

#[derive(Debug, Serialize)]
pub struct CurrentUserOutput {
    pub user: User,
}

// -- Login user

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email : String,
    pub password : String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserInput {
    pub user: LoginUser
}

#[derive(Debug, Serialize)]
pub struct LoginUserOutput {
    pub user: User,
}

// -- Register user

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username : String,
    pub email : String,
    pub password : String,
}


#[derive(Debug, Deserialize)]
pub struct RegisterUserInput {
    pub user: NewUser
}

#[derive(Debug, Serialize)]
pub struct RegisterUserOutput {
    pub user: NewUser,
}

// -- Update user

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct UpdateUserInput {
    pub user: UpdateUser
}

#[derive(Debug, Serialize)]
pub struct UpdateUserOutput {
    pub user: UpdateUser,
}

pub trait UserRepo {
    fn find_user_by_email(&self, email: &String) -> Result<Option<User>, RepoError>;
    fn find_user_by_username(&self, username: &String) -> Result<Option<User>, RepoError>;
    fn save_new_user(&self, new_user: &NewUser) -> Result<usize, RepoError>;
    fn update_user(&self, username: &String, update_user: &UpdateUser) -> Result<usize, RepoError>;
    fn find_user_by_credentials(&self, credentials: &LoginUser) -> Result<Option<User>, RepoError>;
}
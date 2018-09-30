// @TODO: Rethink about errors


#[derive(Debug, Serialize, Fail)]
pub enum RepoError {
    #[fail(display = "repo error: {}", _0)]
    RepoError(String),
}

#[derive(Debug, Serialize, Fail)]
pub enum CurrentUserError {
    #[fail(display = "invalid input: {}", _0)]
    InvalidInput(String),
    #[fail(display = "repo error: {}", _0)]
    RepoError(String),
}

impl From<RepoError> for CurrentUserError {
    fn from(e: RepoError) -> CurrentUserError {
        CurrentUserError::RepoError(e.to_string())
    }
}

#[derive(Debug, Serialize, Fail)]
pub enum LoginUserError {
    #[fail(display = "invalid input: {}", _0)]
    InvalidInput(String),
    #[fail(display = "repo error: {}", _0)]
    RepoError(String),
    #[fail(display = "invalid credentials")]
    InvalidCredentials,
}

impl From<RepoError> for LoginUserError {
    fn from(e: RepoError) -> LoginUserError {
        LoginUserError::RepoError(e.to_string())
    }
}

#[derive(Debug, Serialize, Fail)]
pub enum RegisterUserError {
    #[fail(display = "invalid input: {}", _0)]
    InvalidInput(String),
    #[fail(display = "repo error: {}", _0)]
    RepoError(String),
}

impl From<RepoError> for RegisterUserError {
    fn from(e: RepoError) -> RegisterUserError {
        RegisterUserError::RepoError(e.to_string())
    }
}

#[derive(Debug, Serialize, Fail)]
pub enum UpdateUserError {
    #[fail(display = "invalid input: {}", _0)]
    InvalidInput(String),
    #[fail(display = "repo error: {}", _0)]
    RepoError(String),
}

impl From<RepoError> for UpdateUserError {
    fn from(e: RepoError) -> UpdateUserError {
        UpdateUserError::RepoError(e.to_string())
    }
}

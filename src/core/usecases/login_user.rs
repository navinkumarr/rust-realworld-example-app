use core::types::user::*;
use core::types::token::*;
use core::types::error::*;

pub fn login_user<U, T>(
    login_user_input: LoginUserInput,
    user_repo: &U,
    token_repo: &T,
) -> Result<LoginUserOutput, LoginUserError>
where 
    U: UserRepo,
    T: TokenRepo
{
    let user = user_repo.find_user_by_credentials(&login_user_input.user)?;
    let email_user = user_repo.find_user_by_email(&login_user_input.user.email)?;

    debug!("email user {:?}", email_user);

    if user.is_some() {
        let mut user = user.unwrap();
        let token = token_repo.create_login_token(&user)?;
        debug!("{:?}", token);
        user.token = Some(token);
        debug!("{:?}", user);
        Ok(LoginUserOutput { user })
    }else{
        Err(LoginUserError::InvalidCredentials)
    }
    
}

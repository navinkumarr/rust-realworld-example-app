use core::types::user::*;
use core::types::io::login_user::*;

pub fn login_user<U>(
    login_user_input: LoginUserInput,
    user_repo: &U
) -> Result<LoginUserOutput, LoginUserError>
where 
    U: UserRepo
{
    let login_user = LoginUser {
        email : login_user_input.email,
        password : login_user_input.password,
    };
    
    let user = user_repo.find_user_by_credentials(&login_user)?;

    println!("{:?}", user);

    if user.is_some() {
        Ok(LoginUserOutput { user: user.unwrap() })
    }else{
        Err(LoginUserError::InvalidCredentials)
    }
    
}

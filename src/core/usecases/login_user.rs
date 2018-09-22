use core::types::user::*;
use core::types::io::login_user::*;
use jwt::{Algorithm, encode, decode};
use chrono::Local;

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
        let mut user = user.unwrap();
        let p1 = json!({
            "iss" : "realworld",
            "sub" : user.username,
            "iat" : Local::now().timestamp(),
            "exp" : Local::now().timestamp()
        });

        let secret = "realworld".to_string();
        let header = json!({});
        let token = encode(header, &secret, &p1, Algorithm::HS256).unwrap();
        println!("{:?}", token);
        user.token = token;
        Ok(LoginUserOutput { user })
    }else{
        Err(LoginUserError::InvalidCredentials)
    }
    
}

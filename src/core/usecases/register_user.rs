use core::types::user::*;
use core::types::io::register_user::*;

pub fn register_user<U>(
    register_user_input: RegisterUserInput,
    user_repo: &U
) -> Result<RegisterUserOutput, RegisterUserError>
where 
    U: UserRepo
{
    let user = NewUser {
        username : register_user_input.username,
        email : register_user_input.email,
        password : register_user_input.password,
    };
    
    let data = user_repo.save_new_user(&user)?;

    println!("{:?}", data);
    
    Ok(RegisterUserOutput { user })
}

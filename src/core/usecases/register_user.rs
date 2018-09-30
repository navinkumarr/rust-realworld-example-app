use core::types::user::*;
use core::types::error::*;

pub fn register_user<U>(
    register_user_input: RegisterUserInput,
    user_repo: &U
) -> Result<RegisterUserOutput, RegisterUserError>
where 
    U: UserRepo
{
    let data = user_repo.save_new_user(&register_user_input.user)?;

    println!("{:?}", data);
    
    Ok(RegisterUserOutput { user: register_user_input.user })
}

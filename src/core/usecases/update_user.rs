use core::types::user::*;
use core::types::error::*;

pub fn update_user<U>(
    current_user: CurrentUser,
    update_user_input: UpdateUserInput,
    user_repo: &U
) -> Result<UpdateUserOutput, UpdateUserError>
where 
    U: UserRepo
{
    let data = user_repo.update_user(&current_user.username, &update_user_input.user)?;

    println!("{:?}", data);
    
    Ok(UpdateUserOutput { user: update_user_input.user })
}

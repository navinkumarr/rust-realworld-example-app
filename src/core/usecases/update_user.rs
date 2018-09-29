use core::types::user::*;
use core::types::io::update_user::*;

pub fn update_user<U>(
    current_user: CurrentUser,
    update_user_input: UpdateUserInput,
    user_repo: &U
) -> Result<UpdateUserOutput, UpdateUserError>
where 
    U: UserRepo
{
    let user = UpdateUser {
        username : update_user_input.username,
        email : update_user_input.email,
        password : update_user_input.password,
        bio : update_user_input.bio,
        image : update_user_input.image,
    };
    
    let data = user_repo.update_user(&current_user.username, &user)?;

    println!("{:?}", data);
    
    Ok(UpdateUserOutput { user })
}

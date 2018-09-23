use core::types::user::*;
use core::types::io::get_current_user::*;

pub fn get_current_user<U>(
    current_user: CurrentUser,
    user_repo: &U
) -> Result<CurrentUserOutput, CurrentUserError>
where 
    U: UserRepo
{
    let user = user_repo.find_user_by_username(&current_user.username)?;

    println!("fetched user {:?}", user);

    if user.is_some() {
        Ok(CurrentUserOutput { user: user.unwrap() })
    } else {
        Err(CurrentUserError::InvalidInput(String::from("Invalid input.")))
    }
    
}

use core::types::user::*;
use core::types::error::*;

pub fn get_current_user<U>(
    current_user_input: CurrentUserInput,
    user_repo: &U
) -> Result<CurrentUserOutput, CurrentUserError>
where 
    U: UserRepo
{
    let user = user_repo.find_user_by_username(&current_user_input.user.username)?;

    debug!("fetched user {:?}", user);

    if user.is_some() {
        Ok(CurrentUserOutput { user: user.unwrap() })
    } else {
        Err(CurrentUserError::InvalidInput(String::from("Invalid input.")))
    }
    
}

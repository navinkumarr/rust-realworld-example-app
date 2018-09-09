use core::types::user::*;
use core::types::io::get_current_user::*;

pub fn get_current_user(
    user: User,
) -> Result<CurrentUserOutput, CurrentUserError>
{
    let user = User {
        username : String::from(""),
        email : String::from(""),
        token : String::from(""),
        bio : String::from(""),
        image : String::from(""),
    };
    
    Ok(CurrentUserOutput { user })
}

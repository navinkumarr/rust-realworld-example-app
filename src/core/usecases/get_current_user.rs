use core::types::user::*;
use core::types::io::get_current_user::*;

pub fn get_current_user<U>(
    user: User,
    user_repo: &U
) -> Result<CurrentUserOutput, CurrentUserError>
where 
    U: UserRepo
{
    let user = User {
        username : String::from(""),
        email : String::from(""),
        token : String::from(""),
        bio : String::from(""),
        image : String::from(""),
    };
    
    let data = user_repo.find_user_by_email(String::from(""));

    println!("{:?}", data);
    
    Ok(CurrentUserOutput { user })
}


#[warn(dead_code)]
pub struct User {
    pub username: String,
    pub password: String
}
#[warn(dead_code)]
pub struct Book {
    name: String
}

impl User {

    fn setUser(user: &mut User, username: String, password: String){
        user.username = username;
        user.password = password;
    }
}

#[warn(dead_code)]
pub fn add_user(mut current_data: Vec<User>, new_data: User){
    current_data.push(new_data);
}




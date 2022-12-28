use bar::User;
use bar;
use std::{io};

fn main() {
    //Declaring a vector that contain User struct 
    let users = vec![User{username:"user1".to_string(), password:"user1".to_string()}, User{username:"user2".to_string(), password:"user2".to_string()}];
    println!("Hello, world!");
    println!("Welcome to the library managment system");
    println!("1. Login");
    login(users);
    
}

//Login function or we call authenticate function 
fn login(user_data: Vec<User>){
    //Declare A variable to store the input of the user 
    let mut input = String::new();
    //Declare the Input library in a variable
    let stdin = io::stdin();

    loop{
        println!("Username");
        //it will return a result from read_line whre we need to handle using match in case an error
        match stdin.read_line(&mut input){
            Ok(_) =>{
                
            },
            Err(error)=>{
                println!("Error {}", error);
            }
        }
        //Store the input data into a choice variable and use .to_owned by borrowing the data
        let username_input = input.to_owned();
        //Which then we slice it to make it into a &str type and trim
        let username: &str = &username_input[..].trim();
        println!("Password");
        input.clear();
        //it will return a result from read_line whre we need to handle using match in case an error
        match stdin.read_line(&mut input){
            Ok(_) => {
               
            },
            Err(error)=>{
                println!("Error {}", error);
            }
        }
        //Store the input data into a choice variable and use .to_owned by borrowing the data
        let password_input: String = input.to_owned();
        //Which then we slice it to make it into a &str type and trim
        let password: &str = &password_input[..].trim();
        let result = find_user(&user_data,username,password);

        if let Some(result) = result {
            println!("Sucessful");
            break;
        }else{
            println!("Invalid Login")
        }
    }
    

}

fn find_user<'a>(users: &'a Vec<User>, username: &str, password: &str) -> Option<&'a User> {
    for user in users {
        if user.username == username && user.password == password {
            return Some(user);
        }
    }
    None
}
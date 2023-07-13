#[warn(dead_code)]

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("chinedu"),
        email: String::from("ned1@gmail.com"),
        sign_in_count: 1,
    };

    user1.username = String::from("Progress");

    println!("{}", user1.username);
    //struct update syntax

    let user2 = User {
        email: String::from("mike@gmail.com"),
        ..user1
    };

    println!("{:?} {:?}", user2.email, user2.username);

    //Note: user1 can't be used after user2 has been used, this is because of the ownership rule, the Sting in the username field was moved into user2. if we have given user2 new string vals for username and email then this would have not been the case...

    //The types of active and sugn_in_counts are types that implement the copy trait.

    //struct tuple
    let color = Color(000, 000, 000);
    let point = Point(1, 1, 1);

    //unit like struct
    let subject = AlwaysEqual;
}

// A user defined struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//a build user function that returns a user instance with the given username and email
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

//Tuple struct

//using tuple structs without named fields to create different types

//struct that look similar to tuples are called tuple structs.

//they do not have names associated with thier fields

//they are useful when you want to give the whole tuple a name and make the tuple a diff type from other tuples.

//below is how we name a tuple struct


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


//Unit-like structs without any fields

//defining structs without any fields, are called unit-like struct, this is bcos they behave similarly to ()

struct AlwaysEqual;
struct User {
    first_name : String,
    last_name : String,
    age : i32,
    active : bool
}

fn main() {
    let mut admin = User {
        first_name: String::from("Faruk"),
        last_name: String::from("Durgut"),
        age: 28,
        active: true
    };

    admin.first_name = String::from("Omer Faruk");
    println!("{}", admin.first_name);


    let moderator = create_user(String::from("Faruk"),String::from("Durgut"));
    println!("{} {}", moderator.first_name, moderator.last_name);

    let copy_moderator = User {
        first_name: String::from("Copy Moderator"),
        ..moderator
    };
    println!("{} {}", copy_moderator.first_name, copy_moderator.last_name);

    /*
    let another_copy_moderator = User {
        first_name: moderator.first_name,
        last_name: moderator.last_name,
        age: 30,
        active:false
    };
    println!("{} {}", another_copy_moderator.first_name, another_copy_moderator.last_name);
    */
}

fn create_user (first_name: String, last_name: String) -> User {
    User {
        first_name,
        last_name,
        age: 28,
        active: false
    }
}
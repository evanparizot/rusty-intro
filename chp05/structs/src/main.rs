fn main() {
    println!("Hello, world!");

    //region 5.1
    let user1 = User {
        email: String::from(""),
        username: String::from(""),
        active: true,
        sign_in_count: 1
    };

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1
        }
    }

    fn build_user2(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1
        }
    }

    let user2 = User {
        email: String::from(""),
        username: String::from(""),
        ..user1
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    //endregion


}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
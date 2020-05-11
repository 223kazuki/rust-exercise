struct User {
    userName: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut user = User {
        email: String::from("eee@example.com"),
        userName: String::from("Kazuki Tsutsumi"),
        sign_in_count: 0,
        active: true,
    };

    user.userName = String::from("AAA");
    println!("Hello, world! {}", user.userName);

    let user2 = build_user(String::from("AAA"), String::from("BBB"));
    println!("{}", user2.userName);

    let user3 = User {
        userName: String::from("Test"),
        ..user2
    };
    println!("{}", user3.userName);

    let color = Color(1,1,1);
    println!("{}", color.0);

    let r = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:#?}", r);
    println!("{}", area(&r));
    println!("{}", r.area());
}

fn build_user(userName: String, email: String) -> User {
    User {
        email,
        userName,
        sign_in_count: 1,
        active: true,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

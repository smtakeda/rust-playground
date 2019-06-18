#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone1@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };

    println!("{}", user1.email);

    let mut user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("someusername456"),
        sign_in_count: 1,
        active: true,
    };
    user2.email = String::from("someone3@example.com");

    println!("{}", user2.username);

    let user3 = build_user(String::from("someone4@example.com"), String::from("helloworld"));
    println!("{}", user3.username);

    let user4 = User {
        email: String::from("someone5@example.com"),
        username: String::from("someone987"),
        ..user1
    };
    println!("{}", user4.username);

    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // example program 1 without tuple
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // example program 1 with tuple
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
    println!("{:?}", rect1); // NOTE: interestingly tuple WON'T MOVE the ownership.

    // example program 1 with strudt
    let rect2 = Rectangle { width: 30, height: 50 };
    //println!(
    //    "The area of the rectangle is {} square pixels.",
    //    area3(rect2)
    //);
    println!("{:?}", rect2);

    println!("{:?}", rect2.area());

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(32);
    println!("{:?}", rect4);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(dimensions: Rectangle) -> u32 {
    dimensions.height * dimensions.width
}

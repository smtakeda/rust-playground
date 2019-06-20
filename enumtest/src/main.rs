fn main() {
    // basic
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // with string
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    // variable types
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    // Message
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {}
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    // match statement with default one
    let Ou8 = 5;
    let some_u8_value = Ou8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if statement
    let some_u8_value = Some(Ou8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

fn main() {
    // assignment
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let x2 = 5;
    let x2 = x2 + 1;
    let x2 = x2 * 2;
    println!("The value of x2 is: {}", x2);

    // addition
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remember = 43 % 5;

    //
    let t = true;
    let t: bool = false;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February"];

    let _first = a[0];
    let second = a[1];

    let _index = 10;
    // let element = a[index]; // will raise a runtime error

    // functions
    println!("{}", another_function(6));

    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {}", y);

    // control flow
    let number = 3;
    if number < 5 {
        println!("condition is true!");
    } else {
        println!("condition is false!");
    }

    // if expression
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("value is {}", number);

    // while loop
    let mut counter = 3;
    while counter != 0 {
        println!("{}!", counter);
        counter = counter - 1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index = index + 1;
    }

    // for loop
    for element in a.iter() {
        println!("the value is {}", element);
    }

    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!");
}

fn another_function(x: i32) -> i32 {
    println!("Another function! {}", x);
    return x * 2;
}

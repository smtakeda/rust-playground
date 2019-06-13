fn main() {
    let s1 = String::from("Hello");
    let s2 = takes_ownership(s1);

    // println!("{}", s1);
    println!("{}", s2);

    let x = 5;
    makes_copy(x);

    let s3 = dangle();
    println!("{}", s3);

    let s4 = String::from("hello world");
    let siz = first_word(&s4);
    println!("{}", siz);

    let s41 = &s4[0..5];
    let s42 = &s4[6..];
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn dangle() -> String {
    let r = String::from("hello");
    return r;
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


pub fn string_test() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    s1.push(':');
    println!("s1 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 {}", s3);
    // println!("s1 {}", s1);  // s1 is already moved to s3.
    println!("s2 {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    print!("{}", s);
    print!("{}", s1);

    let s1 = String::from("hはllo");
    let h = &s1[0..1];
    // let h = &s1[0..2];
    print!("{}", h);

    for c in "hはlloろ".chars() {
        println!("{}", c);
    }
}
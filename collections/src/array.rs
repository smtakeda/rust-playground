pub fn array_test() {
    // let v: Vec(i32) = Vec::new();
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{:?}", third);

    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    // let does_not_exist = &v[100];
    let does_not_exist: Option<&i32> = v.get(100);
    println!("{:?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("{:?}", v);
    // println!("{:?}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }
    for i in &mut v2 {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("bluew")),
    ];
}

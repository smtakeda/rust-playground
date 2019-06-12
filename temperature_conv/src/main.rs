use std::io;

fn main() {
    println!("Convert a temperature from F to C or C to F ");

    println!("What is the input unit name? [F,C]");

    let mut input_unit = String::new();
    let mut trim_input_unit = "";
    loop {
        io::stdin().read_line(&mut input_unit).expect("Failed to read line");

        println!("You are typing: {}", input_unit);
        trim_input_unit = input_unit.trim();
        println!("You are typing: {}", trim_input_unit);
        if trim_input_unit == "F" {
            println!("Converting Fl to C...");
            break;
        } else if trim_input_unit == "C" {
            println!("Converting C to F...");
            break;
        }
        input_unit.clear();
    }
    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value).expect("Failed to read line");

    let parse_input_value = input_value.trim().parse::<f32>().unwrap();

    let output = if trim_input_unit == "F" {
        f_to_c(parse_input_value)
    } else {
        c_to_f(parse_input_value)
    };
    println!("Output: {}", output);
}

fn c_to_f(input: f32) -> f32 {
    return input * 9.0 / 5.0 + 32.0;
}

fn f_to_c(input: f32) -> f32 {
    return (input - 32.0) * 5.0 / 9.0;
}

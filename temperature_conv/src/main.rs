use std::io;

fn main() {
    println!("Convert a temperature from F to C or C to F ");

    let mut input_unit = String::new();
    let mut trim_input_unit: &str;
    loop {
        println!("What is the input unit name? [F,C]");
        io::stdin().read_line(&mut input_unit).expect("Failed to read line");

        // Converting to upper case needs multiple conversion between &str and String.
        trim_input_unit = input_unit.trim();
        input_unit = trim_input_unit.to_uppercase();
        trim_input_unit = input_unit.as_str();

        if trim_input_unit == "F" {
            println!("Converting Fl to C...");
            break;
        } else if trim_input_unit == "C" {
            println!("Converting C to F...");
            break;
        }
        println!("Invalid input!");
        // Need to clear the content of string, otherwise the original input value retains.
        input_unit.clear();
    }
    loop {
        println!("Type the input value");
        let mut input_value = String::new();
        io::stdin().read_line(&mut input_value).expect("Failed to read line");

        let parse_input_value = match input_value.trim().parse::<f32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input!");
                input_value.clear();
                continue;
            }
        };

        let output = if trim_input_unit == "F" {
            f_to_c(parse_input_value)
        } else {
            c_to_f(parse_input_value)
        };
        println!("Output: {}", output);
        break;
    }
}

fn c_to_f(input: f32) -> f32 {
    return input * 9.0 / 5.0 + 32.0;
}

fn f_to_c(input: f32) -> f32 {
    return (input - 32.0) * 5.0 / 9.0;
}

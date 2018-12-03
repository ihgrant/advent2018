use std::fs;
use std::num::ParseIntError;

fn get_file() -> Result<String, Box<std::error::Error + 'static>> {
    let input: String = fs::read_to_string("input.txt")?.parse()?;
    Ok(input)
}

fn split_sum_string(input: String) -> i32 {
    let split = input.split("\n");
    let mut sum: i32 = 0;

    for s in split {
        let i = string_to_int32(s);
        match i {
            Ok(x) => {
                sum = sum + x
            },
            Err(e) => println!("error: {}", e)
        }
    }

    return sum;
}

fn string_to_int32(number_string: &str)  -> Result<i32, ParseIntError> {
    let return_integer: i32 = number_string.replace("+", "").parse::<i32>().unwrap();
    Ok(return_integer)
}

fn main() {
    let file = get_file();
    match file {
        Ok(x) => {
            let sum = split_sum_string(x);
            println!("{}", sum)
        },
        Err(e) => println!("error: {}", e),
    }
}

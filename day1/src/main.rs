use std::fs;

fn get_file() -> Result<String, Box<std::error::Error + 'static>> {
    let input: String = fs::read_to_string("input.txt")?.parse()?;
    Ok(input)
}

fn split_string(input: String) {
    let split = input.split("\n");

    for s in split {
        println!("{}", s);
    }
}

fn main() {
    let file = get_file();
    match file {
        Ok(x) => split_string(x),
        Err(e) => println!("error: {}", e),
    }
}

use std::io;

//Unit function to take a Sting input and convert it into a float
pub fn get_float(input: &mut String) -> f64 {
    io::stdin()
        .read_line(input);

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("float not found");
            get_float(input)
        }
    };
    input
}

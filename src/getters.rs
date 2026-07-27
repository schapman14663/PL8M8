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

pub fn get_rounding_type(input: &mut String) -> f64 {
    io::stdin()
        .read_line(input);

    let input: f64 = match input.trim() {
        "Smart" => {smart_round(weight, increment)},
        "Down" => {round_up(weight, increment)},
        "Up" => {round_down(weight, increment)},
        _ => {
                eprintln!("invalid input");
                get_rounding_type(input)
        }
    }
}

pub fn get_unit_type(input: &mut String) -> f64 {
        io::stdin()
            .read_line(input);
        
        let input: String = match input.trim() { 
            "kg" => plate_sort(weight, increment, metric_weight_plates),
            "lbs" => plate_sort(weight, increment, imperial_weight_plates),
            _ => {
                    epinrtln!("You need to enter either kg or lbs!");
                    get_unit_type(input)
        }
    };
}


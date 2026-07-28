use std::{io, collections::HashMap};
use crate::weight_math_ops::{weight_division, rounding};

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

pub fn get_rounding_type(input: &mut String, weight: f64, increment: f64) -> f64 {
    io::stdin()
        .read_line(input);

    let input: f64 = match input.trim() {
        "Smart" => {rounding::smart_round(weight, increment)},
        "Down" => {rounding::round_up(weight, increment)},
        "Up" => {rounding::round_down(weight, increment)},
        _ => {
                eprintln!("invalid input");
                get_rounding_type(input, weight, increment)
        }
    };
    input
}

pub fn get_unit_type<K, V>(input: &mut String, weight: f64, increment: f64) -> HashMap<K, V> {
        io::stdin()
            .read_line(input);
        
        let input: String = match input.trim() { 
            "kg" => plate_sort(weight, increment, metric_weight_plates),
            "lbs" => plate_sort(weight, increment, imperial_weight_plates),
            _ => {
                    eprintln!("You need to enter either kg or lbs!");
                    get_unit_type(input, weight, increment)
        }
    };
}


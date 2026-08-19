use std::{io, path};
use crate::weight_math_ops::rounding;
use crate::logging;

//Function to take a String input and convert it into a float
pub fn get_float(input: &mut String) -> f64 {
    let log_dir = path::Path::new("/tmp/");
    logging::log_event(log_dir, "Function Call: 'get_float'\n");

    io::stdin()
        .read_line(input)
        .expect("error");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            let err_msg = format!("expected float, found something else");
            logging::log_event(log_dir, &err_msg);
            get_float(input)
        }
    };
    input
}

//Function to get a rounding type and then round the weight as indicated
//TODO: This loop does not work and becomes infinite if the wrong entry is provided. Change to a
//1,2,3 choice
pub fn get_rounded_weight(input: &mut String, weight: f64, increment: f64) -> f64 {
    let log_dir = path::Path::new("/tmp/");
    logging::log_event(log_dir, "Function Call: 'get_rounded_weight'\n");

    io::stdin()
        .read_line(input)
        .expect("error");
    
    let input: f64 = match input.trim().parse() {
        Ok(1) => {rounding::round_up(weight, increment)},
        Ok(2) => {rounding::round_down(weight, increment)},
        _ => {
                let err_msg = format!("invalid input, input received: {}, defaulting to smart rounding", input);
                logging::log_event(log_dir, &err_msg);
                rounding::smart_round(weight, increment)
        }   
    };
    input
}

//Function to generate a list of plates that are available to the user based on what weight unit
//the plates are in
pub fn get_available_plates() -> Vec<(f64, u32)> {
    let log_dir = path::Path::new("/tmp/");
    logging::log_event(log_dir, "Function Call: 'get_available_plates'\n");

    let mut units = String::new();
    let mut available_plates: Vec<(f64, u32)> = Vec::new();
    println!("Please select which of the following types of plates you are using:\n1.Metric (Kg)\n2.Imperial (Lbs)");

    io::stdin()
        .read_line(&mut units)
        .expect("error");
    
     let plates: Vec<f64> = match units.trim().parse() {
        Ok(1) => {
            let plates = vec![25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];
            plates
        }, 
        Ok(2) => {
            let plates = vec![55.0, 45.0, 35.0, 25.0, 10.0, 5.0, 2.5];
            plates
        },
        _ => {
            let err_msg = format!("Expected a value of 1 or 2, received something else");
            logging::log_event(log_dir, &err_msg);
            println!("received an incorrect input, defaulting to Kilogram plates");
            let plates = vec![25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];
            plates
        }
    };

    for plate in plates {
        let mut count = String::new();
        println!("How many {} weight plates do you have available to you?", plate);
        
        io::stdin()
            .read_line(&mut count)
            .expect("error");

        let count: u32 = match count.trim().parse() {
            //conversion to the lowest possible even number of plates
            Ok(num) => rounding::round_down(num, 2.0) as u32,
            Err(_) => 10,
        };
        
        available_plates.push((plate, count));
    };

    println!("Some amounts have been changed to the highest even number below the amount given.");

    for plate in &available_plates {
        print!("weight: {}, amount: {}\n", plate.0, plate.1);
    };

    available_plates
}

//Testing all of these requires mocking ?

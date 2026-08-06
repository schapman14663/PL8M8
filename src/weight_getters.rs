use std::io;
use crate::weight_math_ops::rounding;

//Function to take a String input and convert it into a float
pub fn get_float(input: &mut String) -> f64 {
    io::stdin()
        .read_line(input)
        .expect("error");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("float not found");
            get_float(input)
        }
    };
    input
}

//Function to get a rounding type and then round the weight as indicated
//TODO: This loop does not work and becomes infinite if the wrong entry is provided. Change to a
//1,2,3 choice
pub fn get_rounded_weight(input: &mut String, weight: f64, increment: f64) -> f64 {
    io::stdin()
        .read_line(input)
        .expect("error");
    
    let input: f64 = match input.trim() {
        "Smart" => {rounding::smart_round(weight, increment)},
        "Down" => {rounding::round_down(weight, increment)},
        "Up" => {rounding::round_up(weight, increment)},
        _ => {
                eprintln!("invalid input");
                get_rounded_weight(input, weight, increment)
        }
    };
    input
}

//Function to generate a list of plates that are available to the user based on what weight unit
//the plates are in
pub fn get_available_plates() -> Vec<(f64, u32)> {
    let mut units = String::new();
    let available_plates: Vec<(f64, u32)> = Vec::new();
    println!("Please select which of the following types of plates you are using:\n1.Metric (Kg)\n2.Imperial (Lbs)");

    io::stdin()
        .read_line(&mut units)
        .expect("error");
    
    let _units: Vec<(f64, u32)> = match units.trim().parse() {
        Ok(1) => generate_metric_plates(),
        Ok(2) => generate_imperial_plates(),
        _ => panic!(""),
    };
    available_plates
}

//Function to generate a list of metric plates available
fn generate_metric_plates() -> Vec<(f64, u32)> {
    let metric_plates = vec![25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];
    let mut available_plates: Vec<(f64, u32)> = Vec::new();

    for plate in metric_plates {
        let mut count = String::new();
        println!("How many { } kilogram plates do you have available to you?", plate);
        
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

//Function to generate a list of imperial plates available
fn generate_imperial_plates() -> Vec<(f64, u32)> {
    let imperial_plates = vec![55.0, 45.0, 35.0, 25.0, 10.0, 5.0, 2.5];
    let mut available_plates: Vec<(f64, u32)> = Vec::new();

    for plate in imperial_plates {
        let mut count = String::new();
        println!("How many { } pound plates do you have available to you?", plate);
        
        io::stdin()
            .read_line(&mut count)
            .expect("error");

        let count: u32 = match count.trim().parse() {
            //conversion to the lowest possible even number of plates
            Ok(num) => rounding::round_down(num, 2.0) as u32,
            Err(_) => 10, 
        };
    
        available_plates.push((plate, count))
    };
    println!("Some amounts have been changed to the highest even number below the amount given.");
    for plate in &available_plates {
        print!("weight: {}, amount: {}\n", plate.0, plate.1);
    };
   available_plates
}

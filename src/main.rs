use std::io;
use crate::math_ops::div:weight_division;

mod math_ops;
//TODO: Clean Up Notes

fn main() {
    println!("Hello, world!");

    let mut weight = String::new();
    let mut increment = String::new();
    let mut rounding = String::new();
    let mut units = String::new();

    let imperial_weight_plates: [f64; 7] = [55.0, 45.0, 25.0, 15.0, 10.0, 5.0, 2.5];
    let metric_weight_plates: [f64; 7] = [25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];

 
    println!("Please Enter the Weight you are meant to be doing this set:");
        io::stdin()
            .read_line(&mut weight)
            .expect("Failed to read line");

        let weight: f64 = match weight.trim().parse() {
            Ok(num) => num,
            Err(_) => get_weight(),
        };

    println!("Please Enter the smallest weight increment available\n(e.g. if you have 1.25kg plates available the smallest you could add to a barbell is 2.5kg):");
        io::stdin()
            .read_line(&mut increment)
            .expect("Failed to read line");

        let increment: f64 = match increment.trim().parse() {
            Ok(num) => num,
            Err(_) => get_increment(),
        };

    println!("Your Set Weight is {weight} and your increments are {increment}");
    
    println!("Would you like to round down, round up, or round to the actual nearest increment (smart)?");
        io::stdin()
            .read_line(&mut rounding)
            .expect("Failed to read line");
        
        let _rounding: f64 = match rounding.trim() {
            "Smart" => {smart_round(weight, increment)},
            "Down" => {round_up(weight, increment)},
            "Up" => {round_down(weight, increment)},
            _ => 0.0 
        };
    
    println!("What units are we working with Kilograms (kg) or Pounds (lbs)?");
        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");
        
        let units: String = match units.trim() { 
            "kg" => plate_sort(weight, increment, metric_weight_plates),
            "lbs" => plate_sort(weight, increment, imperial_weight_plates),
            _ => "You need to enter either kg or lbs!"
        };
}

//TODO: Draft Function that turns Inputs into a map(?) indicating how many of which plates go on
//each side. 
fn plate_sort(weight: f64, increment: f64, available_plates: ??) -> ?? {
    //available_plates will be the relevant metric/imperial plate set as indicated in the units
    //section.
    //
    //let mut remainder = weight much like weight_divsion() works.
    //
    //loop through available_plates while plates > 2 * increment:
    //  call division for weight and (plates * 2)
    //  append (plates, result) to return type 
    //  update remainder to (_, remainder) 
    
}

//TODO: Draft Function that can take an original weight and a new weight, and figure out the fewest
//plate changes needed to get to that weight. 

fn get_weight() -> f64 {
    let mut weight = String::new();
    println!("Number not found, please enter a number:");
    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");

    let weight: f64 = match weight.trim().parse() {
        Ok(num) => num, 
        Err(_) => get_weight(),
    };
    weight
}

fn get_increment() -> f64 {
    let mut increment = String::new();
    println!("Number not found, please enter a number:");
    //eprintln!("Type Found: {}", Err);
    io::stdin()
        .read_line(&mut increment)
        .expect("Failed to read line");

    let increment: f64 = match increment.trim().parse() {
        Ok(num) => num,
        Err(_) => get_increment(),
    };
    increment
}


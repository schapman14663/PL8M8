use std::io;
use crate::math_ops::{weight_division, rounding};
use crate::getters::{get_float, get_rounding_type, get_unit_type};

pub mod math_ops;
pub mod getters;

//TODO: Clean Up Notes

fn main() {
    println!("Hello, world!");

    let mut weight = String::new();
    let mut increment = String::new();
    let mut rounding_type = String::new();
    let mut unit_type = String::new();

    let imperial_weight_plates: [f64; 7] = [55.0, 45.0, 25.0, 15.0, 10.0, 5.0, 2.5];
    let metric_weight_plates: [f64; 7] = [25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];

    println!("Please Enter the Weight you are meant to be doing this set:");
    let weight = get_float(&mut weight);

    println!("Please Enter the smallest weight increment available\n(e.g. if you have 1.25kg plates available the smallest you could add to a barbell is 2.5kg):");
    let increment = get_float(&mut increment);

    println!("Your Set Weight is {weight} and your increments are {increment}");
    
    println!("Would you like to round down, round up, or round to the actual nearest increment (smart)?");
    let rounding_type = get_rounding_type(&mut rounding_type);

    println!("What units are we working with Kilograms (kg) or Pounds (lbs)?");
    let unit_type = get_unit_type(&mut unit_type);
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

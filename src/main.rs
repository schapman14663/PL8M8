use std::{io, collections::HashMap};
use crate::weight_math_ops::{weight_division, rounding};
use crate::weight_getters::{get_float, get_rounded_weight, get_available_plates}; 
//use crate::weight_structs;

pub mod weight_math_ops;
pub mod weight_getters;
//pub mod weight_structs;

//TODO: Clean Up Notes

fn main() {
    println!("Hello, world!");

    let mut weight = String::new();
    let mut increment = String::new();
    let mut rounding_type = String::new();

    println!("Please Enter the Weight you are meant to be doing this set:");
    let weight = get_float(&mut weight);

    println!("Please Enter the smallest weight increment available\n(e.g. if you have 1.25kg plates available the smallest you could add to a barbell is 2.5kg):");
    let increment = get_float(&mut increment);

    println!("Your Set Weight is {weight} and your increments are {increment}");
    
    println!("Would you like to round down, round up, or round to the actual nearest increment (smart)?");
    let rounded_weight = get_rounded_weight(&mut rounding_type, weight, increment);
    
    println!("Your rounded weight is {rounded_weight}");

    let available_plates = get_available_plates(); 
//  plate_sort(rounded_weight, increment, available_plates);
}

//TODO: Draft Function that turns Inputs into a map(?) indicating how many of which plates go on
//each side. 
fn plate_sort(weight: f64, increment: f64, available_plates: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    //available_plates will be the relevant metric/imperial plate set as indicated in the units
    //section.
    //
    //let mut remainder = weight much like weight_divsion() works.
    //
    //loop through available_plates while plates > 2 * increment:
    //  call division for weight and (plates * 2)
    //  append (plates, result) to return type 
    //  update remainder to (_, remainder) 
    let barbell_weights: Vec<(f64, f64)> = Vec::new(); 
    barbell_weights
}

//TODO: Draft Function that can take an original weight and a new weight, and figure out the fewest
//plate changes needed to get to that weight. 

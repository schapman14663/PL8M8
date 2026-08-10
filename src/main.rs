use crate::weight_math_ops::weight_division;
use crate::weight_getters::{get_float, get_rounded_weight, get_available_plates}; 

pub mod weight_math_ops;
pub mod weight_getters;

//TODO: Clean Up Notes

fn main() {
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
 
    plate_sort(0, rounded_weight, available_plates);
}

//TODO: Draft Function that turns Inputs into a Vector indicating how many of which plates go on
//each side. 


fn plate_sort(x: usize, weight: f64, available_plates: Vec<(f64, u32)>) -> Vec<(f64, u32)> {
    
    let mut barbell_weights: Vec<(f64, u32)> = Vec::new();
    let (plate_count, current_weight) = weight_division(weight, available_plates[x].0);
    barbell_weights.push((available_plates[x].0, plate_count as u32));
    
    let x = x + 1;
    
    if x != available_plates.len() {
        plate_sort(x, current_weight, available_plates);
    }
    barbell_weights
}

//TODO: Draft Function that can take an original weight and a new weight, and figure out the fewest
//plate changes needed to get to that weight. 

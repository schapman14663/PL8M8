use crate::weight_math_ops::weight_division;
use crate::weight_getters::{get_float, get_rounded_weight, get_available_plates}; 
//use crate::weight_structs;

pub mod weight_math_ops;
pub mod weight_getters;
//pub mod weight_structs;

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
 
    plate_sort(rounded_weight, available_plates);
}

//TODO: Draft Function that turns Inputs into a Vector indicating how many of which plates go on
//each side. 
fn plate_sort(weight: f64, available_plates: Vec<(f64, u32)>) -> Vec<(f64, u32)> {

    for p in &available_plates {
        print!("{} , {}\n", p.0, p.1);
    }

    let current_weight = weight;
    let plates_iter = available_plates.iter();
    let mut barbell_weights: Vec<(f64, u32)> = Vec::new(); 
    
    for plate in plates_iter {
        print!("{}", plate.0);
        let two_plate = plate.0 * 2.0;
        let (plate_pair_count, new_weight) = weight_division(current_weight, two_plate);

        print!("{} , {}\n", plate.0, plate_pair_count);
        barbell_weights.push((plate.0, plate_pair_count as u32));
        let current_weight = new_weight;
    }
    println!("Done");
    barbell_weights
}

//TODO: Draft Function that can take an original weight and a new weight, and figure out the fewest
//plate changes needed to get to that weight. 

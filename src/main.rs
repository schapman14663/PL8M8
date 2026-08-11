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
    
    println!("Please enter a number to select an option below take make sure your weight is multiple of the increment you entered:\n1.Round Up\n2.Round Down\n3.Smart (nearest multiple regardless of direction)");
    let rounded_weight = get_rounded_weight(&mut rounding_type, weight, increment);
    
    println!("Your rounded weight is {rounded_weight}");

    let available_plates = get_available_plates(); 
 
    plate_sort(0, rounded_weight, available_plates);
}

//TODO: Draft Function that turns Inputs into a Vector indicating how many of which plates go on
//each side. 


fn plate_sort(x: usize, weight: f64, available_plates: Vec<(f64, u32)>) -> Vec<(f64, u32)> {
    print!("Function Call: 'plate_sort'\n");
    let mut barbell_weights: Vec<(f64, u32)> = Vec::new();
    let plate_pair = 2.0 * available_plates[x].0;
    let available_plate_count = available_plates[x].1;

    let (plate_count, current_weight) = weight_division(weight, plate_pair);

    //TODO: Implement Logic to prevent ""using"" more plates than are available
    plate_availability_check(current_weight, plate_pair, plate_count, available_plate_count);

    barbell_weights.push((available_plates[x].0, plate_count as u32));
    
    let x = x + 1;
    
    if x != available_plates.len() && current_weight > 0.0 {
        plate_sort(x, current_weight, available_plates);
    }
    
    for plate in &barbell_weights {
        if plate.1 != 0 {
            print!("\nWeight: {}, Number of Plates on each side: {}\n", plate.0, plate.1);
        }
    }
    barbell_weights
}

fn plate_availability_check(current_weight: f64, plate_pair: f64, plate_count: f64, available_plate_count: u32) -> f64 {
    if plate_count as u32 > available_plate_count {
        let extra_weight = (plate_count as u32 - available_plate_count) * plate_pair as u32;
        let current_weight = current_weight + extra_weight as f64;
        return current_weight;
    }
    current_weight
}

//TODO: Draft Function that can take an original weight and a new weight, and figure out the fewest
//plate changes needed to get to that weight. 

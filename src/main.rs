use crate::core::{logging, logic};
use std::path;

mod core;

//todo: clean up notes

fn main() {
    let log_dir = path::Path::new("/tmp/");
    logging::init_log(log_dir);

    let mut weight = String::new();
    let mut increment = String::new();
    let rounding_type = logic::RoundingType::RoundUp; //Temp Allocation to stop cargo
    //complaining
    let weight_units = logic::WeightUnits::Metric; //Same as above for RoundingType

    println!("please enter the weight you are meant to be doing this set:");
    let weight = logic::get_float(&mut weight);

    println!(
        "please enter the smallest weight increment available\n(e.g. if you have 1.25kg plates available the smallest you could add to a barbell is 2.5kg):"
    );
    let increment = logic::get_float(&mut increment);

    println!("your set weight is {weight} and your increments are {increment}");

    let rounded_weight = logic::get_rounded_weight(rounding_type, weight, increment);

    println!("your rounded weight is {rounded_weight}");

    let available_plates = logic::get_available_plates(weight_units);

    logic::plate_sort(0, rounded_weight, available_plates, log_dir);
}

//TODO: Draft Function that can take an original weight and a new weight, and figure out the fewest
//plate changes needed to get to that weight.
/*fn change_weight(old_weight: f64, new_weight: f64, barbell_weights: Vec<f64, u32>) -> Vec<f64, u32> {

    let var = plate_sort(0, new_weight, available_plates, log_dir);

    if new_weight < old_weight:
        var
//
//  compare var to barbell_weights where if they are the same then do nothing, if they are
//  different add the difference.
//
}*/

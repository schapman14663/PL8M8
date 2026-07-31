use std::{io, collections::HashMap};
pub mut struct plate_set {
    weight: f64,
//    unit: unit_type,
    rounding: rounding_type,
    available_plates: HashMap<K, V>,
}

/*
pub enum unit_type {
    metric,
    imperial,
}
*/

pub enum rounding_type {
    up,
    down,
    smart,
}

/* method to generate the available plates here */
pub fn generate_metric_plate_set<K, V> () -> HashMap<K, V> {
    let input = String::new();
    let metric_plates = HashMap::new();
    let metric_weights: [f64, 7] = [1.25, 2.5, 5, 10, 15, 20, 25];

    for weight in metric_weights:
        print!("How many { } kilogram plates do you have available?", weight);
        io::stdin
            .read_line(input)

        let input: u32 = match input.trim().parse() {
            Ok(u32) => metric_plates.insert(weight, input),  
            Err(_) => metric_plates.insert(weight, 10),
        };
    //DEBUG: print HashMap (derive debug?)
    metric_plates
   // <This actually should probably be done in the main.rs file> update the struct with the HashMap
}

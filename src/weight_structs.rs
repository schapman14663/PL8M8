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
    let metric_plates = HashMap::new();
    let metric_weights: [f64, 7] = [1.25, 2.5, 5, 10, 15, 20, 25];

    /*
    for weight in metric_weights:
        print!("How many { } kilogram plates do you have available?", weight);
        io::stdin
            ...

        match number:
            Ok -> Append HashMap
            Err -> default to 10
    DEBUG: print HashMap (derive debug?)
    update the struct with the HashMap
    */    
}

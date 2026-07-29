pub mut struct plate_set {
    weight: f64,
    unit: unit_type,
    rounding: rounding_type,
    available_plates: HashMap<K, V>,
}

pub enum unit_type {
    metric,
    imperial,
}

pub enum rounding_type {
    up,
    down,
    smart,
}

/* method to generate the available plates here */

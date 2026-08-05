pub mod rounding {
    use crate::weight_division;
    // Round the input weight down to the nearest multiple of the increment
pub fn round_down(weight: f64, increment: f64) -> f64 {
        let (res, _) = weight_division(weight, increment);
        let rounded_weight = res * increment;
        println!("When rounding down the weight is: {:.2}", rounded_weight);
        rounded_weight
    }

    // Round the input weight up to the nearest multiple of the increment
    pub fn round_up(weight: f64, increment: f64) -> f64 {
        let (res, rem) = weight_division(weight, increment);
        if rem > 0.0 {
            let rounded_weight = (res + 1.0) * increment;
            println!("When rounding up the weight is: {:.2}", rounded_weight);
            rounded_weight
        } else {
            let rounded_weight = res * increment;
            println!("When rounding up the weight is: {:.2}", rounded_weight);
            rounded_weight
        }
    }

    // Round the input weight to the nearest multiple of the increment regardless of direction
    pub fn smart_round(weight: f64, increment: f64) -> f64 {
        let (_,rem) = weight_division(weight, increment);
        let remainder_ratio = rem/increment;
        if remainder_ratio > 0.5 {
            round_up(weight, increment)
        } else {
            round_down(weight, increment)
        }
    }
}

//Divide input weight by available increment weight. 
//Used for rounding to nearest increment weight.
//Also used to determine how many plates of each available pair to use.
pub fn weight_division(weight: f64, increment: f64) -> (f64, f64) {
    let mut result = 0.0;
    let mut remainder = weight;
    while remainder >= increment {
        remainder -= increment;
        println!("{:.2}", remainder); //TODO: remove when better at debugging
        result = result + 1.0;
        println!("{:.2}", result); //TODO: remove when better at debugging
    };
    println!("Division Result: {:.2} , Remainder: {:.2}", result, remainder);//TODO: remove when better at debugging
    (result, remainder)
} 


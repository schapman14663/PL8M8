use crate::math_ops::div:weight_division;

mod div;


// Round the input weight down to the nearest multiple of the increment
fn round_down(weight: f64, increment: f64) -> f64 {
    let (w, _) = weight_division(weight, increment);
    let rounded_weight = w * increment;
    println!("When rounding down the weight is: {:.2}", rounded_weight);
    rounded_weight
    //Like I obviously need to tidy up some of
    //these comments, but the result part of the weight_division function is always going to be 
    //the rounded down version of the divsion. So nothing fancy needs to be done with the
    //remainder.
}

// Round the input weight up to the nearest multiple of the increment
fn round_up(weight: f64, increment: f64) -> f64 {
    let (w, r) = weight_division(weight, increment);
    if r > 0.0 {
        let rounded_weight = (w + 1.0) * increment;
        println!("When rounding up the weight is: {:.2}", rounded_weight);
        rounded_weight
    } else {
        let rounded_weight = w * increment;
        println!("When rounding up the weight is: {:.2}", rounded_weight);
        rounded_weight
    }
}

// Round the input weight to the nearest multiple of the increment regardless of direction
fn smart_round(weight: f64, increment: f64) -> f64 {
    let (_,r) = weight_division(weight, increment);
    let remainder_ratio = r/increment;
    if remainder_ratio > 0.5 {
        round_up(weight, increment)
    } else {
        round_down(weight, increment)
    }
}

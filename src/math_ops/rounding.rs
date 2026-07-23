use crate::math_ops::div:weight_division;

mod div;
//rounding down is floor division
//
//ex floor(122.7/2.5) = 49
//
//you can get the number following the decimal (in the above example: 0.08) by doing the following:
//(X % Y)/ Y
//then to round down subtract this from the original division. 
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

//rounding up is ceiling division
//ex ceil(122.7/2.5) = 50
//as above, but to always round up, you can add (1 - result) to the original division.
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
    //Not sure if this is actually written properly but, I guess we'll find out.
}

//smart division in this case means to the nearest integer regardless of direction.
//the rules above apply, but for smart division we would run the rounding up check, then if (1 -
//result) is above 0.5 apply the round up rule, otherwise apply the round down down rule.
fn smart_round(weight: f64, increment: f64) -> f64 {
    let (_,r) = weight_division(weight, increment);
    let remainder_ratio = r/increment;
    if remainder_ratio > 0.5 {
        round_up(weight, increment)
    } else {
        round_down(weight, increment)
    }
}

//standard barbell is 20kg or 45lbs and additional weight would be added to each side.
//as such weight added to the bar is always halved. Ex 80kg added to the bar is 40kg each side. 
//similarly a 2.5kg progression would actually be 1.25kg on each side of the bar.


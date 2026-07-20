use std::io;

//TODO: Clean Up Notes

fn main() {
    println!("Hello, world!");

    let mut weight = String::new();
    let mut increment = String::new();
    let mut rounding = String::new();
    let mut units = String::new();

    let imperial_weight_plates: [f64; 7] = [55.0, 45.0, 25.0, 15.0, 10.0, 5.0, 2.5];
    let metric_weight_plates: [f64; 7] = [25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];

 
    println!("Please Enter the Weight you are meant to be doing this set:");
        io::stdin()
            .read_line(&mut weight)
            .expect("Failed to read line");

        let weight: f64 = match weight.trim().parse() {
            Ok(num) => num,
            Err(_) => get_weight(),
        };

    println!("Please Enter the smallest weight increment available\0 (e.g. if you have 1.25kg plates available the smallest you could add to a barbell is 2.5kg):");
        io::stdin()
            .read_line(&mut increment)
            .expect("Failed to read line");

        let increment: f64 = match increment.trim().parse() {
            Ok(num) => num,
            Err(_) => get_increment(),
        };

    println!("Your Set Weight is {weight} and your increments are {increment}");
    
    //TODO: Get RoundingType Input
    println!("Would you like to round down, round up, or round to the actual nearest increment (smart)?");
        io::stdin()
            .read_line(&mut rounding)
            .expect("Failed to read line");
        
        let rounding: f64 = match rounding.trim() {
            "Smart" => {smart_round(weight, increment)},
            "Down" => {round_up(weight, increment)},
            "Up" => {round_down(weight, increment)},
            _ => 0.0 
        };
    
    //TODO: Get Weight Units Input
   /* println!("What units are we working with Kilograms (kg) or Pounds (lbs)?");
        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");
        
        let units: Units = match units.trim() { 
            "kg" => units = Units::Metric,
            "lbs" => units = Units::Imperial,
            _ => "You need to enter either kg or lbs!"
        };
*/
}


enum RoundingType {
    Smart,
    AlwaysDown,
    AlwaysUp
}

enum Units {
    Metric,
    Imperial
}

//TODO: Draft Function that turns Inputs into a map(?) indicating how many of which plates go on
//each side. 

//TODO: Draft Function that can take an original weight and a new weight, and figure out the fewest
//plate changes needed to get to that weight. 

fn get_weight() -> f64 {
    let mut weight = String::new();
    println!("Number not found, please enter a number:");
    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");

    let weight: f64 = match weight.trim().parse() {
        Ok(num) => num, 
        Err(_) => get_weight(),
    };
    weight
}

fn get_increment() -> f64 {
    let mut increment = String::new();
    println!("Number not found, please enter a number:");
    //eprintln!("Type Found: {}", Err);
    io::stdin()
        .read_line(&mut increment)
        .expect("Failed to read line");

    let increment: f64 = match increment.trim().parse() {
        Ok(num) => num,
        Err(_) => get_increment(),
    };
    increment
}

//Yep, we are hardcoding a division. Specifically, a divsision such that we get a remainder, in
//much the same way that a modulo function would.
fn weight_division(weight: f64, increment: f64) -> (f64, f64) {
    let mut result = 0.0;
    let mut remainder = weight;
    while remainder >= increment {
        remainder -= increment;
        println!("{:.2}", remainder);
        result = result + 1.0;
        println!("{:.2}", result);
    };
    println!("Division Result: {:.2} , Remainder: {:.2}", result, remainder);
    (result, remainder)
}

//rounding down is floor division
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


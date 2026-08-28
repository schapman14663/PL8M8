use crate::core::logging;
use std::{io, path};

//Function to take a String input and convert it into a float
pub fn get_float(input: &mut String) -> f64 {
    let log_dir = path::Path::new("/tmp/");
    logging::log_event(log_dir, "Function Call: 'get_float'\n");

    io::stdin().read_line(input).expect("error");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            let err_msg = "expected float, found something else".to_string();
            logging::log_event(log_dir, &err_msg);
            get_float(input)
        }
    };
    input
}

//Function to get a rounding type and then round the weight as indicated
//currently defaults to smart_round, final implementation is not expected to be an ask for input
//set up
pub fn get_rounded_weight(input: &mut String, weight: f64, increment: f64) -> f64 {
    let log_dir = path::Path::new("/tmp/");
    logging::log_event(log_dir, "Function Call: 'get_rounded_weight'\n");

    io::stdin().read_line(input).expect("error");

    let input: f64 = match input.trim().parse() {
        Ok(1) => rounding::round_up(weight, increment),
        Ok(2) => rounding::round_down(weight, increment),
        _ => {
            let err_msg = format!(
                "invalid input, input received: {}, defaulting to smart rounding",
                input
            );
            logging::log_event(log_dir, &err_msg);
            rounding::smart_round(weight, increment)
        }
    };
    input
}

//Function to generate a list of plates that are available to the user based on what weight unit
//the plates are in
pub fn get_available_plates() -> Vec<(f64, u32)> {
    let log_dir = path::Path::new("/tmp/");
    logging::log_event(log_dir, "Function Call: 'get_available_plates'\n");

    let mut units = String::new();
    let mut available_plates: Vec<(f64, u32)> = Vec::new();
    println!(
        "Please select which of the following types of plates you are using:\n1.Metric (Kg)\n2.Imperial (Lbs)"
    );

    io::stdin().read_line(&mut units).expect("error");

    let plates: Vec<f64> = match units.trim().parse() {
        Ok(1) => {
            let plates = vec![25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];
            plates
        }
        Ok(2) => {
            let plates = vec![55.0, 45.0, 35.0, 25.0, 10.0, 5.0, 2.5];
            plates
        }
        _ => {
            let err_msg = "Expected a value of 1 or 2, received something else".to_string();
            logging::log_event(log_dir, &err_msg);
            println!("received an incorrect input, defaulting to Kilogram plates");
            let plates = vec![25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];
            plates
        }
    };

    for plate in plates {
        let mut count = String::new();
        println!(
            "How many {} weight plates do you have available to you?",
            plate
        );

        io::stdin().read_line(&mut count).expect("error");

        let count: u32 = match count.trim().parse() {
            //conversion to the lowest possible even number of plates
            Ok(num) => rounding::round_down(num, 2.0) as u32,
            Err(_) => 10,
        };

        available_plates.push((plate, count));
    }

    println!("Some amounts have been changed to the highest even number below the amount given.");

    for plate in &available_plates {
        println!("weight: {}, amount: {}", plate.0, plate.1);
    }

    available_plates
}

pub mod rounding {
    use super::weight_division;
    use crate::logging;
    use std::path;

    // Round the input weight down to the nearest multiple of the increment
    pub fn round_down(weight: f64, increment: f64) -> f64 {
        let log_dir = path::Path::new("/tmp/");
        logging::log_event(log_dir, "Function Call: 'round_down'\n");

        let (res, _) = weight_division(weight, increment);
        let rounded_weight = res * increment;

        //Send to log. Invokes format so that we can log the variables as well
        let log_msg = format!("When rounding down the weight is: {:.2}\n", rounded_weight);
        logging::log_event(log_dir, &log_msg);

        rounded_weight
    }

    // Round the input weight up to the nearest multiple of the increment
    pub fn round_up(weight: f64, increment: f64) -> f64 {
        let log_dir = path::Path::new("/tmp/");
        logging::log_event(log_dir, "Function Call: 'round_up'\n");

        let (res, rem) = weight_division(weight, increment);
        if rem > 0.0 {
            let rounded_weight = (res + 1.0) * increment;
            let log_msg = format!("When rounding up the weight is: {:.2}\n", rounded_weight);
            logging::log_event(log_dir, &log_msg);
            rounded_weight
        } else {
            let rounded_weight = res * increment;
            let log_msg = format!("When rounding up the weight is: {:.2}\n", rounded_weight);
            logging::log_event(log_dir, &log_msg);
            rounded_weight
        }
    }

    // Round the input weight to the nearest multiple of the increment regardless of direction
    pub fn smart_round(weight: f64, increment: f64) -> f64 {
        let log_dir = path::Path::new("/tmp/");
        logging::log_event(log_dir, "Function Call: 'smart_round'\n");

        let (_, rem) = weight_division(weight, increment);
        let remainder_ratio = rem / increment;
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
    let log_dir = path::Path::new("/tmp/");
    logging::log_event(log_dir, "Function Call: 'weight_division'\n");

    let mut result = 0.0;
    let mut remainder = weight;

    if increment == 0.0 {
        panic!("Error: Increment set to zero, aborting...");
        //TODO: Handle this more gracefully, new function in logging.rs to send error to log and
        //then copy the log somewhere less temporary with the datetime in the file name.
    }

    while remainder >= increment {
        remainder -= increment;
        result += 1.0;
        let log_msg = format!(
            "result so far: {:.2}, weight remaining: {:.2}",
            result, remainder
        );
        logging::log_event(log_dir, &log_msg);
    }
    let log_msg = format!(
        "result so far: {:.2}, weight remaining: {:.2}",
        result, remainder
    );
    logging::log_event(log_dir, &log_msg);
    (result, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::rounding::{round_down, round_up, smart_round};

    #[test]
    fn round_down_test() {
        let result = round_down(79.8, 2.0);
        assert_eq!(result, 78.0);
    }
    #[test]
    fn round_up_test() {
        let result = round_up(760.4, 4.0);
        assert_eq!(result, 764.0);
    }
    #[test]
    fn smart_round_down_test() {
        let result = smart_round(93.725, 2.5);
        assert_eq!(result, 92.5);
    }
    #[test]
    fn smart_round_up_test() {
        let result = smart_round(593.533, 8.3);
        assert_eq!(result, 597.6);
    }
    #[test]
    fn weight_div_test() {
        let result = weight_division(102.0, 2.5);
        assert_eq!(result, (40.0, 2.0));
    }
    #[test]
    #[should_panic]
    fn weight_div_panic_test() {
        weight_division(102.0, 0.0);
    }
}

//Function that turns Inputs into a Vector indicating how many of which plates go on each side.
pub fn plate_sort(
    x: usize,
    weight: f64,
    available_plates: Vec<(f64, u32)>,
    log_dir: &path::Path,
) -> Vec<(f64, u32)> {
    logging::log_event(log_dir, "Function Call: 'plate_sort'\n");
    let mut barbell_weights: Vec<(f64, u32)> = Vec::new();
    let plate_pair = 2.0 * available_plates[x].0;
    let available_plate_count = available_plates[x].1;

    let (plate_count, current_weight) = weight_division(weight, plate_pair);
    println!("current weight: {}", current_weight);

    if plate_count as u32 * 2 > available_plate_count {
        let plate_diff = plate_count as u32 * 2 - available_plate_count;
        let plate_count = available_plate_count / 2;

        let new_weight: f64 = current_weight + (plate_diff as f64 * plate_pair) / 2.0;
        println!("new weight: {}", new_weight);

        barbell_weights.push((available_plates[x].0, plate_count));

        let x = x + 1;

        if x != available_plates.len() && new_weight > 0.0 {
            plate_sort(x, new_weight, available_plates, log_dir);
        }

        for plate in &barbell_weights {
            if plate.1 != 0 {
                print!(
                    "\nWeight: {}, Number of Plates on each side: {}\n",
                    plate.0, plate.1
                );
            }
        }
        barbell_weights
    } else {
        barbell_weights.push((available_plates[x].0, plate_count as u32));

        let x = x + 1;

        if x != available_plates.len() && current_weight > 0.0 {
            plate_sort(x, current_weight, available_plates, log_dir);
        }

        for plate in &barbell_weights {
            if plate.1 != 0 {
                print!(
                    "\nWeight: {}, Number of Plates on each side: {}\n",
                    plate.0, plate.1
                );
            }
        }
        barbell_weights
    }
}

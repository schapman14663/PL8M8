use std::path;
use crate::logging; 

pub mod rounding {
    use crate::weight_division;
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
        result = result + 1.0;
        let log_msg = format!("result so far: {:.2}, weight remaining: {:.2}", result, remainder);
        logging::log_event(log_dir, &log_msg);
    };
    let log_msg = format!("result so far: {:.2}, weight remaining: {:.2}", result, remainder);
    logging::log_event(log_dir, &log_msg); 
    (result, remainder)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::weight_math_ops::rounding::{round_up, round_down, smart_round};

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
        assert_eq!(result, 92.);
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

//Divide input weight by available increment weight. 
//Used for rounding to nearest increment weight.
//Also used to determine how many plates of each available pair to use.
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

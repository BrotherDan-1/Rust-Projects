use std::io;

fn main() {
    // Prompt the user to enter the desired p_ratio
    println!("Enter the desired p_ratio:");

    // Read the user input as a string
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a floating-point number
    let p_ratio: f64 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing p_ratio. Please enter a valid number.");
            return;
        }
    };

    // Lambda function for the formula dB = 10^p_ratio
    let calculate_dB = |p_ratio: f64| -> f64 {
        10.0_f64.powf(p_ratio)
    };

    // Calculate dB using the lambda function
    let dB = calculate_dB(p_ratio);

    // Print the result
    println!("For p_ratio = {}, dB: {}", p_ratio, dB);
//--------------------------------
//--------------------------------
 // Prompt the user to enter the desired dB
    println!("Enter the desired dB:");

    // Read the user input as a string
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a floating-point number
    let dB: f64 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Error parsing dB. Please enter a valid number.");
            return;
        }
    };

    // Lambda function for the formula p_ratio = log10(dB)
    let calculate_p_ratio = |dB: f64| -> f64 {
        f64::log10(dB)
    };

    // Calculate p_ratio using the lambda function
    let p_ratio = calculate_p_ratio(dB);

    // Print the result
    println!("For dB = {}, p_ratio: {}", dB, p_ratio);
}

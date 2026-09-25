use std::io;

/*
The goal of this program is NOT the medical accuracy of the BMI calculation,
 but to demonstrate the use of Rust programming language.
*/

fn main() {
    println!("Enter your name:");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let your_name = name_input.trim();

    println!("Enter your weight in kg (example: 75.4):");
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).unwrap();
    let your_weight: f32 = weight_input.trim().parse().unwrap();

    println!("Enter your length in m (example: 1.8):");
    let mut length_input = String::new();
    io::stdin().read_line(&mut length_input).unwrap();
    let your_length: f32 = length_input.trim().parse().unwrap();

    let calculated_bmi = calculate_bmi(your_weight, your_length);
    let bmi_category = determine_bmi_category(calculated_bmi);

    println!("BMI of {:?}: {:.2}", your_name, calculated_bmi);
    println!("Category: {}", bmi_category);
}

fn calculate_bmi(your_weight: f32, your_length: f32) -> f32 {
    your_weight / (your_length * your_length)
}

fn determine_bmi_category(bmi: f32) -> &'static str {
    const UNDERWEIGHT_THRESHOLD: f32 = 18.5;
    const NORMAL_WEIGHT_THRESHOLD: f32 = 25.0;
    const OVERWEIGHT_THRESHOLD: f32 = 30.0;

    if bmi < UNDERWEIGHT_THRESHOLD {
        "Underweight"
    } else if bmi >= UNDERWEIGHT_THRESHOLD && bmi < NORMAL_WEIGHT_THRESHOLD {
        "Normal weight"
    } else if bmi >= NORMAL_WEIGHT_THRESHOLD && bmi < OVERWEIGHT_THRESHOLD {
        "Overweight"
    } else {
        "Obesity"
    }
}

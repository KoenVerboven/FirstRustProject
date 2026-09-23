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
    println!("BMI of {:?}: {:.2}", your_name, calculated_bmi);

    if calculated_bmi < 18.5 {
        println!("Underweight");
    } else if calculated_bmi >= 18.5 && calculated_bmi < 25.0 {
        println!("Normal weight");
    } else if calculated_bmi >= 25.0 && calculated_bmi < 30.0 {
        println!("Overweight");
    } else {
        println!("Obesity");
    }
}

fn calculate_bmi(your_weight: f32, your_length: f32) -> f32 {
    your_weight / (your_length * your_length)
}

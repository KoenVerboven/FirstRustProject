use std::io;

fn main() {
    let yourweight: f32 = 75.0;
    let yourlength: f32 = 1.80;

    println!("Enter your name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let yourname = input.trim();

    let calculatedbmi = calculate_bmi(yourweight, yourlength);
    println!("BMI of {:?}: {:.2}", yourname, calculatedbmi);
    if calculatedbmi < 18.5 {
        println!("Underweight");
    } else if calculatedbmi >= 18.5 && calculatedbmi < 25.0 {
        println!("Normal weight");
    } else if calculatedbmi >= 25.0 && calculatedbmi < 30.0 {
        println!("Overweight");
    } else {
        println!("Obesity");
    }
}

fn calculate_bmi(yourweight: f32, yourlength: f32) -> f32 {
    yourweight / (yourlength * yourlength)
}

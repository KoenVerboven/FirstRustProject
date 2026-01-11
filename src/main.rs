use std::io;

fn main() {
    println!("Enter your name:");
    let mut nameinput = String::new();
    io::stdin().read_line(&mut nameinput).unwrap();
    let yourname = nameinput.trim();

    println!("Enter your weight in kg (example: 75.4):");
    let mut weightinput = String::new();
    io::stdin().read_line(&mut weightinput).unwrap();
    let yourweight: f32 = weightinput.trim().parse().unwrap();

    println!("Enter your length in m (example: 1.8):");
    let mut lengthinput = String::new();
    io::stdin().read_line(&mut lengthinput).unwrap();
    let yourlength: f32 = lengthinput.trim().parse().unwrap();

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

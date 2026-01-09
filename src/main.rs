fn main() {
    let yourname = "Koen Verboven";
    let yourweight: f32 = 75.0;
    let yourlength: f32 = 1.80;
    //let yourage: u8 = 30;

    println!("{} 8 januari 2026", yourname);

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
